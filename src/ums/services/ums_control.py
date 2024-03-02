import asyncio

from ums.protos.ums_control import ums_control_pb2, ums_control_pb2_grpc
from ums.utils import RedisClient


class UMService(ums_control_pb2_grpc.UserManagementServicer):
    def __init__(self, redis: RedisClient):
        self.redis = redis

    async def GetListOfReauth(self, request, context):
        all_keys = await self.redis.keys('*')
        all_values = await asyncio.gather(*[self.redis.get(key) for key in all_keys])

        response_list = []
        # Вывод всех значений
        for key, value in zip(all_keys, all_values):
            response_list.append(ums_control_pb2.Dictionary(key=key, value=value))

        return ums_control_pb2.ListOfDictReply(dicts=response_list)
