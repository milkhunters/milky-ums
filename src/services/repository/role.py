from sqlalchemy import insert, update, delete, func, select, text
from sqlalchemy.orm import joinedload

from src.models import tables
from src.services.repository.base import BaseRepository


class RoleRepo(BaseRepository[tables.Role]):
    table = tables.Role

    async def get(self, as_full: bool = False, **kwargs) -> tables.Role | None:
        req = select(self.table).filter_by(**kwargs)
        if as_full:
            req = req.options(joinedload(self.table.access))
        return (await self._session.execute(req)).scalars().first()

    async def get_all(
            self, limit: int = 100,
            offset: int = 0,
            order_by: str = "id",
            as_full: bool = False,
            **kwargs
    ) -> list[tables.Role]:
        req = select(self.table).filter_by(**kwargs).order_by(text(order_by))
        if as_full:
            req = req.options(joinedload(self.table.access))

        result = await self._session.execute(req.limit(limit).offset(offset))
        return result.scalars().all()
