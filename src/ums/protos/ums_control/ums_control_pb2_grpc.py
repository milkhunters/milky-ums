# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
"""Client and server classes corresponding to protobuf-defined services."""
import grpc

from . import ums_control_pb2 as ums__control__pb2


class UserManagementStub(object):
    """The greeting service definition.
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.GetListOfReauth = channel.unary_unary(
                '/greet.UserManagement/GetListOfReauth',
                request_serializer=ums__control__pb2.GetListRequest.SerializeToString,
                response_deserializer=ums__control__pb2.ListOfDictReply.FromString,
                )


class UserManagementServicer(object):
    """The greeting service definition.
    """

    def GetListOfReauth(self, request, context):
        """Unary
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')


def add_UserManagementServicer_to_server(servicer, server):
    rpc_method_handlers = {
            'GetListOfReauth': grpc.unary_unary_rpc_method_handler(
                    servicer.GetListOfReauth,
                    request_deserializer=ums__control__pb2.GetListRequest.FromString,
                    response_serializer=ums__control__pb2.ListOfDictReply.SerializeToString,
            ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
            'greet.UserManagement', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))


 # This class is part of an EXPERIMENTAL API.
class UserManagement(object):
    """The greeting service definition.
    """

    @staticmethod
    def GetListOfReauth(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/greet.UserManagement/GetListOfReauth',
            ums__control__pb2.GetListRequest.SerializeToString,
            ums__control__pb2.ListOfDictReply.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)