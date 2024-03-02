from argon2 import PasswordHasher
from argon2.exceptions import VerifyMismatchError


def get_hashed_password(password: str) -> str:
    """
    :param password:
    :return: [salt] + [hex hashed password]
    """
    ph = PasswordHasher()

    return ph.hash(password)


def verify_password(password: str, storage: str) -> bool:
    """
    Проверяет пароль на валидность
    :param password:
    :param storage: [salt] + [hex hashed password]
    :return:
    """
    ph = PasswordHasher()

    try:
        ph.verify(storage, password)
    except VerifyMismatchError:
        return False
    return True
