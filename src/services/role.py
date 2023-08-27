import uuid

from src import exceptions
from src.services.repository import RoleRepo
from src.services.repository import AccessRepo
from src.services.repository import RoleAccessRepo
from src.services.auth.filters import access_filter, state_filter

from src.models import schemas
from src.models.auth import BaseUser, UnauthenticatedUser
from src.models.state import UserState
from src.models.access import AccessTags


class RoleApplicationService:

    def __init__(
            self,
            current_user: BaseUser,
            *,
            role_repo: RoleRepo,
            access_repo: AccessRepo,
            role_access_repo: RoleAccessRepo,
    ):
        self._current_user = current_user
        self._role_repo = role_repo
        self._access_repo = access_repo
        self._role_access_repo = role_access_repo

    @access_filter(AccessTags.CAN_GET_ROLE)
    @state_filter(UserState.ACTIVE)
    async def get_roles(self) -> list[schemas.Role]:
        roles = await self._role_repo.get_all(as_full=True)
        return [schemas.Role.model_validate(role) for role in roles]

    @access_filter(AccessTags.CAN_GET_ROLE)
    @state_filter(UserState.ACTIVE)
    async def get_accesses(self) -> list[schemas.Access]:
        accesses = await self._access_repo.get_all()
        return [schemas.Access.model_validate(access) for access in accesses]

    @access_filter(AccessTags.CAN_UPDATE_ROLE)
    @state_filter(UserState.ACTIVE)
    async def update_role(self, role_id: uuid.UUID, data: schemas.UpdateRole) -> None:
        role = await self._role_repo.get(id=role_id)
        if not role:
            raise exceptions.NotFound(f"Роль с id:{role_id} не найдена!")

        await self._role_repo.update(
            id=role_id,
            **data.model_dump(exclude_unset=True)
        )

    @access_filter(AccessTags.CAN_UPDATE_ROLE)
    @state_filter(UserState.ACTIVE)
    async def set_role_access(self, role_id: uuid.UUID, access_tag: str) -> None:
        role = await self._role_repo.get(id=role_id)
        if not role:
            raise exceptions.NotFound(f"Роль с id:{role_id} не найдена!")

        access = await self._access_repo.get(tag=access_tag)
        if not access:
            raise exceptions.NotFound(f"Доступ с тегом:{access_tag} не найден!")

        await self._role_access_repo.create(role_id=role_id, access_id=access.id)

    @access_filter(AccessTags.CAN_CREATE_ROLE)
    @state_filter(UserState.ACTIVE)
    async def create_role(self, data: schemas.CreateRole) -> schemas.Role:
        role = await self._role_repo.get(title=data.title)
        if role:
            raise exceptions.BadRequest(f"Роль с названием:{data.title} уже существует!")
        return await self._role_repo.create(**data.model_dump())

    @access_filter(AccessTags.CAN_CREATE_ROLE)
    @state_filter(UserState.ACTIVE)
    async def create_access(self, data: schemas.CreateAccess) -> schemas.Access:
        access = await self._access_repo.get(tag=data.tag)
        if access:
            raise exceptions.BadRequest(f"Доступ с тегом:{data.tag} уже существует!")
        return await self._access_repo.create(**data.model_dump())

    @access_filter(AccessTags.CAN_DELETE_ROLE)
    @state_filter(UserState.ACTIVE)
    async def delete_role(self, role_id: uuid.UUID) -> None:
        role = await self._role_repo.get(id=role_id)
        if not role:
            raise exceptions.NotFound(f"Роль с id:{role_id} не найдена!")
        await self._role_repo.delete(id=role_id)

    @access_filter(AccessTags.CAN_DELETE_ROLE)
    @state_filter(UserState.ACTIVE)
    async def delete_role_access(self, role_id: uuid.UUID, access_id: uuid.UUID) -> None:
        role_access = await self._role_access_repo.get(role_id=role_id, access_id=access_id)
        if not role_access:
            raise exceptions.NotFound(f"Cвязь роли с id:{role_id} и доступа с id:{access_id} не найдена!")
        await self._role_access_repo.delete(role_access.id)

    @access_filter(AccessTags.CAN_DELETE_ROLE)
    @state_filter(UserState.ACTIVE)
    async def delete_access(self, access_id: uuid.UUID):
        access = await self._access_repo.get(id=access_id)
        if not access:
            raise exceptions.NotFound(f"Доступ с id:{access_id} не найден!")

        role_access = await self._role_access_repo.get(access_id=access_id)
        if role_access:
            raise exceptions.BadRequest(f"Доступ с id:{access_id} используется в роли c id:{role_access.role_id}!")

        await self._access_repo.delete(id=access_id)

    async def guest_access(self) -> list[str]:
        return list(UnauthenticatedUser().access)