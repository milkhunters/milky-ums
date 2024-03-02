import os

from ums.config import Config


class StatsApplicationService:

    def __init__(self, config: Config):
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
