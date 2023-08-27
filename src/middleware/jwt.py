from starlette.authentication import AuthCredentials
from starlette.middleware.base import BaseHTTPMiddleware
from starlette.middleware.exceptions import ExceptionMiddleware

from fastapi.requests import Request

from src.models.auth import AuthenticatedUser, UnauthenticatedUser
from src.services.auth import JWTManager
from src.services.auth import SessionManager


class JWTMiddlewareHTTP(BaseHTTPMiddleware):

    def __init__(self, app: ExceptionMiddleware):
        super().__init__(app)

    async def dispatch(self, request: Request, call_next):
        jwt = JWTManager(config=request.app.state.config)
        session = SessionManager(
            redis_client=request.app.state.redis,
            config=request.app.state.config
        )

        # States
        session_id = session.get_session_id(request)
        current_tokens = jwt.get_jwt_cookie(request)
        is_valid_session = False

        # ----- pre_process -----
        is_valid_access_token = jwt.is_valid_access_token(current_tokens.access_token)
        is_valid_refresh_token = jwt.is_valid_refresh_token(current_tokens.refresh_token)

        if is_valid_refresh_token and session_id:
            # Проверка валидности сессии
            if await session.is_valid_session(session_id, current_tokens.refresh_token):
                is_valid_session = True

        is_auth = (is_valid_access_token and is_valid_refresh_token and is_valid_session)

        # Установка данных авторизации
        if is_auth:
            payload = jwt.decode_access_token(current_tokens.access_token)
            request.scope["user"] = AuthenticatedUser(
                **payload.model_dump(),
                ip=request.client.host,
                is_valid_access_token=is_valid_access_token,
                is_valid_refresh_token=is_valid_refresh_token,
                is_valid_session=is_valid_session
            )
            request.scope["auth"] = AuthCredentials(["authenticated"])
        else:
            request.scope["user"] = UnauthenticatedUser(
                ip=request.client.host,
                is_valid_access_token=is_valid_access_token,
                is_valid_refresh_token=is_valid_refresh_token,
                is_valid_session=is_valid_session
            )
            request.scope["auth"] = AuthCredentials()

        response = await call_next(request)

        # ----- post_process -----

        return response
