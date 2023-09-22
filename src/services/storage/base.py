import uuid
from abc import ABC, abstractmethod
from dataclasses import dataclass
from typing import IO

import typing


@dataclass
class MetaData:
    filename: str
    content_type: str


class AbstractStorage(ABC):
    """Abstract storage class"""

    @abstractmethod
    async def get(self, file_id: uuid.UUID) -> typing.AsyncIterable[str | bytes] | None:
        """
        Получить файл из хранилища
        :param file_id:
        :return:
        """
        pass

    @abstractmethod
    async def save(self, file_id: uuid.UUID, file: bytes | IO, metadata: MetaData = None) -> None:
        """
        Сохранить файл в хранилище
        :param file:
        :param file_id
        :param metadata:
        """
        pass

    @abstractmethod
    async def delete(self, file_id: uuid.UUID) -> None:
        """
        Удалить файл из хранилища
        :param file_id:
        :return:
        """
        pass

    @abstractmethod
    async def info(self, file_id: uuid.UUID) -> MetaData | None:
        """
        Получить мета-информацию о файле
        :param file_id:
        :return:
        """
        pass

    def generate_url(
            self,
            file_id: uuid.UUID,
            content_type: str,
            rcd: typing.Literal["inline", "attachment"],
            filename: str = None
    ) -> str:
        """
        Получить ссылку на файл
        :param rcd:
        :param content_type:
        :param filename:
        :param file_id:
        :return:

        ---
        rcd: ResponseContentDisposition

        * inline - открывает файл в браузере
        * attachment - скачивает файл

        ---
        """
        pass
