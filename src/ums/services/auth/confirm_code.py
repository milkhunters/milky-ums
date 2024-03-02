import time
from random import randint


class ManyGenAttemptsError(Exception):
    pass


class ManyConfirmAttemptsError(Exception):
    pass


class AlreadyGenError(Exception):
    pass


class NotGenError(Exception):
    pass


class InvalidCodeError(Exception):
    pass


class ExpiredCodeError(Exception):
    pass


class ConfirmCodeUtil:

    def __init__(self, redis):
        self._key = None
        self._set_max_gen_attempts = None
        self._max_verify_attempts = None
        self._key_lifetime = None
        self._code_valid_time = None
        self._gen_interval = None
        self._redis = redis

    def set_key(self, key: str):
        self._key = key

    def set_max_gen_attempts(self, attempts: int):
        self._set_max_gen_attempts = attempts

    def set_max_verify_attempts(self, attempts: int):
        self._max_verify_attempts = attempts

    def set_key_lifetime(self, seconds: int):
        self._key_lifetime = seconds

    def set_code_valid_time(self, seconds: int):
        self._code_valid_time = seconds

    def set_gen_interval(self, seconds: int):
        self._gen_interval = seconds

    async def generate(self, from_num: int = 100000, to_num: int = 999999) -> int:
        """
            Генерирует код подтверждения и записывает его в Redis.
            Возвращает сгенерированный код.

            :param from_num: Начальное значение диапазона генерации кода.
            :param to_num: Конечное значение диапазона генерации кода.
            :return: int - сгенерированный код.

            :raise ManyGenAttemptsError: Превышено количество попыток генерации кода.
            :raise AlreadyGenError: Код уже был сгенерирован.

        """

        records = await self.__get_records()

        if len(records) >= self._set_max_gen_attempts:
            raise ManyGenAttemptsError

        if records:
            last_record = list(records.items())[-1]
            last_send_time = last_record[1]['send_time']
            now_time = int(time.time())

            if last_send_time > (now_time - self._gen_interval):
                raise AlreadyGenError

        code = randint(from_num, to_num)
        data = f'{int(time.time())}:0'
        await self._redis.hset(self._key, code, data)
        await self._redis.expire(self._key, self._key_lifetime)
        return code

    async def verify(self, code: int, delete_key: bool = True) -> None:
        """
            Проверяет код подтверждения.
            Если код не совпадает с последним сгенерированным, то увеличивает количество попыток на 1.

            :param code: Код подтверждения.
            :param delete_key: Удалять ли ключ из Redis, если код валиден.

            :raise ManyConfirmAttemptsError: Превышено количество попыток подтверждения кода.
            :raise NotGenError: Код не был сгенерирован.
            :raise InvalidCodeError: Неверный код подтверждения.
            :raise ExpiredCodeError: Код подтверждения просрочен.

        """
        records = await self.__get_records()

        if not records:
            raise NotGenError

        last_record = list(records.items())[-1]
        last_send_code = last_record[0]
        last_send_time = last_record[1]['send_time']
        attempts = last_record[1]['attempts']
        now_time = int(time.time())

        if attempts >= self._max_verify_attempts:
            raise ManyConfirmAttemptsError

        if last_send_code != code:
            await self._redis.hset(self._key, last_send_code, f'{last_send_time}:{attempts + 1}')
            raise InvalidCodeError

        if last_send_time < (now_time - self._code_valid_time):
            raise ExpiredCodeError

        if delete_key:
            await self._redis.delete(self._key)

    async def __get_records(self) -> dict[int, dict[str, int]]:
        records = dict()

        for code, data in (await self._redis.hgetall(self._key)).items():
            send_time = int(data.split(':')[0])
            attempts = int(data.split(':')[1])
            send_code = int(code)

            records[send_code] = dict(
                send_time=send_time,
                attempts=attempts
            )
        return records

    def delete_key(self):
        self._redis.delete(self._key)
