from .base import BaseView
from ..models import schemas


class UserResponse(BaseView):
    content: schemas.UserMedium


class UserSmallResponse(BaseView):
    content: schemas.UserSmall


class UserAvatarResponse(BaseView):
    content: schemas.UserAvatar