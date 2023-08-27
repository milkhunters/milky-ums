from src.models.auth import BaseUser
from src.services.auth import AuthApplicationService, JWTManager, SessionManager
from src.services.repository import RepoFactory
from src.services.role import RoleApplicationService
from src.services.stats import StatsApplicationService
from src.services.user import UserApplicationService
from src.utils import EmailSender


class ServiceFactory:
    def __init__(
            self,
            repo_factory: RepoFactory,
            *,
            current_user: BaseUser,
            config,
            redis_client,
            redis_client_for_kick_list,
            email_sender: EmailSender,
    ):
        self._repo = repo_factory
        self._current_user = current_user
        self._config = config
        self._redis_client = redis_client
        self._redis_client_for_kick_list = redis_client_for_kick_list
        self._email_sender = email_sender

    @property
    def auth(self) -> AuthApplicationService:
        return AuthApplicationService(
            self._current_user,
            jwt_manager=JWTManager(config=self._config),
            session_manager=SessionManager(redis_client=self._redis_client, config=self._config),
            user_repo=self._repo.user,
            redis_client=self._redis_client,
            email=self._email_sender
        )

    @property
    def user(self) -> UserApplicationService:
        return UserApplicationService(
            self._current_user,
            user_repo=self._repo.user,
            email=self._email_sender,
            redis_client=self._redis_client_for_kick_list,
            config=self._config
        )

    @property
    def stats(self) -> StatsApplicationService:
        return StatsApplicationService(redis_client=self._redis_client, config=self._config)

    @property
    def role(self) -> RoleApplicationService:
        return RoleApplicationService(
            self._current_user,
            role_repo=self._repo.role,
            access_repo=self._repo.access,
            role_access_repo=self._repo.role_access
        )
