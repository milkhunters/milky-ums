import uuid

from fastapi import APIRouter, Depends, UploadFile
from fastapi import status as http_status
from fastapi.requests import Request
from fastapi.responses import Response

from src.dependencies.services import get_services
from src.models import schemas
from src.services import ServiceFactory
from src.views import SessionsResponse

from src.views.user import UserResponse, UserSmallResponse, UserAvatarResponse

router = APIRouter()


@router.get("/current", response_model=UserResponse, status_code=http_status.HTTP_200_OK)
async def get_current_user(services: ServiceFactory = Depends(get_services)):
    """
    Получить модель текущего пользователя

    Требуемые права доступа: CAN_GET_SELF

    Состояние: ACTIVE
    """
    return UserResponse(content=await services.user.get_me())


@router.put("/update", response_model=None, status_code=http_status.HTTP_204_NO_CONTENT)
async def update_current_user(data: schemas.UserUpdate, services: ServiceFactory = Depends(get_services)):
    """
    Обновить данные текущего пользователя

    Требуемые права доступа: CAN_UPDATE_SELF

    Состояние: ACTIVE
    """
    await services.user.update_me(data)


@router.put("/update/avatar", response_model=None, status_code=http_status.HTTP_200_OK)
async def update_avatar(file: UploadFile, services: ServiceFactory = Depends(get_services)):
    """
    Обновить аватар текущего пользователя

    Требуемые права доступа: CAN_UPDATE_SELF

    Состояние: ACTIVE
    """
    await services.user.update_avatar(file)


@router.put("/update/password", response_model=None, status_code=http_status.HTTP_204_NO_CONTENT)
async def update_password(old_password: str, new_password: str, services: ServiceFactory = Depends(get_services)):
    """
    Обновить пароль текущего пользователя

    Требуемые права доступа: CAN_UPDATE_SELF

    Состояние: ACTIVE

    """
    await services.user.update_password(old_password, new_password)


@router.put("/update/avatar/{user_id}", response_model=None, status_code=http_status.HTTP_200_OK)
async def update_user_avatar(file: UploadFile, user_id: uuid.UUID, services: ServiceFactory = Depends(get_services)):
    """
    Обновить пароль текущего пользователя

    Требуемые права доступа: CAN_UPDATE_USER

    Состояние: ACTIVE

    """
    await services.user.update_user_avatar(user_id, file)


@router.put("/update/{user_id}", response_model=None, status_code=http_status.HTTP_204_NO_CONTENT)
async def update_user(
        user_id: uuid.UUID,
        data: schemas.UserUpdateByAdmin,
        services: ServiceFactory = Depends(get_services)
):
    """
    Обновить данные пользователя по id

    Требуемые права доступа: CAN_UPDATE_USER

    Состояние: ACTIVE
    """
    await services.user.update_user(user_id, data)


@router.get("/{user_id}", response_model=UserSmallResponse, status_code=http_status.HTTP_200_OK)
async def get_user(user_id: uuid.UUID, services: ServiceFactory = Depends(get_services)):
    """
    Получить модель пользователя по id

    Требуемые права доступа: CAN_GET_USER
    """
    return UserSmallResponse(content=await services.user.get_user(user_id))


@router.get("/avatar/{user_id}", response_model=UserAvatarResponse, status_code=http_status.HTTP_200_OK)
async def get_avatar_url(user_id: uuid.UUID, services: ServiceFactory = Depends(get_services)):
    """
    Получить URL пользовательского аватара по id

    Требуемые права доступа: CAN_GET_USER
    """
    return UserAvatarResponse(content=await services.user.get_avatar_url(user_id))


@router.delete("", response_model=None, status_code=http_status.HTTP_204_NO_CONTENT)
async def delete_current_user(
        password: str,
        request: Request,
        response: Response,
        services: ServiceFactory = Depends(get_services)
):
    """
    Удалить текущего пользователя

    Требуемые права доступа: CAN_DELETE_SELF, CAN_LOGOUT

    Состояние: ACTIVE
    """
    await services.user.delete_me(password)
    await services.auth.logout(request, response)


@router.get("/session/list", response_model=SessionsResponse, status_code=http_status.HTTP_200_OK)
async def get_self_sessions(services: ServiceFactory = Depends(get_services)):
    """
    Получить список сессий текущего пользователя

    Требуемые права доступа: CAN_GET_SELF_SESSIONS

    Состояние: ACTIVE
    """
    return SessionsResponse(content=await services.user.get_my_sessions())


@router.get("/session/list/{user_id}", response_model=SessionsResponse, status_code=http_status.HTTP_200_OK)
async def get_user_sessions(user_id: uuid.UUID, services: ServiceFactory = Depends(get_services)):
    """
    Получить список сессий пользователя по id

    Требуемые права доступа: CAN_GET_USER_SESSIONS

    Состояние: ACTIVE
    """
    return SessionsResponse(content=await services.user.get_user_sessions(user_id))


@router.delete("/session", response_model=None, status_code=http_status.HTTP_204_NO_CONTENT)
async def delete_current_session(session_id: str, services: ServiceFactory = Depends(get_services)):
    """
    Удалить свою сессию по id

    Требуемые права доступа: CAN_DELETE_SELF_SESSION

    Состояние: ACTIVE
    """
    await services.user.delete_my_session(session_id)


@router.delete("/session/{user_id}", response_model=None, status_code=http_status.HTTP_204_NO_CONTENT)
async def delete_user_session(user_id: uuid.UUID, session_id: str, services: ServiceFactory = Depends(get_services)):
    """
    Удалить сессию пользователя по id

    Требуемые права доступа: CAN_DELETE_USER_SESSION

    Состояние: ACTIVE
    """
    await services.user.delete_user_session(user_id, session_id)
