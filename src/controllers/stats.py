from fastapi import APIRouter, Depends
from fastapi import status as http_status

from src.dependencies.services import get_services
from src.services import ServiceFactory

router = APIRouter()


@router.get("/version", response_model=dict, status_code=http_status.HTTP_200_OK)
async def version(details: bool = False, services: ServiceFactory = Depends(get_services)):
    """
    Получить информацию о приложении

    Ограничений по роли нет
    """
    return await services.stats.get_stats(details)


@router.get("/ping_redis", response_model=bool, status_code=http_status.HTTP_200_OK)
async def ping_redis(services: ServiceFactory = Depends(get_services)):
    """
    Получить состояние redis

    Ограничений по роли нет
    """
    return await services.stats.redis_ping()


@router.get("/ping", response_model=str, status_code=http_status.HTTP_200_OK)
def ping():
    return "pong"
