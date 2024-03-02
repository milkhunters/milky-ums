from .base import BaseView
from ums.models import schemas


class SessionsResponse(BaseView):
    content: list[schemas.Session]
