import re


def is_valid_email(email: str) -> bool:
    pattern = r"^[-\w\.]+@([-\w]+\.)+[-\w]{2,4}$"

    if re.match(pattern, email) is not None:
        return True
    else:
        return False


def is_valid_username(username: str) -> bool:
    pattern = r"^(?=.{4,20}$)(?![_.])(?!.*[_.]{2})[a-zA-Z0-9._]+(?<![_.])$"
    if re.match(pattern, username) is not None:
        return True
    else:
        return False


def is_valid_password(password: str) -> bool:
    pattern = r"^[\w.#$%&_](?=.*\d)(?=.{8,26}$)"
    if re.match(pattern, password) is not None:
        return True
    else:
        return False
