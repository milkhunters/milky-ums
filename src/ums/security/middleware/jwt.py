import logging

from fastapi.requests import Request
from starlette.authentication import AuthCredentials
from starlette.middleware.base import BaseHTTPMiddleware
from starlette.middleware.exceptions import ExceptionMiddleware
from user_agents import parse

from ums.models.auth import AuthenticatedUser, UnauthenticatedUser
from ums.security.jwt import JwtTokenProcessor
from ums.security.models import JWTTokens
from ums.repositories import UserRepo

from ums.services import SessionManager
from ums.utils import RedisClient


class JWTMiddlewareHTTP(BaseHTTPMiddleware):

    def __init__(self, app: ExceptionMiddleware):
        super().__init__(app)

    async def dispatch(self, request: Request, call_next):
        global_state = request.app.state

        jwt: JwtTokenProcessor = global_state.jwt
        db_session = request.app.state.db_session
        session: SessionManager = request.app.state.session_manager
        redis: RedisClient = request.app.state.redis_reauth

        # States
        current_tokens = JWTTokens(
            access_token=request.cookies.get("access_token"),
            refresh_token=request.cookies.get("refresh_token")
        )
        session_id = request.cookies.get("session_id")

        is_valid_access_token = False
        is_valid_refresh_token = False
        is_valid_session = False
        is_auth = False

        # ----- pre_process -----
        if current_tokens:
            is_valid_access_token = jwt.is_valid_token(current_tokens.access_token)
            is_valid_refresh_token = jwt.is_valid_token(current_tokens.refresh_token)

            if is_valid_refresh_token and session_id:
                user_id = jwt.validate_token(current_tokens.refresh_token).id
                is_valid_session = await session.is_valid_session(user_id, session_id, current_tokens.refresh_token)

            is_auth = (is_valid_access_token and is_valid_refresh_token and is_valid_session)

            if is_auth:

                # Если требуется обновить данные пользователя, то запрещаем
                # авторизацию по старому refresh token, из-за чего пользователю
                # придется обновить токены или дождаться истечения access токена

                old_ref_token = await redis.get(session_id)
                if old_ref_token and old_ref_token == current_tokens.refresh_token:
                    is_auth = False

        # Установка данных авторизации
        extra = {
            "ip": request.client.host,
            "user_agent": parse(request.headers.get("User-Agent")),
            "is_valid_access_token": is_valid_access_token,
            "is_valid_refresh_token": is_valid_refresh_token,
            "is_valid_session": is_valid_session
        }
        if is_auth:
            payload = jwt.validate_token(current_tokens.access_token)
            request.scope["user"] = AuthenticatedUser(**payload.model_dump(), **extra)
            request.scope["auth"] = AuthCredentials(["authenticated"])
        else:
            request.scope["user"] = UnauthenticatedUser(**extra)
            request.scope["auth"] = AuthCredentials()

        # ----- process -----
        response = await call_next(request)

        # ----- post_process -----

        return response
