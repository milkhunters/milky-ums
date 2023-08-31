import asyncio
import logging
from typing import Callable

import aio_pika
from fastapi import FastAPI
from grpc import aio

import redis.asyncio as redis
from sqlalchemy import select, insert

from src.config import Config, Email as EmailConfig
from src.models.access import AccessTags
from src.protos.ums_control import ums_control_pb2_grpc
from src.services.ums_control import UMService

from src.utils import RedisClient, EmailSender
from src.db import create_psql_async_session
from src.models import tables


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
    app.state.redis = RedisClient(pool_0)
    app.state.redis_client_reauth = RedisClient(pool_1)


async def init_email(app: FastAPI, config: EmailConfig):
    app.state.rmq = await aio_pika.connect_robust(
        host=config.RabbitMQ.HOST,
        port=config.RabbitMQ.PORT,
        login=config.RabbitMQ.USERNAME,
        password=config.RabbitMQ.PASSWORD,
        virtualhost=config.RabbitMQ.VIRTUALHOST,
    )
    app.state.email_sender = EmailSender(app.state.rmq, config)


async def init_default_role(app: FastAPI):
    default_id = "00000000-0000-0000-0000-000000000000"
    async with app.state.db_session() as session:
        role = await session.execute(select(tables.Role).where(tables.Role.id == default_id))
        if not role.scalar():
            await session.execute(insert(tables.Role).values(id=default_id, title="default"))
            await session.execute(
                insert(tables.Access).values(id=default_id, title=AccessTags.CAN_GET_SELF.value)
            )
            await session.execute(insert(tables.RoleAccess).values(role_id=default_id, access_id=default_id))
            can_edit_id = "00000000-0000-0000-0000-000000000001"
            await session.execute(
                insert(tables.Access).values(id=can_edit_id, title=AccessTags.CAN_UPDATE_ROLE.value)
            )
            await session.execute(insert(tables.RoleAccess).values(role_id=default_id, access_id=can_edit_id))
            can_get_id = "00000000-0000-0000-0000-000000000002"
            await session.execute(
                insert(tables.Access).values(id=can_get_id, title=AccessTags.CAN_GET_ROLE.value)
            )
            await session.execute(insert(tables.RoleAccess).values(role_id=default_id, access_id=can_get_id))
            can_delete_id = "00000000-0000-0000-0000-000000000003"
            await session.execute(
                insert(tables.Access).values(id=can_delete_id, title=AccessTags.CAN_DELETE_ROLE.value)
            )
            await session.execute(insert(tables.RoleAccess).values(role_id=default_id, access_id=can_delete_id))
            can_create_id = "00000000-0000-0000-0000-000000000004"
            await session.execute(
                insert(tables.Access).values(id=can_create_id, title=AccessTags.CAN_CREATE_ROLE.value)
            )
            await session.execute(insert(tables.RoleAccess).values(role_id=default_id, access_id=can_create_id))
            await session.commit()


async def grpc_server(app_state):
    server = aio.server()
    ums_control_pb2_grpc.add_UserManagementServicer_to_server(UMService(app_state), server)
    listen_addr = '[::]:50051'
    server.add_insecure_port(listen_addr)
    logging.info("Starting server on %s", listen_addr)
    await server.start()
    await server.wait_for_termination()


def create_start_app_handler(app: FastAPI, config: Config) -> Callable:
    async def start_app() -> None:
        logging.debug("Выполнение FastAPI startup event handler.")
        await init_db(app, config)
        await init_redis_pool(app, config)
        await init_email(app, config.EMAIL)
        await init_default_role(app)
        asyncio.get_running_loop().create_task(grpc_server(app.state))
        logging.info("FastAPI Успешно запущен.")

    return start_app


def create_stop_app_handler(app: FastAPI) -> Callable:
    async def stop_app() -> None:
        logging.debug("Выполнение FastAPI shutdown event handler.")
        await app.state.redis.close()
        await app.state.rmq.close()

    return stop_app
