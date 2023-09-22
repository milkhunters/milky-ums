import uuid

from fastapi import APIRouter, Depends
from fastapi import status as http_status

from src.models import schemas
from src.services import ServiceFactory
from src.dependencies.services import get_services
from src.views import RolesResponse, RoleResponse, AccessResponse, AccessesResponse

router = APIRouter()


@router.post("/new", response_model=RoleResponse, status_code=http_status.HTTP_200_OK)
async def role_create(
        data: schemas.CreateRole,
        services: ServiceFactory = Depends(get_services),
):
    """
    Создать роль

    Требуемое состояние: ACTIVE
    Требуемые права доступа: CAN_CREATE_ROLE
    """
    return RoleResponse(content=await services.role.create_role(data=data))


@router.get("/list", response_model=RolesResponse, status_code=http_status.HTTP_200_OK)
async def role_list(services: ServiceFactory = Depends(get_services)):
    """
    Получить список ролей

    Требуемое состояние: ACTIVE
    Требуемые права доступа: CAN_GET_ROLE
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

    Требуемые права доступа: CAN_UPDATE_ROLE
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
    Требуемые права доступа: CAN_DELETE_ROLE
    """
    await services.role.delete_role(role_id=role_id)


@router.post("/link/new", status_code=http_status.HTTP_204_NO_CONTENT)
async def role_set_access(
        role_id: uuid.UUID,
        access_tag: str,
        services: ServiceFactory = Depends(get_services),
):
    """
    Установить доступ для роли

    Требуемое состояние: ACTIVE

    Требуемые права доступа: CAN_UPDATE_ROLE
    """
    await services.role.set_role_access(role_id=role_id, access_tag=access_tag)


@router.delete("/link", status_code=http_status.HTTP_200_OK)
async def role_delete_role_access(
        role_id: uuid.UUID,
        access_id: uuid.UUID,
        services: ServiceFactory = Depends(get_services),
):
    """
    Удалить доступ у роли

    Требуемое состояние: ACTIVE

    Требуемые права доступа: CAN_DELETE_ROLE
    """
    await services.role.delete_role_access(role_id=role_id, access_id=access_id)


@router.post("/access/new", response_model=AccessResponse, status_code=http_status.HTTP_200_OK)
async def role_create_access(
        data: schemas.CreateAccess,
        services: ServiceFactory = Depends(get_services),
):
    """
    Создать доступ

    Требуемое состояние: ACTIVE

    Требуемые права доступа: CAN_CREATE_ROLE
    """
    return AccessResponse(content=await services.role.create_access(data=data))


@router.get("/access/list", response_model=AccessesResponse, status_code=http_status.HTTP_200_OK)
async def role_access_list(services: ServiceFactory = Depends(get_services)):
    """
    Получить список доступов

    Требуемое состояние: ACTIVE

    Требуемые права доступа: CAN_GET_ROLE
    """
    return AccessesResponse(content=await services.role.get_accesses())


@router.get("/access/guest", response_model=list[str], status_code=http_status.HTTP_200_OK)
async def role_guest_access(services: ServiceFactory = Depends(get_services)):
    """
    Список доступов для локального гостя

    Требуемые права доступа: None
    """
    return await services.role.guest_access()


@router.get("/access/app", response_model=list[str], status_code=http_status.HTTP_200_OK)
async def role_app_access(services: ServiceFactory = Depends(get_services)):
    """
    Список доступов приложения

    Требуемые права доступа: None
    """
    return await services.role.app_access()


@router.delete("/access", status_code=http_status.HTTP_204_NO_CONTENT)
async def role_delete_access(
        access_id: uuid.UUID,
        services: ServiceFactory = Depends(get_services),
):
    """
    Удалить доступ

    Требуемое состояние: ACTIVE

    Требуемые права доступа: CAN_DELETE_ROLE
    """
    await services.role.delete_access(access_id=access_id)
