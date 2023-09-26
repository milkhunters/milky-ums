import uuid

from src import exceptions
from src.services.repository import RoleRepo
from src.services.repository import PermissionRepo
from src.services.repository import RolePermissionRepo
from src.services.auth.filters import permission_filter, state_filter

from src.models import schemas
from src.models.auth import BaseUser, UnauthenticatedUser
from src.models.state import UserState
from src.models.permission import Permission


class RoleApplicationService:

    def __init__(
            self,
            current_user: BaseUser,
            *,
            role_repo: RoleRepo,
            permission_repo: PermissionRepo,
            role_permission_repo: RolePermissionRepo,
    ):
        self._current_user = current_user
        self._role_repo = role_repo
        self._permission_repo = permission_repo
        self._role_permission_repo = role_permission_repo

    @permission_filter(Permission.GET_ROLE)
    @state_filter(UserState.ACTIVE)
    async def get_roles(self) -> list[schemas.Role]:
        roles = await self._role_repo.get_all(as_full=True)
        return [schemas.Role.model_validate(role) for role in roles]

    @permission_filter(Permission.GET_ROLE)
    @state_filter(UserState.ACTIVE)
    async def get_permissiones(self) -> list[schemas.Permission]:
        permissiones = await self._permission_repo.get_all()
        return [schemas.Permission.model_validate(permission) for permission in permissiones]

    @permission_filter(Permission.UPDATE_ROLE)
    @state_filter(UserState.ACTIVE)
    async def update_role(self, role_id: uuid.UUID, data: schemas.UpdateRole) -> None:
        role = await self._role_repo.get(id=role_id)
        if not role:
            raise exceptions.NotFound(f"Роль с id:{role_id} не найдена!")

        await self._role_repo.update(
            id=role_id,
            **data.model_dump(exclude_unset=True)
        )

    @permission_filter(Permission.UPDATE_ROLE)
    @state_filter(UserState.ACTIVE)
    async def set_role_permission(self, role_id: uuid.UUID, permission_tag: str) -> None:
        role = await self._role_repo.get(id=role_id)
        if not role:
            raise exceptions.NotFound(f"Роль с id:{role_id} не найдена!")

        permission = await self._permission_repo.get(title=permission_tag)
        if not permission:
            raise exceptions.NotFound(f"Доступ с тегом: {permission_tag} не найден!")

        await self._role_permission_repo.create(role_id=role_id, permission_id=permission.id)

    @permission_filter(Permission.CREATE_ROLE)
    @state_filter(UserState.ACTIVE)
    async def create_role(self, data: schemas.CreateRole) -> schemas.Role:
        role = await self._role_repo.get(title=data.title)
        if role:
            raise exceptions.BadRequest(f"Роль с названием: {data.title!r} уже существует!")
        return await self._role_repo.create(**data.model_dump())

    @permission_filter(Permission.CREATE_ROLE)
    @state_filter(UserState.ACTIVE)
    async def create_permission(self, data: schemas.CreatePermission) -> schemas.Permission:
        permission = await self._permission_repo.get(title=data.title)
        if permission:
            raise exceptions.BadRequest(f"Доступ с тегом: {data.title!r} уже существует!")
        return await self._permission_repo.create(**data.model_dump())

    @permission_filter(Permission.DELETE_ROLE)
    @state_filter(UserState.ACTIVE)
    async def delete_role(self, role_id: uuid.UUID) -> None:
        role = await self._role_repo.get(id=role_id)
        if not role:
            raise exceptions.NotFound(f"Роль с id:{role_id} не найдена!")
        await self._role_repo.delete(id=role_id)

    @permission_filter(Permission.DELETE_ROLE)
    @state_filter(UserState.ACTIVE)
    async def delete_role_permission(self, role_id: uuid.UUID, permission_id: uuid.UUID) -> None:
        role_permission = await self._role_permission_repo.get(role_id=role_id, permission_id=permission_id)
        if not role_permission:
            raise exceptions.NotFound(f"Cвязь роли с id:{role_id} и доступа с id:{permission_id} не найдена!")
        await self._role_permission_repo.delete(role_permission.id)

    @permission_filter(Permission.DELETE_ROLE)
    @state_filter(UserState.ACTIVE)
    async def delete_permission(self, permission_id: uuid.UUID):
        permission = await self._permission_repo.get(id=permission_id)
        if not permission:
            raise exceptions.NotFound(f"Доступ с id:{permission_id} не найден!")

        role_permission = await self._role_permission_repo.get(permission_id=permission_id)
        if role_permission:
            raise exceptions.BadRequest(f"Доступ с id:{permission_id} используется в роли c id:{role_permission.role_id}!")

        await self._permission_repo.delete(id=permission_id)

    async def guest_permission(self) -> list[str]:
        return list(UnauthenticatedUser().permissions)

    async def app_permission(self) -> list[str]:
        return [permission.value for permission in Permission]