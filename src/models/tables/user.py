import uuid

from sqlalchemy import Column, UUID, VARCHAR, Enum, DateTime, func, ForeignKey
from sqlalchemy.orm import relationship

from src.models.state import UserState

from src.db import Base


class User(Base):
    """
    The User model
    """
    __tablename__ = "users"

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    username = Column(VARCHAR(32), unique=True, nullable=False)
    email = Column(VARCHAR(255), unique=True, nullable=False)
    first_name = Column(VARCHAR(100), nullable=True)
    last_name = Column(VARCHAR(100), nullable=True)

    role_id = Column(UUID(as_uuid=True), ForeignKey("roles.id"), nullable=False)
    role = relationship("models.tables.role.Role", back_populates="users")

    state = Column(Enum(UserState), default=UserState.NOT_CONFIRMED)
    hashed_password = Column(VARCHAR(255))

    created_at = Column(DateTime(timezone=True), server_default=func.now())
    updated_at = Column(DateTime(timezone=True), onupdate=func.now())

    def __repr__(self):
        return f'<{self.__class__.__name__}: {self.id}>'
