from typing import AsyncGenerator, Callable

from src.config import Config
from src.models.auth import BaseUser
from src.services.auth import AuthApplicationService, JWTManager, SessionManager, ConfirmCodeUtil
from src.services.repository import RepoFactory
from src.services.role import RoleApplicationService
from src.services.stats import StatsApplicationService
from src.services.user import UserApplicationService
from src.utils import EmailSender, S3Storage


class ServiceFactory:
    def __init__(
            self,
            repo_factory: RepoFactory,
            *,
            current_user: BaseUser,
            config: Config,
            redis_sessions,
            redis_reauth,
            redis_confirmations,
            email_sender: EmailSender,
            file_storage: S3Storage,
            lazy_session: Callable[[], AsyncGenerator],
    ):
        self._repo = repo_factory
        self._current_user = current_user
        self._config = config
        self._redis_sessions = redis_sessions
        self._redis_reauth = redis_reauth
        self._redis_confirmations = redis_confirmations
        self._email_sender = email_sender
        self._file_storage = file_storage
        self._lazy_session = lazy_session

    @property
    def auth(self) -> AuthApplicationService:
        return AuthApplicationService(
            self._current_user,
            jwt_manager=JWTManager(config=self._config),
            session_manager=SessionManager(redis_client=self._redis_sessions, config=self._config),
            user_repo=self._repo.user,
            redis_client=self._redis_sessions,
            redis_client_reauth=self._redis_reauth,
            confirm_code_util=ConfirmCodeUtil(redis=self._redis_confirmations),
            email=self._email_sender
        )

    @property
    def user(self) -> UserApplicationService:
        return UserApplicationService(
            self._current_user,
            user_repo=self._repo.user,
            role_repo=self._repo.role,
            email=self._email_sender,
            redis_client_reauth=self._redis_reauth,
            session=SessionManager(redis_client=self._redis_sessions, config=self._config),
            config=self._config,
            s3_storage=self._file_storage,
        )

    @property
    def stats(self) -> StatsApplicationService:
        return StatsApplicationService(redis_client=self._redis_sessions, config=self._config)

    @property
    def role(self) -> RoleApplicationService:
        return RoleApplicationService(
            self._current_user,
            role_repo=self._repo.role,
            permission_repo=self._repo.permission,
            role_permission_repo=self._repo.role_permission
        )
