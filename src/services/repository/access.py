from sqlalchemy import insert, update, delete, func, select
from sqlalchemy.orm import joinedload

from src.models import tables
from src.services.repository.base import BaseRepository


class AccessRepo(BaseRepository[tables.Access]):
    table = tables.Access

    async def get(self, as_full: bool = False, **kwargs) -> tables.Access | None:
        req = select(self.table).filter_by(**kwargs)
        if as_full:
            req = req.options(joinedload(self.table.roles))
        return (await self._session.execute(req)).scalars().first()
