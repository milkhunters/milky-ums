from .user import UserRepo
from .role import RoleRepo
from .access import AccessRepo
from .role_access import RoleAccessRepo


class RepoFactory:
    def __init__(self, session):
        self._session = session

    @property
    def user(self) -> UserRepo:
        return UserRepo(self._session)

    @property
    def role(self) -> RoleRepo:
        return RoleRepo(self._session)

    @property
    def access(self) -> AccessRepo:
        return AccessRepo(self._session)

    @property
    def role_access(self) -> RoleAccessRepo:
        return RoleAccessRepo(self._session)
