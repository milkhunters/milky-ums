import uuid

from sqlalchemy import Column, UUID, VARCHAR, DateTime, func
from sqlalchemy.orm import relationship

from src.db import Base


class Access(Base):
    """
    The Access model
    """
    __tablename__ = "access"

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    title = Column(VARCHAR(64), unique=True, nullable=False)

    roles = relationship("models.tables.role.Role", secondary='role_access', back_populates='access')

    created_at = Column(DateTime(timezone=True), server_default=func.now())
    updated_at = Column(DateTime(timezone=True), onupdate=func.now())

    def __repr__(self):
        return f'<{self.__class__.__name__}: {self.title}>'
