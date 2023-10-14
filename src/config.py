import os
from dataclasses import dataclass
from src.version import __version__

import consul


@dataclass
class RedisConfig:
    HOST: str
    PASSWORD: str
    USERNAME: str
    PORT: int = 6379


@dataclass
class PostgresConfig:
    DATABASE: str
    USERNAME: str
    PASSWORD: str
    HOST: str
    PORT: int = 5432


@dataclass
class S3Config:
    BUCKET: str
    ENDPOINT_URL: str
    PUBLIC_ENDPOINT_URL: str
    REGION: str
    ACCESS_KEY_ID: str
    ACCESS_KEY: str


@dataclass
class DbConfig:
    POSTGRESQL: PostgresConfig
    REDIS: RedisConfig
    S3: S3Config


@dataclass
class Contact:
    NAME: str = None
    URL: str = None
    EMAIL: str = None


@dataclass
class JWT:
    ACCESS_EXPIRE_SECONDS: int
    REFRESH_EXPIRE_SECONDS: int
    ACCESS_SECRET_KEY: str
    REFRESH_SECRET_KEY: str


@dataclass
class RabbitMQ:
    HOST: str
    PORT: int
    USERNAME: str
    PASSWORD: str
    VIRTUALHOST: str
    EXCHANGE: str


@dataclass
class Email:
    RabbitMQ: RabbitMQ
    SENDER_ID: str


@dataclass
class Base:
    TITLE: str
    DESCRIPTION: str
    VERSION: str
    CONTACT: Contact


@dataclass
class Config:
    DEBUG: bool
    JWT: JWT
    BASE: Base
    DB: DbConfig
    EMAIL: Email


def to_bool(value) -> bool:
    return str(value).strip().lower() in ("yes", "true", "t", "1")


class KVManager:
    def __init__(self, kv, *, root_name: str):
        self.config = kv
        self.root_name = root_name

    def __call__(self, *args: str) -> int | str | None:
        """
        :param args: list of nodes
        """
        path = "/".join([self.root_name, *args])
        encode_value = self.config.get(path)[1]
        if encode_value and encode_value["Value"]:
            value: str = encode_value['Value'].decode("utf-8")
            if value.isdigit():
                return int(value)
            return value
        return None


def load_consul_config(
        root_name: str,
        *,
        host='127.0.0.1',
        port=8500,
        token=None,
        scheme='http',
        **kwargs
) -> Config:
    """
    Load config from consul

    """

    config = KVManager(
        consul.Consul(
            host=host,
            port=port,
            token=token,
            scheme=scheme,
            **kwargs
        ).kv,
        root_name=root_name
    )
    return Config(
        DEBUG=to_bool(os.getenv('DEBUG', 1)),
        BASE=Base(
            TITLE=config("BASE", "TITLE"),
            DESCRIPTION=config("BASE", "DESCRIPTION"),
            VERSION=__version__,
            CONTACT=Contact(
                NAME=config("BASE", "CONTACT", "NAME"),
                URL=config("BASE", "CONTACT", "URL"),
                EMAIL=config("BASE", "CONTACT", "EMAIL")
            ),
        ),
        JWT=JWT(
            ACCESS_EXPIRE_SECONDS=config("JWT", "ACCESS_EXPIRE_SECONDS"),
            REFRESH_EXPIRE_SECONDS=config("JWT", "REFRESH_EXPIRE_SECONDS"),
            ACCESS_SECRET_KEY=config("JWT", "ACCESS_SECRET_KEY"),
            REFRESH_SECRET_KEY=config("JWT", "REFRESH_SECRET_KEY")
        ),
        DB=DbConfig(
            POSTGRESQL=PostgresConfig(
                HOST=config("DATABASE", "POSTGRESQL", "HOST"),
                PORT=config("DATABASE", "POSTGRESQL", "PORT"),
                USERNAME=config("DATABASE", "POSTGRESQL", "USERNAME"),
                PASSWORD=config("DATABASE", "POSTGRESQL", "PASSWORD"),
                DATABASE=config("DATABASE", "POSTGRESQL", "DATABASE")
            ),
            REDIS=RedisConfig(
                HOST=config("DATABASE", "REDIS", "HOST"),
                USERNAME=config("DATABASE", "REDIS", "USERNAME"),
                PASSWORD=config("DATABASE", "REDIS", "PASSWORD"),
                PORT=config("DATABASE", "REDIS", "PORT")
            ),
            S3=S3Config(
                ENDPOINT_URL=config("DATABASE", "S3", "ENDPOINT_URL"),
                REGION=config("DATABASE", "S3", "REGION"),
                ACCESS_KEY_ID=config("DATABASE", "S3", "ACCESS_KEY_ID"),
                ACCESS_KEY=config("DATABASE", "S3", "ACCESS_KEY"),
                BUCKET=config("DATABASE", "S3", "BUCKET"),
                PUBLIC_ENDPOINT_URL=config("DATABASE", "S3", "PUBLIC_ENDPOINT_URL")
            ),
        ),
        EMAIL=Email(
            RabbitMQ=RabbitMQ(
                HOST=config("EMAIL", "RabbitMQ", "HOST"),
                PORT=config("EMAIL", "RabbitMQ", "PORT"),
                USERNAME=config("EMAIL", "RabbitMQ", "USERNAME"),
                PASSWORD=config("EMAIL", "RabbitMQ", "PASSWORD"),
                VIRTUALHOST=config("EMAIL", "RabbitMQ", "VIRTUALHOST"),
                EXCHANGE=config("EMAIL", "RabbitMQ", "EXCHANGE")
            ),
            SENDER_ID=config("EMAIL", "SENDER_ID")
        )
    )
