from datetime import datetime

from ums import exceptions
from ums.models import schemas, tables
from ums.models.auth import BaseUser
from ums.models.schemas import UserMedium
from ums.repositories import UserRepo
from ums.roles.permission import Permission
from ums.utils import RedisClient, EmailSender

from ums.security.confirm_manager import (
    ConfirmManager,
    ManyGenAttemptsError,
    ManyConfirmAttemptsError,
    AlreadyGenError,
    NotGenError,
    InvalidCodeError,
    ExpiredCodeError
)

from ums.security.filters import permission_filter
from ums.security.jwt import JwtTokenProcessor
from ums.security.utils import verify_password, get_hashed_password
from ums.security.session import SessionManager


class AuthApplicationService:
    def __init__(
            self,
            current_user: BaseUser,
            *,
            jwt: JwtTokenProcessor,
            session_manager: SessionManager,
            user_repo: UserRepo,
            redis_client_reauth: RedisClient,
            confirm_manager: ConfirmManager,
            email: EmailSender,
    ):
        self._current_user = current_user
        self.jwt = jwt
        self.session_manager = session_manager
        self.user_repo = user_repo
        self.redis_client_reauth = redis_client_reauth
        self.confirm_manager = confirm_manager
        self.email = email

    @permission_filter(Permission.CREATE_USER)
    async def create_user(self, user: schemas.UserCreate) -> None:
        """
        Создание нового пользователя

        :param user: UserCreate

        :raise AccessDenied if user is already logged in
        :raise AlreadyExists Conflict if user already exists

        :return: User
        """

        if await self.user_repo.get_by_username_insensitive(user.username):
            raise exceptions.AlreadyExists(f"Пользователь {user.username!r} уже существует")

        if await self.user_repo.get_by_email_insensitive(user.email):
            raise exceptions.AlreadyExists(f"Пользователь с email {user.email!r} уже существует")

        hashed_password = get_hashed_password(user.password)
        await self.user_repo.create(
            **user.model_dump(exclude={"password"}),
            role_id=0,
            hashed_password=hashed_password
        )

    @permission_filter(Permission.AUTHENTICATE)
    async def authenticate(self, data: schemas.UserAuth) -> tuple[UserMedium, tuple[str, str], str]:
        """
        Аутентификация пользователя

        :param data: UserAuth

        :return: User

        :raise AlreadyExists: if user is already logged in
        :raise NotFound: if user not found
        :raise AccessDenied: if user is banned
        """

        user: tables.User = await self.user_repo.get_by_username_insensitive(username=data.username, as_full=True)
        if not user:
            raise exceptions.NotFound("Пользователь не найден")
        if not verify_password(data.password, user.hashed_password):
            raise exceptions.NotFound("Неверная пара логин/пароль")
        if user.state == schemas.UserState.BLOCKED:
            raise exceptions.AccessDenied("Пользователь заблокирован")
        if user.state == schemas.UserState.NOT_CONFIRMED:
            raise exceptions.AccessDenied("Пользователь не подтвержден")
        if user.state == schemas.UserState.DELETED:
            raise exceptions.AccessDenied("Пользователь удален")

        # Генерация и установка токенов
        permission_title_list = [obj.title for obj in user.role.permissions]
        tokens = (
            self.jwt.create_token(user.id, user.username, permission_title_list, user.state, "access"),
            self.jwt.create_token(user.id, user.username, permission_title_list, user.state, "refresh")
        )
        session_id = await self.session_manager.set_session_id(
            refresh_token=tokens[1],
            user_id=user.id,
            ip_address=self._current_user.ip,
            user_agent=str(self._current_user.user_agent)
        )
        user_model = schemas.User.model_validate(user)
        role_model = schemas.RoleMedium(id=user.role.id, title=user.role.title, permissions=permission_title_list)
        return schemas.UserMedium(**user_model.model_dump(exclude={"role"}), role=role_model), tokens, session_id

    @permission_filter(Permission.VERIFY_EMAIL)
    async def send_verify_code(self, email: str) -> None:
        """
        Отправка кода подтверждения на почту

        :param email:

        :raise NotFound: if user not found
        :raise AccessDenied: if user already verified
        :raise BadRequest: if code already sent
        """

        user = await self.user_repo.get(email=email)
        if not user:
            raise exceptions.NotFound("Пользователь не найден")

        if user.state != schemas.UserState.NOT_CONFIRMED:
            raise exceptions.AccessDenied("Пользователь уже подтвержден")

        key_lifetime = 60 * 30
        self.confirm_manager.set_key(f'email_confirm:{email}')
        self.confirm_manager.set_key_lifetime(key_lifetime)
        self.confirm_manager.set_gen_interval(120)
        self.confirm_manager.set_max_gen_attempts(3)

        try:
            code = await self.confirm_manager.generate()
        except ManyGenAttemptsError:
            raise exceptions.BadRequest("Превышено количество попыток, попробуйте позже")
        except AlreadyGenError:
            raise exceptions.BadRequest("Код уже отправлен")

        await self.email.send_email_with_template(
            to=email,
            subject="Подтверждение почты",
            template="confirm_email.html",
            kwargs=dict(code=code),
            priority=13,
            ttl=key_lifetime,
        )

    @permission_filter(Permission.VERIFY_EMAIL)
    async def verify_email(self, email: str, code: int) -> None:
        """
        Подтверждение почты

        :param email:
        :param code:

        """

        user = await self.user_repo.get(email=email)
        if not user:
            raise exceptions.NotFound("Пользователь не найден")

        if user.state != schemas.UserState.NOT_CONFIRMED:
            raise exceptions.AccessDenied("Пользователь уже подтвержден")

        key_lifetime = 60 * 30
        self.confirm_manager.set_key(f'email_confirm:{email}')
        self.confirm_manager.set_max_verify_attempts(3)
        self.confirm_manager.set_code_valid_time(key_lifetime)

        try:
            await self.confirm_manager.verify(code)
        except ManyConfirmAttemptsError:
            raise exceptions.BadRequest("Превышено количество попыток")
        except NotGenError:
            raise exceptions.BadRequest("Код не отправлен")
        except InvalidCodeError:
            raise exceptions.BadRequest("Неверный код")
        except ExpiredCodeError:
            raise exceptions.BadRequest("Код устарел, отправьте новый")

        await self.user_repo.update(user.id, state=schemas.UserState.ACTIVE)

        await self.email.send_email_with_template(
            to=email,
            subject="Подтверждение почты",
            template="successfully_confirm_email.html",
            kwargs=dict(
                username=user.username
            ),
            priority=13,
            ttl=key_lifetime,
        )

    @permission_filter(Permission.RESET_PASSWORD)
    async def send_reset_code(self, email: str) -> None:
        """
        Отправка кода восстановления пароля на почту

        :param email:
        """

        user = await self.user_repo.get(email=email)
        if not user:
            raise exceptions.NotFound("Пользователь не найден")

        key_lifetime = 60 * 30
        self.confirm_manager.set_key(f'password_reset:{email}')
        self.confirm_manager.set_key_lifetime(key_lifetime)
        self.confirm_manager.set_gen_interval(120)
        self.confirm_manager.set_max_gen_attempts(3)

        try:
            code = await self.confirm_manager.generate()
        except ManyGenAttemptsError:
            raise exceptions.BadRequest("Превышено количество попыток, попробуйте позже")
        except AlreadyGenError:
            raise exceptions.BadRequest("Код уже отправлен")

        await self.email.send_email_with_template(
            to=email,
            subject="Восстановление пароля",
            template="reset_password.html",
            kwargs=dict(
                username=user.username,
                code=code
            ),
            priority=13,
            ttl=key_lifetime,
        )

    @permission_filter(Permission.RESET_PASSWORD)
    async def reset_password(self, email: str, code: int, new_password: str) -> None:
        """
        Восстановление пароля

        :param email:
        :param code:
        :param new_password:

        """

        user = await self.user_repo.get(email=email)
        if not user:
            raise exceptions.NotFound("Пользователь не найден")

        key_lifetime = 60 * 30
        self.confirm_manager.set_key(f'password_reset:{email}')
        self.confirm_manager.set_max_verify_attempts(3)
        self.confirm_manager.set_code_valid_time(key_lifetime)

        try:
            await self.confirm_manager.verify(code, delete_key=False)
        except ManyConfirmAttemptsError:
            raise exceptions.BadRequest("Превышено количество попыток")
        except NotGenError:
            raise exceptions.BadRequest("Код не отправлен")
        except InvalidCodeError:
            raise exceptions.BadRequest("Неверный код")
        except ExpiredCodeError:
            raise exceptions.BadRequest("Код устарел, отправьте новый")

        if not schemas.user.is_valid_password(new_password):
            raise exceptions.BadRequest("Невалидный пароль")

        await self.confirm_manager.delete_key()
        await self.user_repo.update(user.id, hashed_password=get_hashed_password(new_password))

        change_time = datetime.now().strftime("%d.%m.%Y в %H:%M")
        await self.email.send_email_with_template(
            to=email,
            subject="Восстановление пароля",
            template="successfully_reset_password.html",
            kwargs=dict(
                username=user.username,
                change_time=change_time,
                ip=self._current_user.ip,
                email=user.email,
            ),
            priority=9
        )

    @permission_filter(Permission.LOGOUT)
    async def logout(self, session_id: str | None) -> None:
        if session_id and self._current_user.id:
            await self.session_manager.delete_session(self._current_user.id, session_id)

    async def refresh_tokens(
            self,
            current_tokens: tuple[str | None, str | None],
            session_id: str | None
    ) -> tuple[schemas.UserMedium, tuple[str, str], str]:
        """
        Обновление токенов

        :param current_tokens: tuple[access_token, refresh_token]
        :param session_id:

        :raise AccessDenied if session is invalid or user is banned
        :raise NotFound if user not found

        :return:
        """

        if not self._current_user.is_valid_session:
            raise exceptions.AccessDenied("Сессия недействительна")

        if not self._current_user.is_valid_refresh_token:
            raise exceptions.AccessDenied("Недействительный refresh token")

        old_payload = self.jwt.validate_token(current_tokens[1])
        user = await self.user_repo.get(id=old_payload.id, as_full=True)
        if not user:
            raise exceptions.NotFound("Пользователь не найден")

        if user.state == schemas.UserState.BLOCKED:
            raise exceptions.AccessDenied("Пользователь заблокирован")

        permission_title_list = [obj.title for obj in user.role.permissions]
        new_tokens = (
            self.jwt.create_token(user.id, user.username, permission_title_list, user.state, "access"),
            self.jwt.create_token(user.id, user.username, permission_title_list, user.state, "refresh")
        )
        new_session_id = await self.session_manager.set_session_id(
            user_id=user.id,
            refresh_token=new_tokens[1],
            ip_address=self._current_user.ip,
            user_agent=str(self._current_user.user_agent),
            session_id=session_id
        )
        await self.redis_client_reauth.delete(session_id)
        user_model = schemas.User.model_validate(user)
        role_model = schemas.RoleMedium(id=user.role.id, title=user.role.title, permissions=permission_title_list)
        return (
            schemas.UserMedium(**user_model.model_dump(exclude={"role"}), role=role_model),
            new_tokens,
            new_session_id
        )
