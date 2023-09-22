import typing
import uuid
from collections import namedtuple
from urllib.parse import urljoin, urlencode, urlparse, urlunparse

from botocore.exceptions import ClientError

from typing import IO

from aiobotocore.response import StreamingBody

from .base import AbstractStorage, MetaData

from aiobotocore.session import AioSession


class S3Storage(AbstractStorage):

    def __init__(self, bucket: str, external_host: str, storage_path: str = ""):
        self._bucket = bucket
        self._external_host = external_host
        self._storage_path = storage_path
        self._client = None

    async def create_session(
            self,
            secret_access_key: str,
            access_key_id: str,
            region_name: str,
            service_name: str,
            endpoint_url: str,
            use_ssl: bool = False
    ):
        session = AioSession()
        self._client = await session.create_client(
            aws_secret_access_key=secret_access_key,
            aws_access_key_id=access_key_id,
            region_name=region_name,
            service_name=service_name,
            endpoint_url=endpoint_url,
            use_ssl=use_ssl
        ).__aenter__()
        return self

    async def close(self):
        await self._client.__aexit__()

    async def get(self, file_id: uuid.UUID) -> StreamingBody | None:
        try:
            response = await self._client.get_object(Bucket=self._bucket, Key=self._storage_path + str(file_id))
            return response['Body']
        except ClientError:
            return None

    async def save(self, file_id: uuid.UUID, file: bytes | IO, metadata: MetaData = None):
        await self._client.put_object(
            Bucket=self._bucket,
            Body=file,
            Key=self._storage_path + str(file_id),
            Metadata={
                "filename": metadata.filename,
                "content_type": metadata.content_type
            }
        )

    async def delete(self, file_id: uuid.UUID) -> None:
        await self._client.delete_object(Bucket=self._bucket, Key=self._storage_path + str(file_id))

    async def info(self, file_id: uuid.UUID) -> MetaData | None:
        try:
            response = await self._client.head_object(Bucket=self._bucket, Key=self._storage_path + str(file_id))
            return MetaData(
                filename=response['ResponseMetadata']['HTTPHeaders']['x-amz-meta-filename'],
                content_type=response['ResponseMetadata']['HTTPHeaders']['x-amz-meta-content_type']
            )
        except ClientError:
            return None

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

        base_url = "/".join(item.strip("/") for item in [
            self._external_host,
            self._bucket,
            self._storage_path,
            str(file_id)
        ] if item != "")

        query_params = urlencode({
            "response-content-disposition": rcd + (f"; filename={filename}" if filename else ""),
            "response-content-type": content_type
        })
        return f"{base_url}?{query_params}"
