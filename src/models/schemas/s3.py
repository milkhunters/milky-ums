from pydantic import BaseModel


class PreSignedPostUrl(BaseModel):
    url: str
    fields: dict

    class Config:
        from_attributes = True
