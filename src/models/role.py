import typing
from enum import Enum, unique
from typing import Tuple


@unique
class MainRole(Enum):
    GUEST = 0
    USER = 1
    MODER = 2
    ADMIN = 3


@unique
class AdditionalRole(Enum):
    ONE = 1
    TWO = 2
    THREE = 3
    FOUR = 4
    FIVE = 5
    SIX = 6
    SEVEN = 7
    EIGHT = 8
    NINE = 9


class RoleRange:
    def __init__(
            self,
            super_symbol: typing.Literal['*'] = None,
            *,
            left: 'Role' = None,
            operator: 'RoleOperationType' = None,
            right: 'Role' = None
    ):
        if super_symbol == "*":
            self.left = Role(MainRole.GUEST, AdditionalRole.ONE)
            self.operator = RoleOperationType.LE
            self.right = Role(MainRole.ADMIN, AdditionalRole.NINE)
        else:
            self.left = left
            self.operator = operator
            self.right = right

    def __eq__(self, other):
        if isinstance(other, Role):
            return self.is_include(other)
        elif isinstance(other, RoleRange):
            return self.left == other.left and self.right == other.right and self.operator == other.operator
        else:
            raise ValueError("Неверный тип данных")

    def __ne__(self, other):
        return not self.__eq__(other)

    def is_include(self, role: 'Role') -> bool:
        if self.operator == RoleOperationType.LT:
            return role.value() < self.right.value()
        elif self.operator == RoleOperationType.GT:
            return role.value() > self.right.value()
        elif self.operator == RoleOperationType.LE:
            return role.value() <= self.right.value()
        elif self.operator == RoleOperationType.GE:
            return role.value() >= self.right.value()


class RoleOperationType(str, Enum):
    LT = "<"
    GT = ">"
    LE = "<="
    GE = ">="


class Role:
    """
    Класс для работы с ролями пользователей

    Принимает на вход два параметра:
    - main_role: enum MainRole
    - additional_role: enum AdditionalRole
    Может вернуть как int, так и tuple из  двух
    переданных параметров

    Пример использования:

    >>> Role(MainRole.user, AdditionalRole.one)

    >>> 11

    :param main_role:
    :param additional_role:

    """

    def __init__(self, main_role: MainRole, additional_role: AdditionalRole):
        self.main_role = main_role
        self.additional_role = additional_role

    def value(self) -> int:
        return int(f"{self.main_role.value}{self.additional_role.value}")

    def to_int(self):
        return int(self)

    def to_tuple(self) -> Tuple[MainRole, AdditionalRole]:
        return self.main_role, self.additional_role

    def __int__(self) -> int:
        return self.value()

    def __eq__(self, other):
        if isinstance(other, Role):
            return self.value() == other.value()
        elif isinstance(other, int):
            return self.value() == other
        else:
            raise ValueError("Неверный тип данных")

    def __ne__(self, other):
        if isinstance(other, Role):
            return self.value() != other.value()
        elif isinstance(other, int):
            return self.value() != other
        else:
            raise ValueError("Неверный тип данных")

    def __repr__(self):
        return f"{self.main_role.name} {self.additional_role.name}"

    def __lt__(self, other):
        if isinstance(other, Role):
            return self.value() < other.value()
        elif isinstance(other, int):
            return self.value() < other

    def __gt__(self, other):
        if isinstance(other, Role):
            return self.value() > other.value()
        elif isinstance(other, int):
            return self.value() > other

    def __le__(self, other):
        if isinstance(other, Role):
            return self.value() <= other.value()
        elif isinstance(other, int):
            return self.value() <= other

    def __ge__(self, other):
        if isinstance(other, Role):
            return self.value() >= other.value()
        elif isinstance(other, int):
            return self.value() >= other

    @classmethod
    def from_int(cls, value: int):
        """
        Преобразует целое число в класс Role
        :param value:

        :return: self
        """
        if value not in range(10, 40):
            raise ValueError("Значение роли должно быть в диапазоне 10-39")

        main_role = value // 10
        additional_role = value % 10
        return cls(MainRole(main_role), AdditionalRole(additional_role))
