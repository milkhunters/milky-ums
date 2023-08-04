from fastapi.exceptions import HTTPException as StarletteHTTPException
from fastapi.responses import JSONResponse
from pydantic import ValidationError

from src.models.error import ErrorType
from src.models.schemas import Error, FieldErrorItem
from src.views import BaseView


class APIError(StarletteHTTPException):
    def __init__(
            self,
            message: str = "Error",
            status_code: int = 400,
            headers: dict = None
    ) -> None:
        self.message = message
        self.status_code = status_code
        super().__init__(status_code=status_code, headers=headers)


class AccessDenied(APIError):
    def __init__(self, message: str = "Access denied") -> None:
        super().__init__(message=message, status_code=403)


class NotFound(APIError):
    def __init__(self, message: str = "Not Found") -> None:
        super().__init__(message=message, status_code=404)


class AlreadyExists(APIError):
    def __init__(self, message: str = "Already exists") -> None:
        super().__init__(message=message, status_code=409)


class BadRequest(APIError):
    def __init__(self, message: str = "Bad Request") -> None:
        super().__init__(message=message, status_code=400)


class ConflictError(APIError):
    def __init__(self, message: str = "Conflict") -> None:
        super().__init__(message=message, status_code=409)


async def handle_api_error(request, exc):
    return JSONResponse(
        status_code=exc.status_code,
        content=BaseView(
            error=Error(
                type=ErrorType.MESSAGE,
                content=exc.message
            )
        ).dict()
    )


async def handle_pydantic_error(request, exc: ValidationError):
    return JSONResponse(
        status_code=400,
        content=BaseView(
            error=Error(
                type=ErrorType.FIELD_LIST,
                content=[
                    FieldErrorItem(
                        field=field.get('loc', ['none'])[-1],
                        location=field.get('loc', []),
                        message=field.get('msg', 'No message'),
                        type=field.get('type', 'empty')
                    ) for field in exc.errors()
                ]
            )
        ).dict()
    )


async def handle_404_error(request, exc):
    if isinstance(exc, NotFound):
        return await handle_api_error(request, exc)

    return JSONResponse(
        status_code=exc.status_code,
        content=BaseView(
            error=Error(
                type=ErrorType.MESSAGE,
                content='Not Found'
            )
        ).dict()
    )