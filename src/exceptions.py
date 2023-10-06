from fastapi.exceptions import HTTPException as StarletteHTTPException, RequestValidationError
from fastapi.responses import JSONResponse

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
    def __init__(self, message: str = "Доступ запрещен") -> None:
        super().__init__(message=message, status_code=403)


class Unauthorized(APIError):
    def __init__(self, message: str = "Несанкционированный") -> None:
        super().__init__(message=message, status_code=401)


class NotFound(APIError):
    def __init__(self, message: str = "Запрашиваемый контент не найден") -> None:
        super().__init__(message=message, status_code=404)


class AlreadyExists(APIError):
    def __init__(self, message: str = "Уже существует") -> None:
        super().__init__(message=message, status_code=409)


class BadRequest(APIError):
    def __init__(self, message: str = "Неверный запрос") -> None:
        super().__init__(message=message, status_code=400)


class ConflictError(APIError):
    def __init__(self, message: str = "Конфликт") -> None:
        super().__init__(message=message, status_code=409)


async def handle_api_error(request, exc):
    return JSONResponse(
        status_code=exc.status_code,
        content=BaseView(
            error=Error(
                type=ErrorType.MESSAGE,
                content=exc.message
            )
        ).model_dump()
    )


async def handle_pydantic_error(request, exc: RequestValidationError):
    content = []
    for error in exc.errors():
        field = error.get('loc', ['none'])[-1]
        location = error.get('loc', [])
        message = error.get('msg', 'No message')
        error_type = error.get('type', 'empty')

        if error_type == "missing":
            message = "Поле является обязательным"
        elif error_type == "value_error":
            message = ", ".join(error['ctx']['error'].args)

        content.append(
            FieldErrorItem(
                field=field,
                location=location,
                message=message,
                type=error_type
            )
        )

    return JSONResponse(
        status_code=400,
        content=BaseView(
            error=Error(
                type=ErrorType.FIELD_LIST,
                content=content
            )
        ).model_dump()
    )


async def handle_404_error(request, exc):
    if isinstance(exc, NotFound):
        return await handle_api_error(request, exc)

    return JSONResponse(
        status_code=exc.status_code,
        content=BaseView(
            error=Error(
                type=ErrorType.MESSAGE,
                content='Запрашиваемый контент не найден'
            )
        ).model_dump()
    )
