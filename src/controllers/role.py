import uuid

from fastapi import APIRouter, Depends
from fastapi import status as http_status

from src.models import schemas
from src.services import ServiceFactory
from src.dependencies.services import get_services
from src.views import RolesResponse, RoleResponse, PermissionResponse, PermissionsResponse

router = APIRouter()


@router.post("/new", response_model=RoleResponse, status_code=http_status.HTTP_200_OK)
async def role_create(
        data: schemas.CreateRole,
        services: ServiceFactory = Depends(get_services),
):
    """
    Создать роль

    Требуемое состояние: ACTIVE
    Требуемые права доступа: CREATE_ROLE
    """
    return RoleResponse(content=await services.role.create_role(data=data))


@router.get("/list", response_model=RolesResponse, status_code=http_status.HTTP_200_OK)
async def role_list(services: ServiceFactory = Depends(get_services)):
    """
    Получить список ролей

    Требуемое состояние: ACTIVE
    Требуемые права доступа: GET_ROLE
    """
    return RolesResponse(content=await services.role.get_roles())


@router.put("", status_code=http_status.HTTP_200_OK)
async def role_update(
        role_id: uuid.UUID,
        data: schemas.UpdateRole,
        services: ServiceFactory = Depends(get_services),
):
    """
    Обновить роль

    Требуемое состояние: ACTIVE

    Требуемые права доступа: UPDATE_ROLE
    """
    await services.role.update_role(role_id=role_id, data=data)


@router.delete("", status_code=http_status.HTTP_200_OK)
async def role_delete(
        role_id: uuid.UUID,
        services: ServiceFactory = Depends(get_services),
):
    """
    Удалить роль

    Требуемое состояние: ACTIVE
    Требуемые права доступа: DELETE_ROLE
    """
    await services.role.delete_role(role_id=role_id)


@router.post("/link/new", status_code=http_status.HTTP_204_NO_CONTENT)
async def role_set_permission(
        role_id: uuid.UUID,
        permission_tag: str,
        services: ServiceFactory = Depends(get_services),
):
    """
    Установить доступ для роли

    Требуемое состояние: ACTIVE

    Требуемые права доступа: UPDATE_ROLE
    """
    await services.role.set_role_permission(role_id=role_id, permission_tag=permission_tag)


@router.delete("/link", status_code=http_status.HTTP_200_OK)
async def role_delete_role_permission(
        role_id: uuid.UUID,
        permission_id: uuid.UUID,
        services: ServiceFactory = Depends(get_services),
):
    """
    Удалить доступ у роли

    Требуемое состояние: ACTIVE

    Требуемые права доступа: DELETE_ROLE
    """
    await services.role.delete_role_permission(role_id=role_id, permission_id=permission_id)


@router.post("/permission/new", response_model=PermissionResponse, status_code=http_status.HTTP_200_OK)
async def role_create_permission(
        data: schemas.CreatePermission,
        services: ServiceFactory = Depends(get_services),
):
    """
    Создать доступ

    Требуемое состояние: ACTIVE

    Требуемые права доступа: CREATE_ROLE
    """
    return PermissionResponse(content=await services.role.create_permission(data=data))


@router.get("/permission/list", response_model=PermissionsResponse, status_code=http_status.HTTP_200_OK)
async def role_permission_list(services: ServiceFactory = Depends(get_services)):
    """
    Получить список доступов

    Требуемое состояние: ACTIVE

    Требуемые права доступа: GET_ROLE
    """
    return PermissionsResponse(content=await services.role.get_permissiones())


@router.get("/permission/guest", response_model=list[str], status_code=http_status.HTTP_200_OK)
async def role_guest_permission(services: ServiceFactory = Depends(get_services)):
    """
    Список доступов для локального гостя

    Требуемые права доступа: None
    """
    return await services.role.guest_permission()


@router.get("/permission/app", response_model=list[str], status_code=http_status.HTTP_200_OK)
async def role_app_permission(services: ServiceFactory = Depends(get_services)):
    """
    Список доступов приложения

    Требуемые права доступа: None
    """
    return await services.role.app_permission()


@router.delete("/permission", status_code=http_status.HTTP_204_NO_CONTENT)
async def role_delete_permission(
        permission_id: uuid.UUID,
        services: ServiceFactory = Depends(get_services),
):
    """
    Удалить доступ

    Требуемое состояние: ACTIVE

    Требуемые права доступа: DELETE_ROLE
    """
    await services.role.delete_role_permission(permission_id=permission_id)
