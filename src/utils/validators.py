import re
from pathlib import Path
from typing import BinaryIO

from PIL import Image, UnidentifiedImageError


def is_valid_username(username: str) -> bool:
    pattern = r"^(?=.{4,20}$)(?![_.])(?!.*[_.]{2})[a-zA-Z0-9._]+(?<![_.])$"
    return re.match(pattern, username) is not None


def is_valid_password(password: str) -> bool:
    pattern = r"^(?=.*[A-Za-z])(?=.*\d)[A-Za-z\d]{8,32}$"
    return re.match(pattern, password) is not None


def is_valid_first_name(first_name: str) -> bool:
    pattern = r"^[a-zA-Zа-яА-Я]+(?: [a-zA-Zа-яА-Я]+)*$"
    return (re.match(pattern, first_name) is not None) and len(first_name) <= 100


def is_valid_last_name(last_name: str) -> bool:
    pattern = r"^[a-zA-Zа-яА-Я]+(?: [a-zA-Zа-яА-Я]+)*$"
    return (re.match(pattern, last_name) is not None) and len(last_name) <= 100


def is_square_image(file: str | bytes | Path | BinaryIO) -> bool:
    try:
        im = Image.open(file)
        im.verify()

        if im.width != im.height:
            return False
        return True
    except UnidentifiedImageError:
        return False
