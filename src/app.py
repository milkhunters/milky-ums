import logging
import os

from fastapi import FastAPI
from fastapi.exceptions import RequestValidationError

import redis.asyncio as redis
import aio_pika

from src.config import load_consul_config, Email as EmailConfig
from src.db import create_psql_async_session
from src.exceptions import APIError, handle_api_error, handle_404_error, handle_pydantic_error
from src.middleware.jwt import JWTMiddlewareHTTP
from src.router import register_api_router
from src.utils import RedisClient, EmailSender, custom_openapi

log = logging.getLogger(__name__)
config = load_consul_config(os.getenv('CONSUL_ROOT'), host="192.168.3.41")

app = FastAPI(
    title=config.BASE.TITLE,
    debug=config.DEBUG,
    version=config.BASE.VERSION,
    description=config.BASE.DESCRIPTION,
    root_path="/api/v1" if not config.DEBUG else "",
    docs_url="/api/docs" if config.DEBUG else "/docs",
    redoc_url="/api/redoc" if config.DEBUG else "/redoc",
    swagger_ui_parameters={"syntaxHighlight.theme": "obsidian"},
    contact={
        "name": config.BASE.CONTACT.NAME,
        "url": config.BASE.CONTACT.URL,
        "email": config.BASE.CONTACT.EMAIL,
    }
)


async def init_db(app: FastAPI):
    engine, session = create_psql_async_session(
        host=config.DB.POSTGRESQL.HOST,
        port=config.DB.POSTGRESQL.PORT,
        username=config.DB.POSTGRESQL.USERNAME,
        password=config.DB.POSTGRESQL.PASSWORD,
        database=config.DB.POSTGRESQL.DATABASE,
        echo=config.DEBUG,
    )
    app.state.db_session = session

    # async with engine.begin() as conn:
    #     # await conn.run_sync(tables.Base.metadata.drop_all)
    #     await conn.run_sync(tables.Base.metadata.create_all)


async def init_redis_pool(app: FastAPI, db: int = 0):
    pool = await redis.from_url(
        f"redis://:{config.DB.REDIS.PASSWORD}@{config.DB.REDIS.HOST}:{config.DB.REDIS.PORT}/{db}",
        encoding="utf-8",
        decode_responses=True,
    )
    app.state.redis = RedisClient(pool)


async def init_email(app: FastAPI, config: EmailConfig):
    app.state.rmq = await aio_pika.connect_robust(
        host=config.RabbitMQ.HOST,
        port=config.RabbitMQ.PORT,
        login=config.RabbitMQ.USERNAME,
        password=config.RabbitMQ.PASSWORD,
        virtualhost=config.RabbitMQ.VIRTUALHOST,
    )
    app.state.email = EmailSender(app.state.rmq, config)


@app.on_event("startup")
async def on_startup():
    log.debug("Выполнение FastAPI startup event handler.")
    await init_db(app)
    await init_redis_pool(app)
    await init_email(app, config.EMAIL)
    log.info("FastAPI Успешно запущен.")


@app.on_event("shutdown")
async def on_shutdown():
    log.debug("Выполнение FastAPI shutdown event handler.")
    await app.state.redis.close()
    await app.state.rmq.close()


app.openapi = lambda: custom_openapi(app, logo_url="https://avatars.githubusercontent.com/u/107867909?s=200&v=4")
app.state.config = config

log.debug("Добавление маршрутов")
app.include_router(register_api_router(config.DEBUG))
log.debug("Регистрация обработчиков исключений.")
app.add_exception_handler(APIError, handle_api_error)
app.add_exception_handler(404, handle_404_error)
app.add_exception_handler(RequestValidationError, handle_pydantic_error)
log.debug("Регистрация middleware.")
app.add_middleware(JWTMiddlewareHTTP)
