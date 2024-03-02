from pydantic import BaseModel


class TokenPayload(BaseModel):
    id: str
    username: str
    permissions: list[str]
    state: str
    exp: int


class JWTTokens(BaseModel):
    access_token: str | None
    refresh_token: str | None
