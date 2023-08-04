from functools import wraps

from src.exceptions import AccessDenied
from src.models.role import Role, RoleRange
from src.models.state import UserState


def role_filter(*roles: Role | RoleRange, exclude: list[Role | RoleRange] = None, min_role: Role = None):
    """
    Role Filter decorator for ApplicationServices
    It is necessary that the class of the method being decorated has a field '_current_user'

    :param roles: user roles
    :param exclude: exclude roles
    :param min_role: minimum role
    :return: decorator
    """

    if not roles:
        roles = [RoleRange("*")]

    if exclude is None:
        exclude = []

    if min_role is None:
        min_role = 0

    def decorator(func):
        @wraps(func)
        async def wrapper(*args, **kwargs):

            service_class: object = args[0]
            current_user = service_class.__getattribute__('_current_user')
            if not current_user:
                raise ValueError('AuthMiddleware not found')

            if current_user.role in roles and current_user.role not in exclude and current_user.role >= min_role:
                return await func(*args, **kwargs)
            else:
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