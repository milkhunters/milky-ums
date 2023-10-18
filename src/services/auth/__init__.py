from datetime import datetime

from fastapi.requests import Request
from fastapi.responses import Response

from src import exceptions
from src.models import schemas
from src.models import tables
from src.models.permission import Permission
from src.models.auth import BaseUser
from src.models.state import UserState
from src.services.repository import UserRepo
from src.utils import EmailSender, is_valid_password
from src.utils import RedisClient

from .confirm_code import ConfirmCodeUtil
from .confirm_code import ManyGenAttemptsError
from .confirm_code import ManyConfirmAttemptsError
from .confirm_code import AlreadyGenError
from .confirm_code import NotGenError
from .confirm_code import InvalidCodeError
from .confirm_code import ExpiredCodeError

from .filters import permission_filter, state_filter
from .jwt import JWTManager
from .password import verify_password, get_hashed_password
from .session import SessionManager


class AuthApplicationService:
    def __init__(
            self,
            current_user: BaseUser,
            *,
            jwt_manager: JWTManager,
            session_manager: SessionManager,
            user_repo: UserRepo,
            redis_client: RedisClient,
            redis_client_reauth: RedisClient,
            confirm_code_util: ConfirmCodeUtil,
            email: EmailSender,
    ):
        self._current_user = current_user
        self._jwt_manager = jwt_manager
        self._session_manager = session_manager
        self._user_repo = user_repo
        self._redis_client = redis_client
        self._redis_client_reauth = redis_client_reauth
        self._confirm_code_util = confirm_code_util
        self._email = email

    @permission_filter(Permission.CREATE_USER)
    async def create_user(self, user: schemas.UserCreate) -> None:
        """
        Создание нового пользователя

        :param user: UserCreate

        :raise AccessDenied if user is already logged in
        :raise AlreadyExists Conflict if user already exists

        :return: User
        """

        if await self._user_repo.get_by_username_insensitive(user.username):
            raise exceptions.AlreadyExists(f"Пользователь {user.username!r} уже существует")

        if await self._user_repo.get_by_email_insensitive(user.email):
            raise exceptions.AlreadyExists(f"Пользователь с email {user.email!r} уже существует")

        hashed_password = get_hashed_password(user.password)
        await self._user_repo.create(
            **user.model_dump(exclude={"password"}),
            role_id="00000000-0000-0000-0000-000000000000",
            hashed_password=hashed_password
        )

    @permission_filter(Permission.AUTHENTICATE)
    async def authenticate(self, data: schemas.UserAuth, response: Response) -> schemas.UserMedium:
        """
        Аутентификация пользователя

        :param data: UserAuth
        :param response: Response

        :return: User

        :raise AlreadyExists: if user is already logged in
        :raise NotFound: if user not found
        :raise AccessDenied: if user is banned
        """

        user: tables.User = await self._user_repo.get_by_username_insensitive(username=data.username, as_full=True)
        if not user:
            raise exceptions.NotFound("Пользователь не найден")
        if not verify_password(data.password, user.hashed_password):
            raise exceptions.NotFound("Неверная пара логин/пароль")
        if user.state == UserState.BLOCKED:
            raise exceptions.AccessDenied("Пользователь заблокирован")
        if user.state == UserState.NOT_CONFIRMED:
            raise exceptions.AccessDenied("Пользователь не подтвержден")
        if user.state == UserState.DELETED:
            raise exceptions.AccessDenied("Пользователь удален")

        # Генерация и установка токенов
        permission_title_list = [obj.title for obj in user.role.permissions]
        tokens = self._jwt_manager.generate_tokens(user.id, user.username, permission_title_list, user.state)
        self._jwt_manager.set_jwt_cookie(response, tokens)
        await self._session_manager.set_session_id(
            response=response,
            refresh_token=tokens.refresh_token,
            user_id=user.id,
            ip_address=self._current_user.ip,
            user_agent=str(self._current_user.user_agent)
        )
        user_model = schemas.User.model_validate(user)
        role_model = schemas.RoleMedium(id=user.role.id, title=user.role.title, permissions=permission_title_list)
        return schemas.UserMedium(**user_model.model_dump(exclude={"role"}), role=role_model)

    @permission_filter(Permission.VERIFY_EMAIL)
    async def send_verify_code(self, email: str) -> None:
        """
        Отправка кода подтверждения на почту

        :param email:

        :raise NotFound: if user not found
        :raise AccessDenied: if user already verified
        :raise BadRequest: if code already sent
        """

        user = await self._user_repo.get(email=email)
        if not user:
            raise exceptions.NotFound("Пользователь не найден")

        if user.state != UserState.NOT_CONFIRMED:
            raise exceptions.AccessDenied("Пользователь уже подтвержден")

        key_lifetime = 60 * 30
        self._confirm_code_util.set_key(f'email_confirm:{email}')
        self._confirm_code_util.set_key_lifetime(key_lifetime)
        self._confirm_code_util.set_gen_interval(120)
        self._confirm_code_util.set_max_gen_attempts(3)

        try:
            code = await self._confirm_code_util.generate()
        except ManyGenAttemptsError:
            raise exceptions.BadRequest("Превышено количество попыток, попробуйте позже")
        except AlreadyGenError:
            raise exceptions.BadRequest("Код уже отправлен")

        await self._email.send_email_with_template(
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

        user = await self._user_repo.get(email=email)
        if not user:
            raise exceptions.NotFound("Пользователь не найден")

        if user.state != UserState.NOT_CONFIRMED:
            raise exceptions.AccessDenied("Пользователь уже подтвержден")

        key_lifetime = 60 * 30
        self._confirm_code_util.set_key(f'email_confirm:{email}')
        self._confirm_code_util.set_max_verify_attempts(3)
        self._confirm_code_util.set_code_valid_time(key_lifetime)

        try:
            await self._confirm_code_util.verify(code)
        except ManyConfirmAttemptsError:
            raise exceptions.BadRequest("Превышено количество попыток")
        except NotGenError:
            raise exceptions.BadRequest("Код не отправлен")
        except InvalidCodeError:
            raise exceptions.BadRequest("Неверный код")
        except ExpiredCodeError:
            raise exceptions.BadRequest("Код устарел, отправьте новый")

        await self._user_repo.update(user.id, state=UserState.ACTIVE)

        await self._email.send_email_with_template(
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

        user = await self._user_repo.get(email=email)
        if not user:
            raise exceptions.NotFound("Пользователь не найден")

        key_lifetime = 60 * 30
        self._confirm_code_util.set_key(f'password_reset:{email}')
        self._confirm_code_util.set_key_lifetime(key_lifetime)
        self._confirm_code_util.set_gen_interval(120)
        self._confirm_code_util.set_max_gen_attempts(3)

        try:
            code = await self._confirm_code_util.generate()
        except ManyGenAttemptsError:
            raise exceptions.BadRequest("Превышено количество попыток, попробуйте позже")
        except AlreadyGenError:
            raise exceptions.BadRequest("Код уже отправлен")

        await self._email.send_email_with_template(
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

        user = await self._user_repo.get(email=email)
        if not user:
            raise exceptions.NotFound("Пользователь не найден")

        key_lifetime = 60 * 30
        self._confirm_code_util.set_key(f'password_reset:{email}')
        self._confirm_code_util.set_max_verify_attempts(3)
        self._confirm_code_util.set_code_valid_time(key_lifetime)

        try:
            await self._confirm_code_util.verify(code, delete_key=False)
        except ManyConfirmAttemptsError:
            raise exceptions.BadRequest("Превышено количество попыток")
        except NotGenError:
            raise exceptions.BadRequest("Код не отправлен")
        except InvalidCodeError:
            raise exceptions.BadRequest("Неверный код")
        except ExpiredCodeError:
            raise exceptions.BadRequest("Код устарел, отправьте новый")

        if not is_valid_password(new_password):
            raise exceptions.BadRequest("Невалидный пароль")

        await self._confirm_code_util.delete_key()
        await self._user_repo.update(user.id, hashed_password=get_hashed_password(new_password))

        change_time = datetime.now().strftime("%d.%m.%Y в %H:%M")
        await self._email.send_email_with_template(
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
    async def logout(self, request: Request, response: Response) -> None:
        self._jwt_manager.delete_jwt_cookie(response)
        session_id = self._session_manager.get_session_id(request)
        if session_id and self._current_user.id:
            await self._session_manager.delete_session(self._current_user.id, session_id, response)

    async def refresh_tokens(self, request: Request, response: Response) -> schemas.UserMedium:
        """
        Обновление токенов
        :param request:
        :param response:

        :raise AccessDenied if session is invalid or user is banned
        :raise NotFound if user not found

        :return:
        """

        current_tokens = self._jwt_manager.get_jwt_cookie(request)
        session_id = self._session_manager.get_session_id(request)

        if not self._current_user.is_valid_session:
            raise exceptions.AccessDenied("Invalid session")

        if not self._current_user.is_valid_refresh_token:
            raise exceptions.AccessDenied("Invalid refresh token")

        old_payload = self._jwt_manager.decode_refresh_token(current_tokens.refresh_token)
        user = await self._user_repo.get(id=old_payload.id, as_full=True)
        if not user:
            raise exceptions.NotFound("Пользователь не найден")

        if user.state == UserState.BLOCKED:
            raise exceptions.AccessDenied("Пользователь заблокирован")

        permission_title_list = [obj.title for obj in user.role.permissions]
        new_tokens = self._jwt_manager.generate_tokens(user.id, user.username, permission_title_list, user.state)
        self._jwt_manager.set_jwt_cookie(response, new_tokens)
        await self._session_manager.set_session_id(
            response=response,
            user_id=user.id,
            refresh_token=new_tokens.refresh_token,
            ip_address=self._current_user.ip,
            user_agent=str(self._current_user.user_agent),
            session_id=session_id
        )
        await self._redis_client_reauth.delete(session_id)
        user_model = schemas.User.model_validate(user)
        role_model = schemas.RoleMedium(id=user.role.id, title=user.role.title, permissions=permission_title_list)
        return schemas.UserMedium(**user_model.model_dump(exclude={"role"}), role=role_model)
