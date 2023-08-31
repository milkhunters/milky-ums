import asyncio
import uuid
from datetime import datetime

from src import exceptions
from src.config import Config
from src.services import SessionManager
from src.utils import EmailSender, RedisClient
from src.services.repository import UserRepo, RoleRepo
from src.services.auth.filters import access_filter, state_filter
from src.services.auth.password import verify_password, get_hashed_password

from src.models import schemas
from src.models.auth import BaseUser
from src.models.state import UserState
from src.models.access import AccessTags
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
            config: Config
    ):
        self._current_user = current_user
        self._repo = user_repo
        self._role_repo = role_repo
        self._email = email
        self._redis_client_reauth = redis_client_reauth
        self._session = session
        self._config = config

    @access_filter(AccessTags.CAN_GET_SELF)
    @state_filter(UserState.ACTIVE)
    async def get_me(self) -> schemas.UserMedium:
        user = await self._repo.get(id=self._current_user.id, as_full=True)
        access_list = [access.title for access in user.role.access]
        user_model = schemas.User.model_validate(user)
        role_model = schemas.RoleMedium(id=user.role.id, title=user.role.title, access=access_list)
        return schemas.UserMedium(**user_model.model_dump(exclude={"role"}), role=role_model)

    @access_filter(AccessTags.CAN_GET_USER)
    async def get_user(self, user_id: uuid.UUID) -> schemas.UserSmall:
        user = await self._repo.get(id=user_id)
        if not user:
            raise exceptions.NotFound(f"Пользователь с id:{user_id} не найден!")
        return schemas.UserSmall.model_validate(user)

    @access_filter(AccessTags.CAN_UPDATE_SELF)
    @state_filter(UserState.ACTIVE)
    async def update_me(self, data: schemas.UserUpdate) -> None:
        await self._repo.update(
            id=self._current_user.id,
            **data.model_dump(exclude_unset=True)
        )

    @access_filter(AccessTags.CAN_UPDATE_USER)
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

    @access_filter(AccessTags.CAN_UPDATE_SELF)
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
        await self._email.send_mail(
            to=user.email,
            subject="Пароль MilkHunters изменен",
            content=f"""
                Здравствуйте, <b>{user.username}!</b><br><br>
                Пароль от вашего аккаунта MilkHunters был успешно изменен сегодня {change_time} 
                (ip:{self._current_user.ip}).<br><br>
                Это оповещение отправлено в целях обеспечения конфиденциальности и безопасности 
                вашего аккаунта MilkHunters. <b>Если изменение пароля запросили вы, 
                то дальнейших действий не потребуется.</b><br>
                <b>Если это сделали не вы</b>, измените пароль от своего аккаунта MilkHunters. 
                Также рекомендуем изменить пароль от этой эл. почты, 
                чтобы обеспечить максимальную защиту аккаунта. <br>
                Если вы не можете получить доступ к своему аккаунту, пройдите по этой 
                <a href='https://milkhunters.ru/password_reset?email={user.email}.'>ссылке</a>, 
                чтобы восстановить доступ к аккаунту.<br><br>
                С любовью, команда MilkHunters.
            """
        )

    @access_filter(AccessTags.CAN_DELETE_SELF)
    @state_filter(UserState.ACTIVE)
    async def delete_me(self, password: str) -> None:
        user = await self._repo.get(id=self._current_user.id)
        if not verify_password(password, user.hashed_password):
            raise exceptions.BadRequest("Неверный пароль!")

        await self._repo.update(
            id=self._current_user.id,
            state=UserState.DELETED
        )
