from pydantic import BaseModel


class Tokens(BaseModel):
    access_token: str
    refresh_token: str


class TokenPayload(BaseModel):
    id: str
    username: str
    access: list[str]
    state_id: int
    exp: int
