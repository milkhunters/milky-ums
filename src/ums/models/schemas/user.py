from uuid import UUID
from enum import Enum
from typing import NewType

from pydantic import BaseModel, field_validator, EmailStr
from datetime import datetime

from .role import RoleMedium, RoleSmall, Role
from ums import validators


class UserState(Enum):
    NOT_CONFIRMED = 0
    ACTIVE = 1
    BLOCKED = 2
    DELETED = 3


UserID = NewType('TaskID', UUID)


class User(BaseModel):
    """
    Модель пользователя

    """
    id: UserID
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
    id: UserID
    username: str
    email: EmailStr
    first_name: str | None
    last_name: str | None
    role: RoleMedium
    state: UserState

    created_at: datetime


class UserSmall(BaseModel):
    id: UserID
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
    role_id: UUID = None
    state: UserState = None
