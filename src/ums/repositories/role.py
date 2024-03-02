from sqlalchemy import select, text, insert, delete
from sqlalchemy.orm import joinedload

from ums.models import tables
from ums.models.schemas.role import RoleID, PermissionID
from ums.repositories.base import BaseRepository


class RoleRepo(BaseRepository[tables.Role]):
    table = tables.Role

    async def get(self, as_full: bool = False, **kwargs) -> tables.Role | None:
        req = select(self.table).filter_by(**kwargs)
        if as_full:
            req = req.options(joinedload(self.table.permissions))
        return (await self._session.execute(req)).scalars().first()

    async def get_all(
            self, limit: int = 100,
            offset: int = 0,
            order_by: str = "id",
            as_full: bool = False,
            **kwargs
    ) -> list[tables.Role]:
        req = select(self.table).filter_by(**kwargs)
        if as_full:
            req = req.options(joinedload(self.table.permissions))

        result = (await self._session.execute(req.order_by(text(order_by)).limit(limit).offset(offset))).unique()
        return result.scalars().all()

    async def add_link(self, role_id: RoleID, permission_id: PermissionID, commit: bool = True):
        stmt = insert(tables.RolePermission).values(role_id=role_id, permission_id=permission_id)
        await self._session.execute(stmt)
        if commit:
            await self._session.commit()

    async def remove_link(self, role_id: RoleID, permission_id: PermissionID, commit: bool = True):
        stmt = (
            delete(tables.RolePermission)
            .where(tables.RolePermission.role_id == role_id)
            .where(tables.RolePermission.permission_id == permission_id)
        )
        await self._session.execute(stmt)
        if commit:
            await self._session.commit()

    async def has_link(self, role_id: RoleID, permission_id: PermissionID) -> bool:
        stmt = (
            select(tables.RolePermission)
            .where(tables.RolePermission.role_id == role_id)
            .where(tables.RolePermission.permission_id == permission_id)
        )
        return (await self._session.execute(stmt)).scalar_one_or_none() is not None
