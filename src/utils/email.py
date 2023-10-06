import aio_pika
from jinja2 import Environment, PackageLoader, select_autoescape

from src.config import Email as EmailConfig


class EmailSender:
    def __init__(self, rmq: aio_pika.abc.AbstractRobustConnection, config: EmailConfig):
        self._rmq = rmq
        self._config = config
        self._templates = Environment(
            loader=PackageLoader('src', 'views/email_templates'),
            autoescape=select_autoescape(['html', 'xml'])
        )

    async def send_email(self, to: str, subject: str, content: str, content_type: str = "text/html"):
        """
        Отправляет сообщение на почту

        :param to: почта получателя.
        :param subject: тема сообщения
        :param content: текст сообщения
        :param content_type: тип контента
        """

        channel = await self._rmq.channel()
        queue = await channel.declare_queue(self._config.RabbitMQ.QUEUE, durable=True)

        await channel.default_exchange.publish(
            aio_pika.Message(
                headers={
                    "To": to,
                    "Subject": subject,
                    "FromName": self._config.FROM_NAME
                },
                body=content.encode(),
                content_type=content_type
            ),
            routing_key=queue.name,
        )

    async def send_email_with_template(self, to: str, subject: str, template: str, **kwargs):
        """
        Отправляет сообщение на почту с использованием шаблона jinja2
        """
        template = self._templates.get_template(template)
        await self.send_email(to, subject, template.render(**kwargs), content_type="text/html")
