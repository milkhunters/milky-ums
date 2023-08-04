from src.services.auth import AuthApplicationService
from src.services.repository import RepoFactory


class ServiceFactory:
    def __init__(
            self,
            repo_factory: RepoFactory,
            *,
            current_user: BaseUser,
            config,
            redis_client,
            rmq: aio_pika.RobustConnection,
            file_storage: AbstractStorage,
    ):
        self._repo = repo_factory
        self._current_user = current_user
        self._config = config
        self._redis_client = redis_client
        self._rmq = rmq
        self._file_storage = file_storage

    @property
    def auth(self) -> AuthApplicationService:
        return AuthApplicationService(
            self._current_user,
            jwt_manager=auth.JWTManager(config=self._config),
            session_manager=auth.SessionManager(redis_client=self._redis_client, config=self._config),
            user_repo=self._repo.user,
            redis_client=self._redis_client,
            email_service=self.email,
        )