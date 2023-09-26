from functools import wraps

from src.exceptions import AccessDenied, Unauthorized
from src.models.permission import Permission
from src.models.auth import UnauthenticatedUser
from src.models.state import UserState


def permission_filter(*tags: Permission):
    """
    Permission Tag Filter decorator for ApplicationServices
    It is necessary that the class of the method being decorated has a field '_current_user'

    :param tags: tuple of tags
    :return: decorator
    """

    def decorator(func):
        @wraps(func)
        async def wrapper(*args, **kwargs):

            service_class: object = args[0]
            current_user = service_class.__getattribute__('_current_user')
            if not current_user:
                raise ValueError('AuthMiddleware not found')

            if {tag.value for tag in tags}.issubset(current_user.permissions):
                return await func(*args, **kwargs)

            if isinstance(current_user, UnauthenticatedUser):
                raise Unauthorized('Вы не авторизованы')

            raise AccessDenied('У Вас нет прав для выполнения этого действия')

        return wrapper

    return decorator


def state_filter(*states: UserState):
    if not states:
        states = [state for state in UserState]

    def decorator(func):
        @wraps(func)
        async def wrapper(*args, **kwargs):
            service_class: object = args[0]
            current_user = service_class.__getattribute__('_current_user')
            if not current_user:
                raise ValueError('AuthMiddleware not found')

            if current_user.state in states:
                return await func(*args, **kwargs)
            else:
                raise AccessDenied('У Вас нет прав для выполнения этого действия')

        return wrapper

    return decorator
