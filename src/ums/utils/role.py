import json
import os

from dataclasses import dataclass


@dataclass
class RoleModel:
    id: str
    title: str
    permissions: list[str]


def load_roles(path: str) -> list[RoleModel]:
    roles = []
    for role in os.listdir(path):
        if os.path.isfile(f'{path}/{role}') and role.endswith('.json'):
            with open(f'{path}/{role}', 'r') as file:
                roles.append(RoleModel(**json.load(file)))
    return roles
