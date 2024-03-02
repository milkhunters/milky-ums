from .base import BaseView
from ums.models import schemas


class S3UploadResponse(BaseView):
    content: schemas.PreSignedPostUrl
