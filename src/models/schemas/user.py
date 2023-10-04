import uuid

from pydantic import BaseModel, field_validator, EmailStr
from datetime import datetime

from .role import RoleMedium, RoleSmall, Role
from src.models.state import UserState
from src.utils import validators


class User(BaseModel):
    """
    Модель пользователя

    """
    id: uuid.UUID
    username: str
    email: EmailStr
    first_name: str | None
    last_name: str | None
    role: Role
    state: UserState

    created_at: datetime
    updated_at: datetime | None

    class Config:
        from_attributes = True


class UserAvatar(BaseModel):
    avatar_url: str | None


class UserMedium(BaseModel):
    """
    Модель пользователя

    """
    id: uuid.UUID
    username: str
    email: EmailStr
    first_name: str | None
    last_name: str | None
    role: RoleMedium
    state: UserState

    created_at: datetime


class UserSmall(BaseModel):
    id: uuid.UUID
    username: str
    first_name: str | None
    last_name: str | None
    role: RoleSmall
    state: UserState

    created_at: datetime

    class Config:
        from_attributes = True


class UserCreate(BaseModel):
    username: str
    email: EmailStr
    password: str
    first_name: str = None
    last_name: str = None

    @field_validator('username')
    def username_must_be_valid(cls, value):
        if not validators.is_valid_username(value):
            raise ValueError("Имя пользователя должно быть валидным")
        return value

    @field_validator('password')
    def password_must_be_valid(cls, value):
        if not validators.is_valid_password(value):
            raise ValueError("Пароль должен быть валидным")
        return value

    @field_validator('first_name')
    def first_name_must_be_valid(cls, value):
        if value and not validators.is_valid_first_name(value):
            raise ValueError("Имя должно быть валидным")
        return value

    @field_validator('last_name')
    def last_name_must_be_valid(cls, value):
        if value and not validators.is_valid_last_name(value):
            raise ValueError("Фамилия должна быть валидной")
        return value


class UserAuth(BaseModel):
    username: str
    password: str

    @field_validator('username')
    def username_must_be_valid(cls, value):
        if not validators.is_valid_username(value):
            raise ValueError("Имя пользователя должно быть валидным")
        return value

    @field_validator('password')
    def password_must_be_valid(cls, value):
        if not validators.is_valid_password(value):
            raise ValueError("Пароль должен быть валидным")
        return value


class UserUpdate(BaseModel):
    username: str = None
    first_name: str = None
    last_name: str = None

    @field_validator('username')
    def username_must_be_valid(cls, value):
        if value and not validators.is_valid_username(value):
            raise ValueError("Имя пользователя должно быть валидным")
        return value

    @field_validator('first_name')
    def first_name_must_be_valid(cls, value):
        if value and not validators.is_valid_first_name(value):
            raise ValueError("Имя должно быть валидным")
        return value

    @field_validator('last_name')
    def last_name_must_be_valid(cls, value):
        if value and not validators.is_valid_last_name(value):
            raise ValueError("Фамилия должна быть валидной")
        return value


class UserUpdateByAdmin(UserUpdate):
    email: EmailStr = None
    role_id: uuid.UUID = None
    state: UserState = None
