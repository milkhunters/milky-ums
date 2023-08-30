import uuid
from abc import ABC, abstractmethod

from starlette import authentication

from src.models.access import AccessTags
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
    def access(self) -> set[str]:
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
    def __init__(self, id: str, username: str, access: list[AccessTags], state_id: int, exp: int, **kwargs):
        self._id = uuid.UUID(id)
        self._username = username
        self._access = access
        self._state_id = state_id
        self._exp = exp
        self._ip = kwargs.get('ip')
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
    def access(self) -> set[AccessTags]:
        return set(self._access)

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
    def access(self) -> set[str]:
        return {
            AccessTags.CAN_AUTHENTICATE.value,
            AccessTags.CAN_CREATE_USER.value,
            AccessTags.CAN_SEND_VERIFY_CODE.value,
            AccessTags.CAN_VERIFY_EMAIL.value,
            AccessTags.CAN_RESET_PASSWORD.value,
            AccessTags.CAN_CONFIRM_RESET_PASSWORD.value,
            AccessTags.CAN_REFRESH_TOKENS.value,
            AccessTags.CAN_GET_USER.value,
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
