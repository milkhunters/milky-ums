import time
from typing import Optional

import jwt
from fastapi import Response, Request
from fastapi.websockets import WebSocket

from src.config import Config
from src.models import schemas


class JWTManager:
    ALGORITHM = "HS256"
    ACCESS_TOKEN_EXPIRE_MINUTES = 15  # 15 minutes
    REFRESH_TOKEN_EXPIRE_MINUTES = 60 * 24 * 7  # 7 days

    COOKIE_EXP = 31536000
    COOKIE_PATH = "/api"
    COOKIE_DOMAIN = None
    COOKIE_ACCESS_KEY = "access_token"
    COOKIE_REFRESH_KEY = "refresh_token"

    def __init__(self, config: Config):
        self._config = config

        self.JWT_ACCESS_SECRET_KEY = config.JWT.ACCESS_SECRET_KEY
        self.JWT_REFRESH_SECRET_KEY = config.JWT.REFRESH_SECRET_KEY

    def is_valid_refresh_token(self, token: str) -> bool:
        """
        Проверяет refresh-токен на валидность
        :param token:
        :return:
        """
        return self._is_valid_jwt(token, self.JWT_REFRESH_SECRET_KEY)

    def is_valid_access_token(self, token: str) -> bool:
        """
        Проверяет access-токен на валидность
        :param token:
        :return:
        """
        return self._is_valid_jwt(token, self.JWT_ACCESS_SECRET_KEY)

    def decode_access_token(self, token: str) -> schemas.TokenPayload:
        """
        Декодирует access-токен (получает payload)
        :param token:
        :return:
        """
        return self._decode_jwt(token, self.JWT_ACCESS_SECRET_KEY)

    def decode_refresh_token(self, token: str) -> schemas.TokenPayload:
        """
        Декодирует refresh-токен (получает payload)
        :param token:
        :return:
        """
        return self._decode_jwt(token, self.JWT_REFRESH_SECRET_KEY)

    def generate_tokens(self, id: str, username: str, role_id: int, state_id: int) -> schemas.Tokens:
        """
        Генерирует access- и refresh-токены
        :param id:
        :param username:
        :param role_id:
        :param state_id:
        :return:
        """
        return schemas.Tokens(
            access_token=self._generate_token(
                exp_minutes=self.ACCESS_TOKEN_EXPIRE_MINUTES,
                secret_key=self.JWT_ACCESS_SECRET_KEY,
                id=id,
                username=username,
                role_id=role_id,
                state_id=state_id,
            ),
            refresh_token=self._generate_token(
                exp_minutes=self.REFRESH_TOKEN_EXPIRE_MINUTES,
                secret_key=self.JWT_REFRESH_SECRET_KEY,
                id=id,
                username=username,
                role_id=role_id,
                state_id=state_id,
            )
        )

    def set_jwt_cookie(self, response: Response, tokens: schemas.Tokens) -> None:
        """
        Устанавливает в куки access- и refresh-токены
        :param response:
        :param tokens:
        :return:
        """
        response.set_cookie(
            key=self.COOKIE_ACCESS_KEY,
            value=tokens.access_token,
            secure=self._config.IS_SECURE_COOKIE,
            httponly=True,
            samesite="strict",
            max_age=self.COOKIE_EXP,
            path=self.COOKIE_PATH,
            domain=self.COOKIE_DOMAIN
        )
        response.set_cookie(
            key=self.COOKIE_REFRESH_KEY,
            value=tokens.refresh_token,
            secure=self._config.IS_SECURE_COOKIE,
            httponly=True,
            samesite="strict",
            max_age=self.COOKIE_EXP,
            path=self.COOKIE_PATH,
            domain=self.COOKIE_DOMAIN
        )

    def get_jwt_cookie(self, req_obj: Request | WebSocket) -> Optional[schemas.Tokens]:
        """
        Получает из кук access и refresh-токены
        :param req_obj:
        :return: None или Tokens
        """
        access_token = req_obj.cookies.get(self.COOKIE_ACCESS_KEY)
        refresh_token = req_obj.cookies.get(self.COOKIE_REFRESH_KEY)
        if not access_token or not refresh_token:
            return None
        return schemas.Tokens(access_token=access_token, refresh_token=refresh_token)

    def delete_jwt_cookie(self, response: Response) -> None:
        """
        Удаляет из кук access и refresh-токены
        :param response:
        :return:
        """
        tokens = schemas.Tokens(access_token="", refresh_token="")
        self.set_jwt_cookie(response, tokens)

    def _is_valid_jwt(self, token: str, secret_key: str) -> bool:
        try:
            jwt.decode(token, secret_key, algorithms=self.ALGORITHM)
        except (jwt.exceptions.InvalidTokenError, jwt.exceptions.ExpiredSignatureError, jwt.exceptions.DecodeError):
            return False
        return True

    def _generate_token(self, exp_minutes: int, secret_key: str, **kwargs) -> str:
        """
        param: exp_minutes: время жизни токена в минутах
        param: secret_key: секретный ключ
        param: kwargs: параметры для payload
        :return: токен
        """
        payload = schemas.TokenPayload(**kwargs, exp=int(time.time() + exp_minutes * 60))
        return jwt.encode(payload.model_dump(), secret_key, algorithm=self.ALGORITHM)

    def _decode_jwt(self, token: str, secret_key: str) -> schemas.TokenPayload:
        """
        param: token: токен
        param: secret_key: секретный ключ
        :return: payload
        """
        return schemas.TokenPayload.parse_obj(jwt.decode(
            token,
            secret_key,
            algorithms=self.ALGORITHM,
            options={"verify_signature": False}
        ))