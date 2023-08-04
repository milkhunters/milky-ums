from fastapi import APIRouter, Depends
from fastapi.requests import Request
from fastapi.responses import Response
from fastapi import status as http_status

from src.views import UserResponse
from src.dependencies.services import get_services
from src.models import schemas
from src.services import ServiceFactory

router = APIRouter()


@router.post("/signUp", response_model=None, status_code=http_status.HTTP_201_CREATED)
async def sign_up(data: schemas.UserCreate, services: ServiceFactory = Depends(get_services)):
    """
    Регистрация нового пользователя

    Роль: GUEST.ONE
    """
    await services.auth.create_user(data)


@router.post("/signIn", response_model=UserResponse, status_code=http_status.HTTP_200_OK)
async def sign_in(user: schemas.UserAuth, response: Response, services: ServiceFactory = Depends(get_services)):
    """
    Вход в систему

    Роль: GUEST.ONE
    """
    return UserResponse(content=await services.auth.authenticate(user, response))


@router.post('/logout', response_model=None, status_code=http_status.HTTP_204_NO_CONTENT)
async def logout(request: Request, response: Response, services: ServiceFactory = Depends(get_services)):
    """
    Выход из системы

    Роль: Все кроме GUEST.ONE
    """
    await services.auth.logout(request, response)


@router.post('/refresh_tokens', response_model=None, status_code=http_status.HTTP_204_NO_CONTENT)
async def refresh(request: Request, response: Response, services: ServiceFactory = Depends(get_services)):
    """
    Обновить токены jwt

    Роль: Все кроме GUEST.ONE
    Состояние: ACTIVE
    """
    await services.auth.refresh_tokens(request, response)


@router.post("/send/{email}", response_model=None, status_code=http_status.HTTP_204_NO_CONTENT)
async def send_email(email: str, services: ServiceFactory = Depends(get_services)):
    """
    Отправить письмо с кодом для подтверждения email

    Роль: GUEST.ONE
    """
    await services.auth.send_verify_code(email)


@router.post("/confirm/{email}", response_model=None, status_code=http_status.HTTP_204_NO_CONTENT)
async def confirm_email(email: str, code: int, services: ServiceFactory = Depends(get_services)):
    """
    Подтвердить email

    Роль: GUEST.ONE
    """
    await services.auth.verify_email(email, code)


@router.post("/reset/{email}", response_model=None, status_code=http_status.HTTP_204_NO_CONTENT)
async def reset_password(email: str, services: ServiceFactory = Depends(get_services)):
    """
    Отправить письмо с кодом для сброса пароля

    Роль: GUEST.ONE
    """
    await services.auth.reset_password(email)


@router.post("/reset/{email}/{code}", response_model=None, status_code=http_status.HTTP_204_NO_CONTENT)
async def confirm_reset_password(email: str, code: int, password: str, services: ServiceFactory = Depends(get_services)):
    """
    Сбросить пароль

    Роль: GUEST.ONE
    """
    await services.auth.confirm_reset_password(email, code, password)