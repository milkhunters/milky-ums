from starlette.authentication import AuthCredentials
from starlette.middleware.base import BaseHTTPMiddleware
from starlette.middleware.exceptions import ExceptionMiddleware

from fastapi.requests import Request
from user_agents import parse

from src.models.auth import AuthenticatedUser, UnauthenticatedUser
from src.services.auth import JWTManager
from src.services.auth import SessionManager
from src.utils import RedisClient


class JWTMiddlewareHTTP(BaseHTTPMiddleware):

    def __init__(self, app: ExceptionMiddleware):
        super().__init__(app)

    async def dispatch(self, request: Request, call_next):
        jwt = JWTManager(config=request.app.state.config)
        session = SessionManager(
            redis_client=request.app.state.redis_sessions,
            config=request.app.state.config
        )
        redis_reauth: RedisClient = request.app.state.redis_reauth

        # States
        session_id = session.get_session_id(request)
        current_tokens = jwt.get_jwt_cookie(request)
        is_valid_session = False

        # ----- pre_process -----
        is_valid_access_token = jwt.is_valid_access_token(current_tokens.access_token)
        is_valid_refresh_token = jwt.is_valid_refresh_token(current_tokens.refresh_token)

        if is_valid_refresh_token and session_id:
            user_id = jwt.decode_refresh_token(current_tokens.refresh_token).id
            is_valid_session = await session.is_valid_session(user_id, session_id, current_tokens.refresh_token)

        is_auth = (is_valid_access_token and is_valid_refresh_token and is_valid_session)

        if is_auth:

            # Если требуется обновить данные пользователя, то запрещаем
            # авторизацию по старому refresh токену, из-за чего пользователю
            # придется обновить токены или дождаться истечения access токена

            old_ref_token = await redis_reauth.get(session_id)
            if old_ref_token and old_ref_token == current_tokens.refresh_token:
                is_auth = False

        # Установка данных авторизации
        if is_auth:
            payload = jwt.decode_access_token(current_tokens.access_token)
            request.scope["user"] = AuthenticatedUser(
                **payload.model_dump(),
                ip=request.client.host,
                user_agent=parse(request.headers.get("User-Agent")),
                is_valid_access_token=is_valid_access_token,
                is_valid_refresh_token=is_valid_refresh_token,
                is_valid_session=is_valid_session
            )
            request.scope["auth"] = AuthCredentials(["authenticated"])
        else:
            request.scope["user"] = UnauthenticatedUser(
                ip=request.client.host,
                user_agent=parse(request.headers.get("User-Agent")),
                is_valid_access_token=is_valid_access_token,
                is_valid_refresh_token=is_valid_refresh_token,
                is_valid_session=is_valid_session
            )
            request.scope["auth"] = AuthCredentials()

        response = await call_next(request)

        # ----- post_process -----

        return response
