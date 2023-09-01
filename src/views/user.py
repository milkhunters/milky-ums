from .base import BaseView
from src.models import schemas


class UserResponse(BaseView):
    content: schemas.UserMedium


class UserSmallResponse(BaseView):
    content: schemas.UserSmall
