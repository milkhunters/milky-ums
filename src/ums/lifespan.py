import asyncio
import logging
from typing import Callable, AsyncGenerator

import aio_pika
import redis.asyncio as redis
from fastapi import FastAPI
from grpc import aio

from ums.config import Config, RedisConfig
from ums.db import create_psql_async_session
from ums.models.schemas.role import RoleID
from ums.protos.ums_control import ums_control_pb2_grpc
from ums.security.confirm_manager import ConfirmManager
from ums.services import SessionManager
from ums.repositories import RoleRepo, PermissionRepo
from ums.services.ums_control import UMService
from ums.utils.s3 import S3Storage
from ums.utils import RedisClient, EmailSender
from ums.roles import RoleLoader


async def init_db(app: FastAPI, config: Config):
    engine, session = create_psql_async_session(
        host=config.DB.POSTGRESQL.HOST,
        port=config.DB.POSTGRESQL.PORT,
        username=config.DB.POSTGRESQL.USERNAME,
        password=config.DB.POSTGRESQL.PASSWORD,
        database=config.DB.POSTGRESQL.DATABASE,
        echo=config.DEBUG,
    )
    getattr(app, "state").db_session = session


async def init_sessions(app: FastAPI, config: RedisConfig):
    client = await redis.from_url(
        f"redis://:{config.PASSWORD}@{config.HOST}:{config.PORT}/0",
        encoding="utf-8",
        decode_responses=True,
    )
    getattr(app, "state").session_manager = SessionManager(
        redis_client=RedisClient(client)
    )


async def init_confirm_manager(app: FastAPI, config: RedisConfig):
    client = await redis.from_url(
        f"redis://:{config.PASSWORD}@{config.HOST}:{config.PORT}/2",
        encoding="utf-8",
        decode_responses=True,
    )
    getattr(app, "state").confirm_manager = ConfirmManager(RedisClient(client))


async def init_redis_pool(app: FastAPI, config: Config):
    pool_1 = await redis.from_url(
        f"redis://:{config.DB.REDIS.PASSWORD}@{config.DB.REDIS.HOST}:{config.DB.REDIS.PORT}/1",
        encoding="utf-8",
        decode_responses=True,
    )
    getattr(app, "state").redis_reauth = RedisClient(pool_1)


async def init_email(app: FastAPI, config: Config):
    getattr(app, "state").rmq = await aio_pika.connect_robust(
        host=config.EMAIL.RabbitMQ.HOST,
        port=config.EMAIL.RabbitMQ.PORT,
        login=config.EMAIL.RabbitMQ.USERNAME,
        password=config.EMAIL.RabbitMQ.PASSWORD,
        virtualhost=config.EMAIL.RabbitMQ.VIRTUALHOST,
    )
    getattr(app, "state").email_sender = EmailSender(
        getattr(app, "state").rmq,
        config.EMAIL,
        config.BASE.TITLE
    )


async def init_s3_storage(app: FastAPI, config: Config):
    getattr(app, "state").file_storage = await S3Storage(
        bucket=config.DB.S3.BUCKET,
        external_host=config.DB.S3.PUBLIC_ENDPOINT_URL
    ).create_session(
        endpoint_url=config.DB.S3.ENDPOINT_URL,
        region_name=config.DB.S3.REGION,
        access_key_id=config.DB.S3.ACCESS_KEY_ID,
        secret_access_key=config.DB.S3.ACCESS_KEY,
    )


async def init_roles(db_session: Callable[[], AsyncGenerator], models_path: str) -> None:
    logging.debug("Инициализация ролей.")

    roles = RoleLoader(models_path).roles

    default_id = 0
    default_role_model = next((role for role in roles if role.id == default_id), None)

    if not default_role_model:
        raise FileNotFoundError(f"Файл роли по умолчанию с default_id:{default_id} не найден.")

    async with db_session() as session:
        role_repo = RoleRepo(session)
        permission_repo = PermissionRepo(session)

        for _ in roles:
            role = await role_repo.get(id=_.id, as_full=True)
            if not role:
                await role_repo.create(id=_.id, title=_.title)
                await session.commit()

                for permission_tag in _.permissions:
                    permission = await permission_repo.get(title=permission_tag)
                    if not permission:
                        permission = await permission_repo.create(title=permission_tag)
                        await session.commit()

                    await role_repo.add_link(role_id=RoleID(_.id), permission_id=permission.id)
                    await session.commit()


async def grpc_server(redis_reauth, host: str, port: int):
    server = aio.server()
    ums_control_pb2_grpc.add_UserManagementServicer_to_server(UMService(redis_reauth), server)
    listen_addr = f"{host}:{port}"
    server.add_insecure_port(listen_addr)
    logging.info(f"Starting gRPC server on {listen_addr}", )
    await server.start()
    await server.wait_for_termination()


class LifeSpan:

    def __init__(self, app: FastAPI, config: Config):
        self.app = app
        self.config = config

    async def startup_handler(self) -> None:
        logging.debug("Выполнение FastAPI startup event handler.")
        await init_db(self.app, self.config)
        await init_redis_pool(self.app, self.config)
        await init_sessions(self.app, self.config.DB.REDIS)
        await init_confirm_manager(self.app, self.config.DB.REDIS)
        await init_email(self.app, self.config)
        await init_roles(getattr(self.app, "state").db_session, "src/ums/roles/models/")
        await init_s3_storage(self.app, self.config)
        asyncio.get_running_loop().create_task(
            grpc_server(
                getattr(self.app, "state").redis_reauth,
                host=self.config.CONTROL.HOST,
                port=self.config.CONTROL.PORT
            )
        )
        logging.info("FastAPI Успешно запущен.")

    async def shutdown_handler(self) -> None:
        logging.debug("Выполнение FastAPI shutdown event handler.")
        await getattr(self.app, "state").redis_sessions.close()
        await getattr(self.app, "state").redis_reauth.close()
        await getattr(self.app, "state").redis_confirmations.close()
        await getattr(self.app, "state").rmq.close()
