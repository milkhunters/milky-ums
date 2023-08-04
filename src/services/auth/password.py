import hashlib
import random


def get_hashed_password(password: str, salt: str = None) -> str:
    """
    :param password:
    :param salt:
    :return: [salt] + [hex hashed password]
    """
    if not salt:
        salt = str(random.randint(100000, 999999))

    return salt + hashlib.pbkdf2_hmac(
        'sha256', password.encode('utf-8'), salt.encode("utf8"), 50000, dklen=32
    ).hex()


def verify_password(password: str, storage: str) -> bool:
    """
    Проверяет пароль на валидность
    :param password:
    :param storage: [salt] + [hex hashed password]
    :return:
    """
    salt = str(storage[:6])
    hashed_pass_from_storage = storage[6:]
    new_hash_pass = hashlib.pbkdf2_hmac('sha256', password.encode('utf-8'), salt.encode('utf-8'), 50000, dklen=32).hex()
    return new_hash_pass == hashed_pass_from_storage