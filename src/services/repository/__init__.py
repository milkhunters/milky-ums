from .user import UserRepo
from .role import RoleRepo
from .access import PermissionRepo
from .role_permission import RolePermissionRepo


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
    def permission(self) -> PermissionRepo:
        return PermissionRepo(self._session)

    @property
    def role_permission(self) -> RolePermissionRepo:
        return RolePermissionRepo(self._session)
