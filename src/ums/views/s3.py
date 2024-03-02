from .base import BaseView
from ..models import schemas


class S3UploadResponse(BaseView):
    content: schemas.PreSignedPostUrl
