from enum import Enum


class SourceKind(int, Enum):
    OPEN = 0
    HIGH = 1
    LOW = 2
    CLOSE = 3
    VOLUME = 4
    OHLC4 = 5
    HLC3 = 6
    HL2 = 7


SOURCE_KINDS = [
    SourceKind.OPEN.value,
    SourceKind.HIGH.value,
    SourceKind.LOW.value,
    SourceKind.CLOSE.value,
    SourceKind.VOLUME.value,
    SourceKind.OHLC4.value,
    SourceKind.HLC3.value,
    SourceKind.HL2.value,
]
