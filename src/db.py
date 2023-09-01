import urllib.parse

from sqlalchemy.ext.asyncio import AsyncSession, create_async_engine, AsyncEngine, async_sessionmaker
from sqlalchemy.orm import declarative_base


def create_psql_async_session(
        username: str,
        password: str,
        host: str,
        port: int,
        database: str,
        echo: bool = False,
) -> tuple[AsyncEngine, async_sessionmaker[AsyncSession]]:
    engine = create_async_engine(
        "postgresql+asyncpg://{username}:{password}@{host}:{port}/{database}".format(
            username=urllib.parse.quote_plus(username),
            password=urllib.parse.quote_plus(password),
            host=host,
            port=port,
            database=database
        ),
        echo=echo,
        future=True
    )
    return engine, async_sessionmaker(engine, expire_on_commit=False)


Base = declarative_base()
