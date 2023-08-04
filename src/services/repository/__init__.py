from .user import UserRepo


class RepoFactory:
    def __init__(self, session):
        self._session = session

    @property
    def user(self) -> UserRepo:
        return UserRepo(self._session)
