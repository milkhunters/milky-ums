from src.models import tables
from src.services.repository.base import BaseRepository


class RolePermissionRepo(BaseRepository[tables.RolePermission]):
    table = tables.RolePermission
