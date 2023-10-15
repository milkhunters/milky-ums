from .base import BaseView
from src.models import schemas


class S3UploadResponse(BaseView):
    content: schemas.PreSignedPostUrl
