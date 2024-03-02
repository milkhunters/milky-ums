from typing import AsyncGenerator, Callable

from ums.config import Config
from ums.models.auth import BaseUser
from ums.security.jwt import JwtTokenProcessor
from ums.security.session import SessionManager
from ums.security.confirm_manager import ConfirmManager
from ums.repositories import RepoFactory

from ums.services.auth import AuthApplicationService
from ums.services.role import RoleApplicationService
from ums.services.stats import StatsApplicationService
from ums.services.user import UserApplicationService

from ums.utils import EmailSender, S3Storage


class ServiceFactory:
    def __init__(
            self,
            repo_factory: RepoFactory,
            *,
            current_user: BaseUser,
            config: Config,
            redis_reauth,
            confirm_manager: ConfirmManager,
            session_manager: SessionManager,
            jwt: JwtTokenProcessor,
            email_sender: EmailSender,
            file_storage: S3Storage,
            lazy_session: Callable[[], AsyncGenerator],
    ):
        self._repo = repo_factory
        self._current_user = current_user
        self._config = config
        self.session_manager = session_manager
        self._redis_reauth = redis_reauth
        self.confirm_manager = confirm_manager
        self.jwt = jwt
        self._email_sender = email_sender
        self._file_storage = file_storage
        self._lazy_session = lazy_session

    @property
    def auth(self) -> AuthApplicationService:
        return AuthApplicationService(
            self._current_user,
            jwt=self.jwt,
            session_manager=self.session_manager,
            user_repo=self._repo.user,
            redis_client_reauth=self._redis_reauth,
            confirm_manager=self.confirm_manager,
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
            session=self.session_manager,
            config=self._config.JWT,
            s3_storage=self._file_storage,
        )

    @property
    def stats(self) -> StatsApplicationService:
        return StatsApplicationService(config=self._config)

    @property
    def role(self) -> RoleApplicationService:
        return RoleApplicationService(
            self._current_user,
            role_repo=self._repo.role,
            permission_repo=self._repo.permission,
            role_permission_repo=self._repo.role_permission
        )
