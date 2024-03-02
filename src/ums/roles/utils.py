import json
import os

from dataclasses import dataclass


@dataclass
class RoleModel:
    id: int
    title: str
    permissions: list[str]


class RoleLoader:
    def __init__(self, path: str | os.PathLike[str]):
        self._path = path
        self._roles = None

    @property
    def roles(self) -> list[RoleModel]:
        if self._roles is None:
            self._roles = RoleLoader.load_roles(self._path)
        return self._roles

    @staticmethod
    def load_roles(path:  str | os.PathLike[str]) -> list[RoleModel]:
        roles = []
        for role in os.listdir(path):
            if os.path.isfile(f'{path}/{role}') and role.endswith('.json'):
                with open(f'{path}/{role}', 'r') as file:
                    roles.append(RoleModel(**json.load(file)))
        return roles
