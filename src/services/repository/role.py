from sqlalchemy import select, text
from sqlalchemy.orm import joinedload

from src.models import tables
from src.services.repository.base import BaseRepository


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
