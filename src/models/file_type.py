from enum import Enum


class FileType(str, Enum):
    PHOTO_JPEG = "image/jpeg"
    PHOTO_PNG = "image/png"
    PHOTO_GIF = "image/gif"

    @classmethod
    def has_value(cls, value):
        return value in cls._value2member_map_
