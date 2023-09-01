from sqlalchemy import insert, update, delete, func, select
from sqlalchemy.orm import joinedload

from src.models import tables
from src.services.repository.base import BaseRepository


class RoleAccessRepo(BaseRepository[tables.RoleAccess]):
    table = tables.RoleAccess
