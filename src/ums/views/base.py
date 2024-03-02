from typing import Any

from pydantic import BaseModel

from ums.models.schemas import Error


class BaseView(BaseModel):
    content: Any = None
    error: Error = None
