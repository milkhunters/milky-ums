from uuid import UUID
from datetime import datetime
from typing import NewType

from pydantic import BaseModel

RoleID = NewType("RoleID", int)
PermissionID = NewType("PermissionID", UUID)


class Permission(BaseModel):
    id: PermissionID
    title: str
    created_at: datetime
    updated_at: datetime | None

    class Config:
        from_attributes = True


class Role(BaseModel):
    id: RoleID
    title: str
    permissions: list[Permission] | None
    created_at: datetime
    updated_at: datetime | None

    class Config:
        from_attributes = True


class RoleMedium(BaseModel):
    id: RoleID
    title: str
    permissions: list[str] | None


class RoleSmall(BaseModel):
    id: RoleID
    title: str

    class Config:
        from_attributes = True


class UpdateRole(BaseModel):
    title: str


class CreateRole(BaseModel):
    title: str


class CreatePermission(BaseModel):
    title: str
