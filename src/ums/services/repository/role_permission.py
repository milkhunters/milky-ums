from ums.models import tables
from ums.services.repository.base import BaseRepository


class RolePermissionRepo(BaseRepository[tables.RolePermission]):
    table = tables.RolePermission
