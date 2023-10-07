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
        redis_sessions=global_scope.redis_sessions,
        redis_reauth=global_scope.redis_reauth,
        redis_confirmations=global_scope.redis_confirmations,
        config=global_scope.config,
        email_sender=global_scope.email_sender,
        file_storage=global_scope.file_storage,
        lazy_session=global_scope.db_session
    )
