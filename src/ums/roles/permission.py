from enum import Enum


class PermissionOrSet:
    def __init__(self, *permissions: "Permission"):
        self.permissions = set(permissions)

    def __or__(self, other: "PermissionOrSet"):
        return PermissionOrSet(*self.permissions, *other.permissions)

    def __iter__(self):
        return iter(self.permissions)

    def __repr__(self):
        return f"{self.__class__.__name__}({', '.join(repr(p) for p in self.permissions)})"


class Permission(Enum):
    AUTHENTICATE = "AUTHENTICATE"
    VERIFY_EMAIL = "VERIFY_EMAIL"
    RESET_PASSWORD = "RESET_PASSWORD"
    LOGOUT = "LOGOUT"
    REFRESH_TOKENS = "REFRESH_TOKENS"

    GET_USER = "GET_USER"
    GET_USER_FULL = "GET_USER_FULL"
    CREATE_USER = "CREATE_USER"
    UPDATE_USER = "UPDATE_USER"

    GET_SELF = "GET_SELF"
    UPDATE_SELF = "UPDATE_SELF"
    DELETE_SELF = "DELETE_SELF"

    GET_ROLE = "GET_ROLE"
    UPDATE_ROLE = "UPDATE_ROLE"
    DELETE_ROLE = "DELETE_ROLE"
    CREATE_ROLE = "CREATE_ROLE"

    GET_SELF_SESSIONS = "GET_SELF_SESSIONS"
    DELETE_SELF_SESSION = "DELETE_SELF_SESSION"
    GET_USER_SESSIONS = "GET_USER_SESSIONS"
    DELETE_USER_SESSION = "DELETE_USER_SESSION"

    def __or__(self, other):
        if isinstance(other, Permission):
            return PermissionOrSet(self, other)
        elif isinstance(other, PermissionOrSet):
            return PermissionOrSet(self, *other)
        else:
            raise TypeError(f"unsupported operand type(s) for |: 'Permission' and '{type(other).__name__}'")
