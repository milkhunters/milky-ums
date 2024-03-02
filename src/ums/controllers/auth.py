from typing import Annotated

from fastapi import APIRouter, Depends
from fastapi.requests import Request
from fastapi.responses import Response
from fastapi import status as http_status
from pydantic import EmailStr

from ums.config import JWTConfig
from ums.dependencies.config import get_jwt_config
from ums.views import UserResponse
from ums.dependencies.services import get_services
from ums.models import schemas
from ums.services import ServiceFactory

router = APIRouter()


@router.post("/signUp", response_model=None, status_code=http_status.HTTP_201_CREATED)
async def sign_up(data: schemas.UserCreate, services: ServiceFactory = Depends(get_services)):
    """
    Регистрация нового пользователя

    Требуемые права доступа: CREATE_USER
    """
    await services.auth.create_user(data)


@router.post("/signIn", response_model=UserResponse, status_code=http_status.HTTP_200_OK)
async def sign_in(
        user: schemas.UserAuth,
        response: Response,
        services: Annotated[ServiceFactory, Depends(get_services)],
        config: Annotated[JWTConfig, Depends(get_jwt_config)]
):
    """
    Вход в систему

    Требуемые права доступа: AUTHENTICATE
    """
    content, jwt_tokens, session_id = await services.auth.authenticate(user)
    response.set_cookie(
        key="access_token",
        value=jwt_tokens[0],
        secure=True,
        httponly=True,
        samesite="none",
        max_age=config.ACCESS_EXP_SEC,
        path="/api"
    )
    response.set_cookie(
        key="refresh_token",
        value=jwt_tokens[1],
        secure=True,
        httponly=True,
        samesite="none",
        max_age=config.REFRESH_EXP_SEC,
        path="/api",
    )
    response.set_cookie(
        key="session_id",
        value=session_id,
        secure=True,
        httponly=True,
        samesite="none",
        max_age=config.REFRESH_EXP_SEC,
        path="/api"
    )

    return UserResponse(content=content)


@router.post('/logout', response_model=None, status_code=http_status.HTTP_204_NO_CONTENT)
async def logout(request: Request, response: Response, services: ServiceFactory = Depends(get_services)):
    """
    Выход из системы

    Требуемые права доступа: LOGOUT
    """
    response.set_cookie(
        key="access_token",
        value="",
        secure=True,
        httponly=True,
        samesite="none",
        max_age=1,
        path="/api"
    )
    response.set_cookie(
        key="refresh_token",
        value="",
        secure=True,
        httponly=True,
        samesite="none",
        max_age=1,
        path="/api",
    )
    response.set_cookie(
        key="session_id",
        value="",
        secure=True,
        httponly=True,
        samesite="none",
        max_age=1,
        path="/api"
    )
    session_id = request.cookies.get("session_id")
    await services.auth.logout(session_id)


@router.post('/refresh_tokens', response_model=UserResponse, status_code=http_status.HTTP_200_OK)
async def refresh(
        request: Request,
        response: Response,
        services: Annotated[ServiceFactory, Depends(get_services)],
        config: Annotated[JWTConfig, Depends(get_jwt_config)]
):
    """
    Обновить токены jwt

    Требуемые права доступа: None
    Состояние: ACTIVE
    """
    jwt_tokens = (
        request.cookies.get("access_token"),
        request.cookies.get("refresh_token")
    )
    session_id = request.cookies.get("session_id")

    content, new_jwt_tokens, new_session_id = await services.auth.refresh_tokens(jwt_tokens, session_id)
    response.set_cookie(
        key="access_token",
        value=new_jwt_tokens[0],
        secure=True,
        httponly=True,
        samesite="none",
        max_age=config.ACCESS_EXP_SEC,
        path="/api"
    )
    response.set_cookie(
        key="refresh_token",
        value=new_jwt_tokens[1],
        secure=True,
        httponly=True,
        samesite="none",
        max_age=config.REFRESH_EXP_SEC,
        path="/api",
    )
    response.set_cookie(
        key="session_id",
        value=new_session_id,
        secure=True,
        httponly=True,
        samesite="none",
        max_age=config.REFRESH_EXP_SEC,
        path="/api"
    )
    return UserResponse(content=content)


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
