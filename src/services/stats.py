import os


class StatsApplicationService:

    def __init__(self, redis_client, config):
        self._redis_client = redis_client
        self._config = config

    async def get_stats(self, details: bool = False) -> dict:
        info = {
            "version": self._config.BASE.VERSION,
        }
        if details:
            info.update(
                {
                    "DEBUG": self._config.DEBUG,
                    "build": os.getenv("BUILD", "unknown"),
                    "branch": os.getenv("BRANCH", "unknown"),
                }
            )
        return info

    async def redis_ping(self) -> bool:
        return await self._redis_client.ping()
