from .base import BaseView
from src.models import schemas


class RoleResponse(BaseView):
    content: schemas.Role


class RolesResponse(BaseView):
    content: list[schemas.Role]


class AccessResponse(BaseView):
    content: schemas.Access


class AccessesResponse(BaseView):
    content: list[schemas.Access]
