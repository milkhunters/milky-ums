import asyncio
import uuid
from datetime import datetime

from src import exceptions
from src.config import Config
from src.models import schemas
from src.models.auth import BaseUser
from src.models.file_type import FileType
from src.models.permission import Permission
from src.models.state import UserState
from src.services import SessionManager
from src.services.auth.filters import permission_filter, state_filter
from src.services.auth.password import verify_password, get_hashed_password
from src.services.repository import UserRepo, RoleRepo
from src.utils import EmailSender, RedisClient, S3Storage
from src.utils.validators import is_valid_password


class UserApplicationService:

    def __init__(
            self,
            current_user: BaseUser,
            *,
            user_repo: UserRepo,
            role_repo: RoleRepo,
            email: EmailSender,
            redis_client_reauth: RedisClient,
            session: SessionManager,
            config: Config,
            s3_storage: S3Storage,
    ):
        self._current_user = current_user
        self._repo = user_repo
        self._role_repo = role_repo
        self._email = email
        self._redis_client_reauth = redis_client_reauth
        self._session = session
        self._config = config
        self._file_storage = s3_storage

    @permission_filter(Permission.GET_SELF)
    @state_filter(UserState.ACTIVE)
    async def get_me(self) -> schemas.UserMedium:
        user = await self._repo.get(id=self._current_user.id, as_full=True)
        permission_title_list = [permission.title for permission in user.role.permissions]
        user_model = schemas.User.model_validate(user)
        role_model = schemas.RoleMedium(id=user.role.id, title=user.role.title, permissions=permission_title_list)
        return schemas.UserMedium(**user_model.model_dump(exclude={"role"}), role=role_model)

    @permission_filter(Permission.GET_USER)
    async def get_user(self, user_id: uuid.UUID) -> schemas.UserSmall:
        user = await self._repo.get(id=user_id, as_full=True)
        if not user:
            raise exceptions.NotFound(f"Пользователь с id:{user_id} не найден!")
        return schemas.UserSmall.model_validate(user)

    @permission_filter(Permission.UPDATE_SELF)
    @state_filter(UserState.ACTIVE)
    async def update_me(self, data: schemas.UserUpdate) -> None:
        await self._repo.update(
            id=self._current_user.id,
            **data.model_dump(exclude_unset=True)
        )

    @permission_filter(Permission.UPDATE_USER)
    @state_filter(UserState.ACTIVE)
    async def update_user(self, user_id: uuid.UUID, data: schemas.UserUpdateByAdmin) -> None:
        user = await self._repo.get(id=user_id)
        if not user:
            raise exceptions.NotFound(f"Пользователь с id:{user_id} не найден!")

        if data.role_id:
            role = await self._role_repo.get(id=data.role_id)
            if not role:
                raise exceptions.NotFound(f"Роль с id:{data.role_id} не найдена!")

        if data.state or data.role_id:
            session_id_list = await self._session.get_user_sessions(user_id)
            await asyncio.gather(
                self._redis_client_reauth.set(
                    session_id, data["refresh_token"], expire=self._config.JWT.ACCESS_EXPIRE_SECONDS
                ) for session_id, data in session_id_list.items()
            )

        await self._repo.update(
            id=user_id,
            **data.model_dump(exclude_unset=True)
        )

    @permission_filter(Permission.UPDATE_SELF)
    @state_filter(UserState.ACTIVE)
    async def update_password(self, old_password: str, new_password: str) -> None:
        if old_password == new_password:
            raise exceptions.BadRequest("Новый пароль не должен совпадать со старым!")

        user = await self._repo.get(id=self._current_user.id)
        if not verify_password(old_password, user.hashed_password):
            raise exceptions.BadRequest("Неверный пользовательский пароль!")

        if not is_valid_password(new_password):
            raise exceptions.BadRequest("Неверный формат пароля!")

        await self._repo.update(
            id=self._current_user.id,
            hashed_password=get_hashed_password(new_password)
        )

        change_time = datetime.now().strftime("%d.%m.%Y в %H:%M")
        await self._email.send_email_with_template(
            to=user.email,
            subject="Пароль MilkHunters изменен",
            template="successfully_reset_password.html",
            kwargs=dict(
                username=user.username,
                change_time=change_time,
                ip=self._current_user.ip,
                email=user.email,
            ),
            priority=9
        )

    @permission_filter(Permission.DELETE_SELF)
    @state_filter(UserState.ACTIVE)
    async def delete_me(self, password: str) -> None:
        user = await self._repo.get(id=self._current_user.id)
        if not verify_password(password, user.hashed_password):
            raise exceptions.BadRequest("Неверный пароль!")

        await self._repo.update(
            id=self._current_user.id,
            state=UserState.DELETED
        )

    @permission_filter(Permission.GET_SELF_SESSIONS)
    @state_filter(UserState.ACTIVE)
    async def get_my_sessions(self) -> list[schemas.Session]:
        session_id_list = await self._session.get_user_sessions(self._current_user.id)
        return [
            schemas.Session(
                id=session_id,
                ip=data["ip"],
                time=data["time"],
                user_agent=data["user_agent"]
            )
            for session_id, data in session_id_list.items()
        ]

    @permission_filter(Permission.GET_USER_SESSIONS)
    @state_filter(UserState.ACTIVE)
    async def get_user_sessions(self, user_id: uuid.UUID) -> list[schemas.Session]:
        session_id_list = await self._session.get_user_sessions(user_id)
        return [
            schemas.Session(
                id=session_id,
                ip=data["ip"],
                time=data["time"],
                user_agent=data["user_agent"]
            )
            for session_id, data in session_id_list.items()
        ]

    @permission_filter(Permission.DELETE_SELF_SESSION)
    @state_filter(UserState.ACTIVE)
    async def delete_my_session(self, session_id: str) -> None:
        session_data = await self._session.get_data_from_session(str(self._current_user.id), session_id)
        await self._session.delete_session(self._current_user.id, session_id)
        await self._redis_client_reauth.set(
            session_id, session_data["refresh_token"], expire=self._config.JWT.ACCESS_EXPIRE_SECONDS
        )

    @permission_filter(Permission.DELETE_USER_SESSION)
    @state_filter(UserState.ACTIVE)
    async def delete_user_session(self, user_id: uuid.UUID, session_id: str) -> None:
        session_data = await self._session.get_data_from_session(str(user_id), session_id)
        await self._session.delete_session(user_id, session_id)
        await self._redis_client_reauth.set(
            session_id, session_data["refresh_token"], expire=self._config.JWT.ACCESS_EXPIRE_SECONDS
        )

    @permission_filter(Permission.UPDATE_SELF)
    @state_filter(UserState.ACTIVE)
    async def update_avatar(self, file_type: FileType) -> schemas.PreSignedPostUrl:
        resp = await self._file_storage.generate_upload_url(
            file_id=self._current_user.id,
            content_type=file_type.value,
            content_length=(1, 20 * 1024 * 1024),  # 20mb
            expires_in=30 * 60  # 30 min
        )
        return schemas.PreSignedPostUrl.model_validate(resp)

    @permission_filter(Permission.UPDATE_USER)
    @state_filter(UserState.ACTIVE)
    async def update_user_avatar(self, user_id: uuid.UUID, file_type: FileType) -> schemas.PreSignedPostUrl:
        if not await self._repo.get(id=user_id):
            raise exceptions.NotFound(f"Пользователь с id:{user_id} не найден!")

        url = await self._file_storage.generate_upload_url(
            file_id=user_id,
            content_type=file_type.value,
            content_length=(1, 20 * 1024 * 1024),  # 20mb
            expires_in=30 * 60  # 30 min
        )
        return schemas.PreSignedPostUrl.model_validate(url)

    @permission_filter(Permission.GET_USER)
    async def get_user_avatar_url(self, user_id: uuid.UUID) -> schemas.UserAvatar:

        if (info := await self._file_storage.info(file_id=user_id)) is None:
            raise exceptions.NotFound(f"Аватар пользователя с id:{user_id} не найден!")

        return schemas.UserAvatar(avatar_url=self._file_storage.generate_download_public_url(
            file_id=user_id,
            content_type=info.content_type,
            rcd="inline"
        ))

    @permission_filter(Permission.GET_SELF)
    @state_filter(UserState.ACTIVE)
    async def get_self_avatar_url(self) -> schemas.UserAvatar:
        await self._repo.session.close()

        if (info := await self._file_storage.info(file_id=self._current_user.id)) is None:
            raise exceptions.NotFound(f"Аватар пользователя с id:{self._current_user.id} не найден!")

        return schemas.UserAvatar(avatar_url=self._file_storage.generate_download_public_url(
            file_id=self._current_user.id,
            content_type=info.content_type,
            rcd="inline"
        ))
