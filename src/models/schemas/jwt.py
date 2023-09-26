from pydantic import BaseModel


class Tokens(BaseModel):
    access_token: str | None
    refresh_token: str | None


class TokenPayload(BaseModel):
    id: str
    username: str
    permissions: list[str]
    state_id: int
    exp: int
