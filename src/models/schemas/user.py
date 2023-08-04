import uuid

from pydantic import BaseModel, field_validator
from datetime import datetime

from src.models.role import Role
from src.models.state import UserState
from src.utils import validators


class User(BaseModel):
    """
    Модель пользователя

    """
    id: uuid.UUID
    username: str
    email: str
    first_name: str | None
    last_name: str | None
    role_id: int
    state: UserState

    created_at: datetime
    updated_at: datetime | None

    class Config:
        from_attributes = True


class UserSmall(BaseModel):
    id: uuid.UUID
    username: str
    first_name: str | None
    last_name: str | None
    role_id: int
    state: UserState

    created_at: datetime

    class Config:
        from_attributes = True


class UserCreate(BaseModel):
    username: str
    email: str
    password: str

    @field_validator('username')
    def username_must_be_valid(cls, value):
        if not validators.is_valid_username(value):
            raise ValueError("Имя пользователя должно быть валидным")
        return value

    @field_validator('email')
    def email_must_be_valid(cls, value):
        if not validators.is_valid_email(value):
            raise ValueError("Email должен быть валидным")
        return value

    @field_validator('password')
    def password_must_be_valid(cls, value):
        if not validators.is_valid_password(value):
            raise ValueError("Пароль должен быть валидным")
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
        if value and len(value.strip()) > 100:
            raise ValueError("Имя должно быть валидным")
        return value.strip()

    @field_validator('last_name')
    def last_name_must_be_valid(cls, value):
        if value and len(value.strip()) > 100:
            raise ValueError("Фамилия должна быть валидной")
        return value.strip()


class UserUpdateByAdmin(UserUpdate):
    email: str = None
    role_id: int = None
    state: UserState = None

    @field_validator('email')
    def email_must_be_valid(cls, value):
        if value and not validators.is_valid_email(value):
            raise ValueError("Email должен быть валидным")
        return value

    @field_validator('role_id')
    def role_id_must_be_valid(cls, value):
        if value:
            Role.from_int(value)
        return value
