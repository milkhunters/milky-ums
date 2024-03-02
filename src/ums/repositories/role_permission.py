from ums.models import tables
from ums.repositories.base import BaseRepository


class RolePermissionRepo(BaseRepository[tables.RolePermission]):
    table = tables.RolePermission
