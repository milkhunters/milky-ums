import uuid

from sqlalchemy import Column, UUID, VARCHAR, DateTime, func
from sqlalchemy.orm import relationship

from src.db import Base


class Permission(Base):
    """
    The Permission model
    """
    __tablename__ = "permissions"

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    title = Column(VARCHAR(64), unique=True, nullable=False)

    roles = relationship("models.tables.role.Role", secondary='role_permission', back_populates='permissions')

    created_at = Column(DateTime(timezone=True), server_default=func.now())
    updated_at = Column(DateTime(timezone=True), onupdate=func.now())

    def __repr__(self):
        return f'<{self.__class__.__name__}: {self.title}>'
