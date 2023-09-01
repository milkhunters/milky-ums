from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class GetListRequest(_message.Message):
    __slots__ = []
    def __init__(self) -> None: ...

class Dictionary(_message.Message):
    __slots__ = ["key", "value"]
    KEY_FIELD_NUMBER: _ClassVar[int]
    VALUE_FIELD_NUMBER: _ClassVar[int]
    key: str
    value: str
    def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...

class ListOfDictReply(_message.Message):
    __slots__ = ["dicts"]
    DICTS_FIELD_NUMBER: _ClassVar[int]
    dicts: _containers.RepeatedCompositeFieldContainer[Dictionary]
    def __init__(self, dicts: _Optional[_Iterable[_Union[Dictionary, _Mapping]]] = ...) -> None: ...
