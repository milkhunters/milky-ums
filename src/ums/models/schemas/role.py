import uuid
from datetime import datetime

from pydantic import BaseModel


class Permission(BaseModel):
    id: uuid.UUID
    title: str
    created_at: datetime
    updated_at: datetime | None

    class Config:
        from_attributes = True


class Role(BaseModel):
    id: uuid.UUID
    title: str
    permissions: list[Permission] | None
    created_at: datetime
    updated_at: datetime | None

    class Config:
        from_attributes = True


class RoleMedium(BaseModel):
    id: uuid.UUID
    title: str
    permissions: list[str] | None


class RoleSmall(BaseModel):
    id: uuid.UUID
    title: str

    class Config:
        from_attributes = True


class UpdateRole(BaseModel):
    title: str


class CreateRole(BaseModel):
    title: str


class CreatePermission(BaseModel):
    title: str
