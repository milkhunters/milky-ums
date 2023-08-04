from enum import Enum, unique


@unique
class ErrorType(int, Enum):
    MESSAGE = 1
    FIELD_LIST = 2
