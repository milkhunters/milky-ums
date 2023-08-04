import uuid

from sqlalchemy import Column, UUID, VARCHAR, INTEGER, Enum, DateTime, func
from sqlalchemy.orm import relationship

from src.models.role import Role, MainRole as M, AdditionalRole as A
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
    role_id = Column(INTEGER(), default=Role(M.USER, A.ONE).value())
    state = Column(Enum(UserState), default=UserState.NOT_CONFIRMED)
    hashed_password = Column(VARCHAR(255))

    articles = relationship("models.tables.article.Article", back_populates="owner")
    comments = relationship("models.tables.comment.Comment", back_populates="owner")
    notifications = relationship("models.tables.notification.Notification", back_populates="owner")
    files = relationship("models.tables.file.File", back_populates="owner")

    created_at = Column(DateTime(timezone=True), server_default=func.now())
    updated_at = Column(DateTime(timezone=True), onupdate=func.now())

    def __repr__(self):
        return f'<{self.__class__.__name__}: {self.id}>'
