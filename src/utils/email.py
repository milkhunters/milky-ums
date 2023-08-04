import aio_pika

from src.config import Email as EmailConfig


class EmailSender:
    def __init__(self, rmq: aio_pika.RobustConnection, config: EmailConfig):
        self._rmq = rmq
        self._config = config

    async def send_mail(self, to: str, subject: str, content: str, content_type: str = "text/html"):
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
