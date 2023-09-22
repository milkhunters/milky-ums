import re


def is_valid_username(username: str) -> bool:
    pattern = r"^(?=.{4,20}$)(?![_.])(?!.*[_.]{2})[a-zA-Z0-9._]+(?<![_.])$"
    return re.match(pattern, username) is not None


def is_valid_password(password: str) -> bool:
    pattern = r"^(?=.*[A-Za-z])(?=.*\d)[A-Za-z\d]{8,32}$"
    return re.match(pattern, password) is not None
