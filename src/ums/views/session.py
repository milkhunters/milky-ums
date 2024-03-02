from .base import BaseView
from ..models import schemas


class SessionsResponse(BaseView):
    content: list[schemas.Session]
