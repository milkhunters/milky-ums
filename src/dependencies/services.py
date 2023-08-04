from fastapi import Depends
from fastapi.requests import Request

from src.dependencies.repos import get_repos
from src.services import ServiceFactory
from src.services.repository import RepoFactory


async def get_services(request: Request, repos: RepoFactory = Depends(get_repos)) -> ServiceFactory:
    global_scope = request.app.state
    local_scope = request.scope

    yield ServiceFactory(
        repos,
        current_user=local_scope.get("user"),
        redis_client=global_scope.redis,
        config=global_scope.config,
        email_sender=global_scope.email_sender,
    )
