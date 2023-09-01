import time
import uuid

from fastapi import Request, Response

from src.config import Config
from src.utils import RedisClient


class SessionManager:
    COOKIE_PATH = "/api"
    COOKIE_DOMAIN = None
    COOKIE_SESSION_KEY = "session_id"

    def __init__(self, redis_client: RedisClient, config: Config, debug: bool = False):
        self._redis_client = redis_client
        self._config = config
        self._debug = debug

    def get_session_id(self, req_obj: Request) -> str | None:
        """
        Получить идентификатор сессии из куков

        :param req_obj:
        :return: session_id
        """

        return req_obj.cookies.get(self.COOKIE_SESSION_KEY)

    async def set_session_id(
            self,
            response: Response,
            user_id: uuid.UUID,
            refresh_token: str,
            ip_address: str,
            user_agent: str,
            session_id: str = None,
    ) -> str:
        """
        Генерирует (если не передано) и устанавливает сессию в redis и в куки

        :param response:
        :param refresh_token:
        :param session_id:
        :param user_id:
        :param ip_address:
        :param user_agent:
        :return: session_id
        """
        if not session_id:
            session_id = uuid.uuid4().hex
        response.set_cookie(
            key=self.COOKIE_SESSION_KEY,
            value=session_id,
            secure=True,
            httponly=True,
            samesite="none",
            max_age=self._config.JWT.REFRESH_EXPIRE_SECONDS,
            path=self.COOKIE_PATH
        )
        data = f"{refresh_token}:{ip_address}:{int(time.time())}:{user_agent}"
        await self._redis_client.hset(f'session_mapping:{user_id}', session_id, data)
        await self._redis_client.expire(f'session_mapping:{user_id}', 15_638_400)
        return session_id

    async def get_user_sessions(self, user_id: uuid.UUID) -> dict[str, dict[str, str]]:
        records = await self._redis_client.hgetall(f'session_mapping:{user_id}')
        response = dict()
        for key, value in records.items():
            data = value.split(":", 4)
            response[key] = dict(
                refresh_token=data[0],
                ip=data[1],
                time=int(data[2]),
                user_agent=data[3],
            )
        return response

    async def delete_session(self, user_id, session_id: str, response: Response = None) -> None:
        """
        Удаляет сессию из куков и из redis

        :param user_id:
        :param session_id
        :param response
        """
        if await self._redis_client.hexists(f'session_mapping:{user_id}', session_id):
            await self._redis_client.hdel(f'session_mapping:{user_id}', session_id)
        if response:
            response.delete_cookie(
                key=self.COOKIE_SESSION_KEY,
                secure=True,
                httponly=True,
                samesite="none",
                path=self.COOKIE_PATH
            )

    async def get_data_from_session(self, user_id: str, session_id: str) -> dict[str, str] | None:
        """
        Получает данные сессии из session

        :param user_id:
        :param session_id:
        :return: dict
        """
        data = await self._redis_client.hget(f'session_mapping:{user_id}', session_id)
        if not data:
            return None
        data = data.split(":", 4)
        return dict(
            refresh_token=data[0],
            ip=data[1],
            time=data[2],
            user_agent=data[3],
        )

    async def is_valid_session(self, user_id: str, session_id: str, refresh_token: str) -> bool:
        """
        Проверяет валидность сессии

        :param user_id:
        :param session_id:
        :param refresh_token:
        :return: True or False
        """
        data = await self.get_data_from_session(user_id, session_id)
        if not data:
            return False
        if data["refresh_token"] != refresh_token:
            return False
        return True
