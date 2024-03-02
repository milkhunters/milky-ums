from fastapi import Depends
from fastapi.requests import Request

from ums.dependencies.repos import get_repos
from ums.services import ServiceFactory
from ums.repositories import RepoFactory


async def get_services(request: Request, repos: RepoFactory = Depends(get_repos)) -> ServiceFactory:
    global_scope = request.app.state
    local_scope = request.scope

    yield ServiceFactory(
        repos,
        current_user=local_scope.get("user"),
        redis_reauth=global_scope.redis_reauth,
        confirm_manager=global_scope.confirm_manager,
        session_manager=global_scope.session_manager,
        config=global_scope.config,
        jwt=global_scope.jwt,
        email_sender=global_scope.email_sender,
        file_storage=global_scope.file_storage,
        lazy_session=global_scope.db_session
    )
