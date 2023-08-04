import uuid

from fastapi import APIRouter, Depends
from fastapi import status as http_status
from fastapi.requests import Request
from fastapi.responses import Response

from src.dependencies.services import get_services
from src.models import schemas
from src.services import ServiceFactory

from src.views.user import UserResponse, UserSmallResponse

router = APIRouter()


@router.get("/current", response_model=UserResponse, status_code=http_status.HTTP_200_OK)
async def get_current_user(services: ServiceFactory = Depends(get_services)):
    """
    Получить модель текущего пользователя

    Минимальная роль: USER.ONE

    Состояние: ACTIVE
    """
    return UserResponse(content=await services.user.get_me())


@router.put("/update", response_model=None, status_code=http_status.HTTP_204_NO_CONTENT)
async def update_current_user(data: schemas.UserUpdate, services: ServiceFactory = Depends(get_services)):
    """
    Обновить данные текущего пользователя

    Минимальная роль: USER.ONE

    Состояние: ACTIVE
    """
    await services.user.update_me(data)


@router.put("/update/password", response_model=None, status_code=http_status.HTTP_204_NO_CONTENT)
async def update_password(old_password: str, new_password: str, services: ServiceFactory = Depends(get_services)):
    """
    Обновить пароль текущего пользователя

    Минимальная роль: USER.ONE

    Состояние: ACTIVE

    """
    await services.user.update_password(old_password, new_password)


@router.put("/update/{user_id}", response_model=None, status_code=http_status.HTTP_204_NO_CONTENT)
async def update_user(
        user_id: uuid.UUID,
        data: schemas.UserUpdateByAdmin,
        services: ServiceFactory = Depends(get_services)
):
    """
    Обновить данные пользователя по id

    Минимальная роль: ADMIN.ONE

    Состояние: ACTIVE
    """
    await services.user.update_user(user_id, data)


@router.get("/{user_id}", response_model=UserSmallResponse, status_code=http_status.HTTP_200_OK)
async def get_user(user_id: uuid.UUID, services: ServiceFactory = Depends(get_services)):
    """
    Получить модель пользователя по id

    Минимальная роль: GUEST.ONE
    """
    return UserSmallResponse(content=await services.user.get_user(user_id))


@router.delete("/delete", response_model=None, status_code=http_status.HTTP_204_NO_CONTENT)
async def delete_current_user(
        password: str,
        request: Request,
        response: Response,
        services: ServiceFactory = Depends(get_services)
):
    """
    Удалить текущего пользователя

    Минимальная роль: USER.ONE

    Состояние: ACTIVE
    """
    await services.user.delete_me(password)
    await services.auth.logout(request, response)
