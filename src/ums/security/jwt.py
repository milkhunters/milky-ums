from datetime import datetime, UTC, timedelta
from typing import Literal

from jose import JWTError, jwt

from ums.models.schemas import UserID, UserState
from ums.security.models import TokenPayload


class JWTValidationError(Exception):
    pass


Algorithm = Literal[
    "ES256"
]

TokenType = Literal["access", "refresh"]


class JwtTokenProcessor:
    def __init__(
            self,
            private_key: str,
            public_key: str,
            access_expires: timedelta,
            refresh_expires: timedelta,
            algorithm: Algorithm,
    ):
        self.private_key = private_key
        self.public_key = public_key
        self.access_expires = access_expires
        self.refresh_expires = refresh_expires
        self.algorithm = algorithm

    def create_token(
            self,
            user_id: UserID,
            username: str,
            permissions: list[str],
            state: UserState,
            token_type: TokenType,
    ) -> str:
        expiration = self.access_expires if token_type == "access" else self.refresh_expires
        to_encode = {
            "id": str(user_id),
            "username": username,
            "permissions": permissions,
            "state": state.value,
            "exp": datetime.now(UTC) + expiration
        }
        return jwt.encode(
            to_encode, self.private_key, algorithm=self.algorithm,
        )

    def validate_token(self, token: str) -> TokenPayload:
        try:
            return TokenPayload(**jwt.decode(
                token, self.public_key, algorithms=[self.algorithm],
            ))
        except (JWTError, ValueError, AttributeError):
            raise JWTValidationError

    def is_valid_token(self, token: str | None) -> bool:
        try:
            self.validate_token(token)
        except JWTValidationError:
            return False
        return True
