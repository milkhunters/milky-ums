from enum import Enum
from typing import Any

from pydantic import BaseModel


class Error(BaseModel):
    type: ErrorType
    content: Any


class FieldErrorItem(BaseModel):
    field: Any
    location: list[Any]
    message: str
    type: str


class ErrorType(Enum):
    MESSAGE = 1
    FIELD_LIST = 2
