import asyncio
import logging
from typing import Callable

import aio_pika
import redis.asyncio as redis
from fastapi import FastAPI
from grpc import aio

from src.config import Config, Email as EmailConfig
from src.db import create_psql_async_session
from src.protos.ums_control import ums_control_pb2_grpc
from src.services.repository import RoleRepo, PermissionRepo, RolePermissionRepo
from src.services.storage.s3 import S3Storage
from src.services.ums_control import UMService
from src.utils import RedisClient, EmailSender
from src.utils.role import load_roles


async def init_db(app: FastAPI, config: Config):
    engine, session = create_psql_async_session(
        host=config.DB.POSTGRESQL.HOST,
        port=config.DB.POSTGRESQL.PORT,
        username=config.DB.POSTGRESQL.USERNAME,
        password=config.DB.POSTGRESQL.PASSWORD,
        database=config.DB.POSTGRESQL.DATABASE,
        echo=config.DEBUG,
    )
    app.state.db_session = session


async def init_redis_pool(app: FastAPI, config: Config):
    pool_0 = await redis.from_url(
        f"redis://:{config.DB.REDIS.PASSWORD}@{config.DB.REDIS.HOST}:{config.DB.REDIS.PORT}/0",
        encoding="utf-8",
        decode_responses=True,
    )
    pool_1 = await redis.from_url(
        f"redis://:{config.DB.REDIS.PASSWORD}@{config.DB.REDIS.HOST}:{config.DB.REDIS.PORT}/1",
        encoding="utf-8",
        decode_responses=True,
    )
    pool_2 = await redis.from_url(
        f"redis://:{config.DB.REDIS.PASSWORD}@{config.DB.REDIS.HOST}:{config.DB.REDIS.PORT}/2",
        encoding="utf-8",
        decode_responses=True,
    )
    app.state.redis_sessions = RedisClient(pool_0)
    app.state.redis_reauth = RedisClient(pool_1)
    app.state.redis_confirmations = RedisClient(pool_2)


async def init_email(app: FastAPI, config: Config):
    app.state.rmq = await aio_pika.connect_robust(
        host=config.EMAIL.RabbitMQ.HOST,
        port=config.EMAIL.RabbitMQ.PORT,
        login=config.EMAIL.RabbitMQ.USERNAME,
        password=config.EMAIL.RabbitMQ.PASSWORD,
        virtualhost=config.EMAIL.RabbitMQ.VIRTUALHOST,
    )
    app.state.email_sender = EmailSender(app.state.rmq, config)


async def init_s3_storage(app: FastAPI, config: Config):
    app.state.file_storage = await S3Storage(
        bucket=config.DB.S3.BUCKET,
        external_host=config.DB.S3.PUBLIC_ENDPOINT_URL
    ).create_session(
        service_name=config.DB.S3.SERVICE_NAME,
        endpoint_url=config.DB.S3.ENDPOINT_URL,
        region_name=config.DB.S3.REGION,
        access_key_id=config.DB.S3.ACCESS_KEY_ID,
        secret_access_key=config.DB.S3.ACCESS_KEY,
    )


async def init_role(app: FastAPI):
    logging.debug("Инициализация ролей.")

    roles = load_roles("src/models/roles")

    default_id = "00000000-0000-0000-0000-000000000000"
    default_role_model = next((role for role in roles if role.id == default_id), None)

    if not default_role_model:
        raise FileNotFoundError(f"Роль по умолчанию с default_id:{default_id} не найдена.")

    async with app.state.db_session() as session:
        role_repo = RoleRepo(session)
        permission_repo = PermissionRepo(session)
        role_permission_repo = RolePermissionRepo(session)

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
                link = await role_permission_repo.get(role_id=_.id, permission_id=permission.id)
                if not link:
                    await role_permission_repo.create(role_id=_.id, permission_id=permission.id)
                    await session.commit()


async def grpc_server(app_state):
    server = aio.server()
    ums_control_pb2_grpc.add_UserManagementServicer_to_server(UMService(app_state), server)
    listen_addr = "0.0.0.0:50051"
    server.add_insecure_port(listen_addr)
    logging.info(f"Starting gRPC server on {listen_addr}", )
    await server.start()
    await server.wait_for_termination()


def create_start_app_handler(app: FastAPI, config: Config) -> Callable:
    async def start_app() -> None:
        logging.debug("Выполнение FastAPI startup event handler.")
        await init_db(app, config)
        await init_redis_pool(app, config)
        await init_email(app, config)
        await init_role(app)
        await init_s3_storage(app, config)
        asyncio.get_running_loop().create_task(grpc_server(app.state))
        logging.info("FastAPI Успешно запущен.")

    return start_app


def create_stop_app_handler(app: FastAPI) -> Callable:
    async def stop_app() -> None:
        logging.debug("Выполнение FastAPI shutdown event handler.")
        await app.state.redis.close()
        await app.state.rmq.close()

    return stop_app
