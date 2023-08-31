from .base import BaseView
from src.models import schemas


class SessionsResponse(BaseView):
    content: list[schemas.Session]
