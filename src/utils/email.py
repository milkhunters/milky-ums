import time
import uuid

import aio_pika
from aio_pika import ExchangeType
from jinja2 import Environment, PackageLoader, select_autoescape

from src.config import Config


class EmailSender:
    def __init__(self, rmq: aio_pika.abc.AbstractRobustConnection, config: Config):
        self._rmq = rmq
        self._config = config
        self._templates = Environment(
            loader=PackageLoader('src', 'views/email_templates'),
            autoescape=select_autoescape(['html', 'xml'])
        )

    async def send_email(
            self,
            to: str,
            subject: str,
            content: str,
            content_type: str = "text/html",
            priority: int = 0,
            ttl: int = None,

    ):
        """
        Отправляет сообщение на почту

        :param to: почта получателя.
        :param subject: тема сообщения
        :param content: текст сообщения
        :param content_type: тип контента
        :param priority: приоритет 0-255
        :param ttl:
        """

        channel = await self._rmq.channel()

        exchange = await channel.declare_exchange(
            self._config.EMAIL.RabbitMQ.EXCHANGE,
            ExchangeType.DIRECT,
            durable=True,
            auto_delete=False,
            internal=False,
            passive=True,
        )

        await exchange.publish(
            aio_pika.Message(
                headers={
                    "To": to,
                    "Subject": subject,
                    "FromId": self._config.EMAIL.SENDER_ID,
                },
                body=content.encode(),
                content_type=content_type,
                priority=priority,
                expiration=ttl,
                timestamp=time.time(),
                message_id=str(uuid.uuid4()),
                app_id=self._config.BASE.TITLE
            ),
            routing_key="",
        )

    async def send_email_with_template(
            self,
            to: str,
            subject: str,
            template: str,
            kwargs: dict,
            priority: int = 0,
            ttl: int = None
    ):
        """
        Отправляет сообщение на почту с использованием шаблона jinja2
        """
        template = self._templates.get_template(template)
        await self.send_email(
            to,
            subject,
            template.render(**kwargs),
            content_type="text/html",
            priority=priority,
            ttl=ttl
        )
