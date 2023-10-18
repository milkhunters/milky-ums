from pydantic import BaseModel


class Session(BaseModel):
    id: str
    ip: str
    time: int
    user_agent: str
