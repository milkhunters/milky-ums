from functools import wraps

from ums.exceptions import AccessDenied
from ums.roles.permission import Permission, PermissionOrSet
from ums.models.schemas import UserState


def permission_filter(*tags: Permission | PermissionOrSet):
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

            access_error = AccessDenied('У Вас нет прав для выполнения этого действия')

            # Проверка одиночных "and" тегов
            if not {tag.value for tag in tags if isinstance(tag, Permission)}.issubset(current_user.permissions):
                raise access_error

            # Проверка "or" тегов (PermissionOrSet)
            for perm_set in tags:
                if isinstance(perm_set, PermissionOrSet):
                    if not perm_set.permissions.intersection(current_user.permissions):
                        raise access_error

            return await func(*args, **kwargs)

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
                raise ValueError('AuthMiddleware не установлен')

            if current_user.state in states:
                return await func(*args, **kwargs)

            raise AccessDenied('У Вас нет прав для выполнения этого действия')

        return wrapper

    return decorator
