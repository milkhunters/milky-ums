from .base import BaseView
from src.models import schemas


class UserResponse(BaseView):
    content: schemas.User


class UserSmallResponse(BaseView):
    content: schemas.UserSmall
