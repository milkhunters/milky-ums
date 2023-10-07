import asyncio

from src.protos.ums_control import ums_control_pb2, ums_control_pb2_grpc
from src.utils import RedisClient


class UMService(ums_control_pb2_grpc.UserManagementServicer):
    def __init__(self, app_state):
        self.redis_reauth: RedisClient = app_state.redis_reauth

    async def GetListOfReauth(self, request, context):
        all_keys = await self.redis_reauth.keys('*')
        all_values = await asyncio.gather(*[self.redis_reauth.get(key) for key in all_keys])

        response_list = []
        # Вывод всех значений
        for key, value in zip(all_keys, all_values):
            response_list.append(ums_control_pb2.Dictionary(key=key, value=value))

        return ums_control_pb2.ListOfDictReply(dicts=response_list)
