import os
from dataclasses import dataclass
from logging import getLogger

import yaml
import consul
from dotenv import load_dotenv

from ums import version


logger = getLogger(__name__)


class ConfigParseError(ValueError):
    pass


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
class ContactConfig:
    NAME: str = None
    URL: str = None
    EMAIL: str = None


@dataclass
class JWTConfig:
    ACCESS_EXP_SEC: int
    REFRESH_EXP_SEC: int
    PRIVATE_KEY: str
    PUBLIC_KEY: str


@dataclass
class RabbitMQ:
    HOST: str
    PORT: int
    USERNAME: str
    PASSWORD: str
    VIRTUALHOST: str
    EXCHANGE: str


@dataclass
class EmailConfig:
    RabbitMQ: RabbitMQ
    SENDER_ID: str


@dataclass
class BaseConfig:
    TITLE: str
    DESCRIPTION: str
    VERSION: str
    SERVICE_PATH_PREFIX: str
    CONTACT: ContactConfig


@dataclass
class ControlConfig:
    HOST: str
    PORT: int


@dataclass
class Config:
    DEBUG: bool
    JWT: JWTConfig
    BASE: BaseConfig
    DB: DbConfig
    CONTROL: ControlConfig
    EMAIL: EmailConfig


def to_bool(value) -> bool:
    return str(value).strip().lower() in ("yes", "true", "t", "1")


def get_str_env(key: str, optional: bool = False) -> str:
    val = os.getenv(key)
    if not val and not optional:
        logger.error("%s is not set", key)
        raise ConfigParseError(f"{key} is not set")
    return val


def load_config() -> Config:
    """
    Load config from consul

    """
    env_file = ".env"

    if os.path.exists(env_file):
        load_dotenv(env_file)
    else:
        logger.info("Loading env from os.environ")

    is_debug = to_bool(get_str_env('DEBUG'))
    root_name = get_str_env("CONSUL_ROOT")
    host = get_str_env("CONSUL_HOST")
    port = int(get_str_env("CONSUL_PORT"))
    grpc_host = get_str_env("GRPC_HOST")
    grpc_port = int(get_str_env("GRPC_PORT"))
    service_path_prefix = get_str_env('SERVICE_PATH_PREFIX', optional=True)

    raw_yaml_config = consul.Consul(host=host, port=port, scheme="http").kv.get(root_name)[1]['Value'].decode("utf-8")
    if not raw_yaml_config:
        raise ConfigParseError("Consul config is empty")
    config = yaml.safe_load(raw_yaml_config)

    return Config(
        DEBUG=is_debug,
        BASE=BaseConfig(
            TITLE=config["base"]["title"],
            DESCRIPTION=config["base"]["description"],
            CONTACT=ContactConfig(
                NAME=config['base']['contact']['name'],
                URL=config['base']['contact']['url'],
                EMAIL=config['base']['contact']['email']
            ),
            VERSION=version.__version__,
            SERVICE_PATH_PREFIX=service_path_prefix
        ),
        JWT=JWTConfig(
            ACCESS_EXP_SEC=config['jwt']['access_expire_seconds'],
            REFRESH_EXP_SEC=config['jwt']['refresh_expire_second'],
            PUBLIC_KEY=config['jwt']['public_key'],
            PRIVATE_KEY=config['jwt']['private_key']
        ),
        DB=DbConfig(
            POSTGRESQL=PostgresConfig(
                HOST=config['database']['postgresql']['host'],
                PORT=config['database']['postgresql']['port'],
                USERNAME=config['database']['postgresql']['username'],
                PASSWORD=config['database']['postgresql']['password'],
                DATABASE=config['database']['postgresql']['database']
            ),
            REDIS=RedisConfig(
                HOST=config['database']['redis']['host'],
                USERNAME=config['database']['redis']['username'],
                PASSWORD=config['database']['redis']['password'],
                PORT=config['database']['redis']['port']
            ),
            S3=S3Config(
                ENDPOINT_URL=config['database']['s3']['endpoint_url'],
                REGION=config['database']['s3']['region'],
                ACCESS_KEY_ID=config['database']['s3']['access_key_id'],
                ACCESS_KEY=config['database']['s3']['secret_access_key'],
                BUCKET=config['database']['s3']['bucket'],
                PUBLIC_ENDPOINT_URL=config['database']['s3']['public_endpoint_url']
            ),
        ),
        EMAIL=EmailConfig(
            RabbitMQ=RabbitMQ(
                HOST=config['email']['rabbitmq']['host'],
                PORT=config['email']['rabbitmq']['port'],
                USERNAME=config['email']['rabbitmq']['username'],
                PASSWORD=config['email']['rabbitmq']['password'],
                VIRTUALHOST=config['email']['rabbitmq']['virtual_host'],
                EXCHANGE=config['email']['rabbitmq']['exchange']
            ),
            SENDER_ID=config['email']['sender_id']
        ),
        CONTROL=ControlConfig(
            HOST=grpc_host,
            PORT=grpc_port
        )
    )
