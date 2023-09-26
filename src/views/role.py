from .base import BaseView
from src.models import schemas


class RoleResponse(BaseView):
    content: schemas.Role


class RolesResponse(BaseView):
    content: list[schemas.Role]


class PermissionResponse(BaseView):
    content: schemas.Permission


class PermissionsResponse(BaseView):
    content: list[schemas.Permission]
