from fastapi.requests import Request

from ums.config import JWTConfig


async def get_jwt_config(request: Request) -> JWTConfig:
    global_scope = request.app.state

    return global_scope.config.JWT
