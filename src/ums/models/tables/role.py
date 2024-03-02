import uuid

from sqlalchemy import Column, UUID, VARCHAR, DateTime, func, ForeignKey, Integer
from sqlalchemy.orm import relationship


from ums.db import Base


class Role(Base):
    """
    The Role model
    """
    __tablename__ = "roles"

    id = Column(Integer, primary_key=True, default=uuid.uuid4)
    title = Column(VARCHAR(32), unique=True, nullable=False)

    permissions = relationship(
        'Permission', secondary='role_permission', back_populates='roles'
    )
    users = relationship('User', back_populates='role')

    created_at = Column(DateTime(timezone=True), server_default=func.now())
    updated_at = Column(DateTime(timezone=True), onupdate=func.now())

    def __repr__(self):
        return f'<{self.__class__.__name__}: {self.id}>'


class RolePermission(Base):
    """
    Many-to-many table for Role and Permission
    """
    __tablename__ = "role_permission"

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    role_id = Column(Integer, ForeignKey("roles.id", ondelete="cascade"), nullable=False)
    permission_id = Column(UUID(as_uuid=True), ForeignKey("permissions.id", ondelete="cascade"), nullable=False)

    def __repr__(self):
        return f'<{self.__class__.__name__}: {self.id}>'
