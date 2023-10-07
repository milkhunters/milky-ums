from fastapi import APIRouter, Depends
from fastapi.requests import Request
from fastapi.responses import Response
from fastapi import status as http_status
from pydantic import EmailStr

from src.views import UserResponse
from src.dependencies.services import get_services
from src.models import schemas
from src.services import ServiceFactory

router = APIRouter()


@router.post("/signUp", response_model=None, status_code=http_status.HTTP_201_CREATED)
async def sign_up(data: schemas.UserCreate, services: ServiceFactory = Depends(get_services)):
    """
    Регистрация нового пользователя

    Требуемые права доступа: CREATE_USER
    """
    await services.auth.create_user(data)


@router.post("/signIn", response_model=UserResponse, status_code=http_status.HTTP_200_OK)
async def sign_in(user: schemas.UserAuth, response: Response, services: ServiceFactory = Depends(get_services)):
    """
    Вход в систему

    Требуемые права доступа: AUTHENTICATE
    """
    return UserResponse(content=await services.auth.authenticate(user, response))


@router.post('/logout', response_model=None, status_code=http_status.HTTP_204_NO_CONTENT)
async def logout(request: Request, response: Response, services: ServiceFactory = Depends(get_services)):
    """
    Выход из системы

    Требуемые права доступа: LOGOUT
    """
    await services.auth.logout(request, response)


@router.post('/refresh_tokens', response_model=UserResponse, status_code=http_status.HTTP_200_OK)
async def refresh(request: Request, response: Response, services: ServiceFactory = Depends(get_services)):
    """
    Обновить токены jwt

    Требуемые права доступа: None
    Состояние: ACTIVE
    """
    return UserResponse(content=await services.auth.refresh_tokens(request, response))


@router.post("/send/{email}", response_model=None, status_code=http_status.HTTP_204_NO_CONTENT)
async def send_email(email: str, services: ServiceFactory = Depends(get_services)):
    """
    Отправить письмо с кодом для подтверждения email

    Требуемые права доступа: VERIFY_EMAIL
    """
    await services.auth.send_verify_code(email)


@router.post("/confirm/{email}", response_model=None, status_code=http_status.HTTP_204_NO_CONTENT)
async def confirm_email(email: EmailStr, code: int, services: ServiceFactory = Depends(get_services)):
    """
    Подтвердить email

    Требуемые права доступа: VERIFY_EMAIL
    """
    await services.auth.verify_email(email, code)


@router.post("/reset/{email}", response_model=None, status_code=http_status.HTTP_204_NO_CONTENT)
async def send_reset_code(email: EmailStr, services: ServiceFactory = Depends(get_services)):
    """
    Отправить письмо с кодом для сброса пароля

    Требуемые права доступа: RESET_PASSWORD
    """
    await services.auth.send_reset_code(email)


@router.post("/reset/{email}/{code}", response_model=None, status_code=http_status.HTTP_204_NO_CONTENT)
async def reset_password(
        email: EmailStr,
        code: int,
        password: str,
        services: ServiceFactory = Depends(get_services)
):
    """
    Сбросить пароль

    Требуемые права доступа: RESET_PASSWORD
    """
    await services.auth.reset_password(email, code, password)
