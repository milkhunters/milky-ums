import uuid
from abc import ABC, abstractmethod

from starlette import authentication
from user_agents.parsers import UserAgent

from src.models.permission import Permission
from src.models.state import UserState


class BaseUser(ABC, authentication.BaseUser):

    @property
    @abstractmethod
    def id(self) -> uuid.UUID:
        pass

    @property
    @abstractmethod
    def username(self) -> str:
        pass

    @property
    @abstractmethod
    def permissions(self) -> set[str]:
        pass

    @property
    @abstractmethod
    def state(self) -> UserState | None:
        pass

    @property
    @abstractmethod
    def access_exp(self) -> int:
        pass

    @property
    @abstractmethod
    def ip(self) -> str:
        pass

    @property
    @abstractmethod
    def user_agent(self) -> UserAgent:
        pass

    @property
    @abstractmethod
    def is_valid_access_token(self) -> bool:
        pass

    @property
    @abstractmethod
    def is_valid_refresh_token(self) -> bool:
        pass

    @property
    @abstractmethod
    def is_valid_session(self) -> bool:
        pass

    @abstractmethod
    def __eq__(self, other):
        pass

    def __repr__(self):
        return f"<{self.__class__.__name__}>({self.display_name})"


class AuthenticatedUser(BaseUser):
    def __init__(self, id: str, username: str, permissions: list[Permission], state_id: int, exp: int, **kwargs):
        self._id = uuid.UUID(id)
        self._username = username
        self._permissions = permissions
        self._state_id = state_id
        self._exp = exp
        self._ip = kwargs.get('ip')
        self._user_agent = kwargs.get('user_agent')
        self._is_valid_access_token = kwargs.get('is_valid_access_token')
        self._is_valid_refresh_token = kwargs.get('is_valid_refresh_token')
        self._is_valid_session = kwargs.get('is_valid_session')

    @property
    def is_authenticated(self) -> bool:
        return True

    @property
    def display_name(self) -> str:
        return self.username

    @property
    def identity(self) -> uuid.UUID:
        return self._id

    @property
    def id(self) -> uuid.UUID:
        return self._id

    @property
    def username(self) -> str:
        return self.username

    @property
    def permissions(self) -> set[Permission]:
        return set(self._permissions)

    @property
    def state(self) -> UserState:
        return UserState(self._state_id)

    @property
    def access_exp(self) -> int:
        return self._exp

    @property
    def ip(self) -> str:
        return self._ip

    @property
    def user_agent(self) -> UserAgent:
        return self._user_agent

    @property
    def is_valid_access_token(self) -> bool:
        return self._is_valid_access_token

    @property
    def is_valid_refresh_token(self) -> bool:
        return self._is_valid_refresh_token

    @property
    def is_valid_session(self) -> bool:
        return self._is_valid_session

    def __eq__(self, other):
        return isinstance(other, AuthenticatedUser) and self._id == other.id

    def __hash__(self):
        return hash(self._id)

    def __repr__(self):
        return f"<{self.__class__.__name__}(id={self._id}, username={self._username})>"


class UnauthenticatedUser(BaseUser):
    def __init__(self, exp: int = None, **kwargs):
        self._exp = exp
        self._ip = kwargs.get('ip')
        self._user_agent = kwargs.get('user_agent')
        self._is_valid_access_token = kwargs.get('is_valid_access_token')
        self._is_valid_refresh_token = kwargs.get('is_valid_refresh_token')
        self._is_valid_session = kwargs.get('is_valid_session')

    @property
    def is_authenticated(self) -> bool:
        return False

    @property
    def display_name(self) -> str:
        return "Guest"

    @property
    def identity(self) -> None:
        return None

    @property
    def id(self) -> None:
        return None

    @property
    def username(self) -> None:
        return None

    @property
    def permissions(self) -> set[str]:
        return {
            Permission.AUTHENTICATE.value,
            Permission.CREATE_USER.value,
            Permission.VERIFY_EMAIL.value,
            Permission.RESET_PASSWORD.value,
            Permission.GET_USER.value,
        }

    @property
    def state(self) -> None:
        return None

    @property
    def access_exp(self) -> int | None:
        return self._exp

    @property
    def ip(self) -> str:
        return self._ip

    @property
    def user_agent(self) -> UserAgent:
        return self._user_agent

    @property
    def is_valid_access_token(self) -> bool:
        return self._is_valid_access_token

    @property
    def is_valid_refresh_token(self) -> bool:
        return self._is_valid_refresh_token

    @property
    def is_valid_session(self) -> bool:
        return self._is_valid_session

    def __eq__(self, other):
        return isinstance(other, UnauthenticatedUser)

    def __repr__(self):
        return f"<{self.__class__.__name__}>({self.display_name})"
