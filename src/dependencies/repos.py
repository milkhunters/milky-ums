from fastapi.requests import Request

from src.services.repository import RepoFactory


async def get_repos(request: Request) -> RepoFactory:
    global_scope = request.app.state

    async with global_scope.db_session() as session:
        yield RepoFactory(session)
