import time
from random import randint

from fastapi.requests import Request
from fastapi.responses import Response

from src import exceptions
from src.models import schemas
from src.models import tables
from src.models.permission import Permission
from src.models.auth import BaseUser
from src.models.state import UserState
from src.services.repository import UserRepo
from src.utils import EmailSender
from src.utils import RedisClient
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
            redis_confirmations: RedisClient,
            email: EmailSender,
    ):
        self._current_user = current_user
        self._jwt_manager = jwt_manager
        self._session_manager = session_manager
        self._user_repo = user_repo
        self._redis_client = redis_client
        self._redis_client_reauth = redis_client_reauth
        self._redis_confirmations = redis_confirmations
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

        key = f'email_confirm:{email}'
        records = dict()
        for code, data in (await self._redis_confirmations.hgetall(key)).items():
            send_time = data.split(':')[0]
            attempts = data.split(':')[1]
            records[int(code)] = dict(
                send_time=int(send_time),
                attempts=int(attempts)
            )

        if len(records) >= 3:
            raise exceptions.BadRequest("Превышено количество попыток, попробуйте позже")

        if records:
            last_record = list(records.items())[-1]
            last_send_time = last_record[1]['send_time']
            now_time = int(time.time())

            if last_send_time > (now_time - 120):
                raise exceptions.BadRequest("Код уже отправлен")

        code = randint(100000, 999999)
        data = f'{int(time.time())}:0'
        await self._redis_confirmations.hset(key, code, data)
        await self._redis_confirmations.expire(key, 60 * 30)
        await self._email.send_email_with_template(
            to=email,
            subject="Подтверждение почты",
            template="confirm_email.html",
            code=code
        )

    @permission_filter(Permission.VERIFY_EMAIL)
    async def verify_email(self, email: str, code: int) -> None:
        """
        Подтверждение почты

        :param email:
        :param code:

        :raise NotFound: if user not found
        :raise AccessDenied: if user is banned
        """

        user = await self._user_repo.get(email=email)
        if not user:
            raise exceptions.NotFound("Пользователь не найден")

        if user.state != UserState.NOT_CONFIRMED:
            raise exceptions.AccessDenied("Пользователь уже подтвержден")

        key = f'email_confirm:{email}'
        records = dict()
        for send_code, data in (await self._redis_confirmations.hgetall(key)).items():
            send_time = data.split(':')[0]
            attempts = data.split(':')[1]
            records[int(send_code)] = dict(
                send_time=int(send_time),
                attempts=int(attempts)
            )

        if not records:
            raise exceptions.BadRequest("Код не отправлен")

        last_record = list(records.items())[-1]
        last_send_code = last_record[0]
        last_send_time = last_record[1]['send_time']
        attempts = last_record[1]['attempts']
        now_time = int(time.time())

        if attempts >= 3:
            raise exceptions.BadRequest("Превышено количество попыток")

        if last_send_code != code:
            await self._redis_confirmations.hset(key, last_send_code, f'{last_send_time}:{attempts + 1}')
            raise exceptions.BadRequest("Неверный код")

        if last_send_time < (now_time - 86400):
            raise exceptions.BadRequest("Код устарел, отправьте новый")

        await self._user_repo.update(user.id, state=UserState.ACTIVE)

    @permission_filter(Permission.RESET_PASSWORD)
    async def reset_password(self, email: str) -> None:
        """
        Отправка кода восстановления пароля на почту

        :param email:

        :raise NotFound: if user not found
        :raise AccessDenied: if user is banned
        """

        user: tables.User = await self._user_repo.get(email=email)
        if not user:
            raise exceptions.NotFound("Пользователь не найден")

        keys = await self._redis_client.keys(pattern=f'reset:{email}*')
        if keys:
            data_key = keys[0].split(':')
            if int(data_key[2]) > int(time.time()) - 120:
                raise exceptions.BadRequest("Код уже отправлен")
            await self._redis_client.delete(keys[0])

        code = randint(100000, 999999)
        await self._redis_client.set(f"reset:{email}:{int(time.time())}:0", code, expire=60 * 60)
        await self._email.send_email(email, "Восстановление пароля", f"Код восстановления: <b>{code}</b>")

    @permission_filter(Permission.RESET_PASSWORD)
    async def confirm_reset_password(self, email: str, code: int, new_password: str) -> None:
        """
        Восстановление пароля

        :param email:
        :param code:
        :param new_password:

        :raise NotFound: if user not found
        :raise AccessDenied: if user is banned
        """

        user = await self._user_repo.get(email=email)
        if not user:
            raise exceptions.NotFound("Пользователь не найден")

        keys = await self._redis_client.keys(pattern=f'reset:{email}*')
        if not keys:
            raise exceptions.BadRequest("Код не отправлен")

        data = keys[0].split(':')
        email = data[1]
        timestamp = int(data[2])
        attempts = int(data[3])

        if timestamp < int(time.time()) - 86400:
            raise exceptions.BadRequest("Код устарел, отправьте новый")

        if attempts > 3:
            raise exceptions.BadRequest("Превышено количество попыток")

        code_in_db = await self._redis_client.get(keys[0])
        if int(code_in_db) != code:
            await self._redis_client.delete(keys[0])
            await self._redis_client.set(f"reset:{email}:{timestamp}:{attempts + 1}", code, expire=60 * 60)
            raise exceptions.BadRequest("Неверный код")

        await self._user_repo.update(user.id, hashed_password=get_hashed_password(new_password))
        await self._redis_client.delete(keys[0])

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
