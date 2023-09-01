import uuid

from sqlalchemy import Column, UUID, VARCHAR, DateTime, func, ForeignKey
from sqlalchemy.orm import relationship


from src.db import Base


class Role(Base):
    """
    The Role model
    """
    __tablename__ = "roles"

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    title = Column(VARCHAR(32), unique=True, nullable=False)

    access = relationship('models.tables.access.Access', secondary='role_access', back_populates='roles')
    users = relationship('models.tables.user.User', back_populates='role')

    created_at = Column(DateTime(timezone=True), server_default=func.now())
    updated_at = Column(DateTime(timezone=True), onupdate=func.now())

    def __repr__(self):
        return f'<{self.__class__.__name__}: {self.id}>'


class RoleAccess(Base):
    """
    Many-to-many table for Role and Access
    """
    __tablename__ = "role_access"

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    role_id = Column(UUID(as_uuid=True), ForeignKey("roles.id"), nullable=False)
    access_id = Column(UUID(as_uuid=True), ForeignKey("access.id"), nullable=False)

    def __repr__(self):
        return f'<{self.__class__.__name__}: {self.id}>'
