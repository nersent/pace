from enum import Enum


class SourceKind(Enum):
    OPEN = 0
    HIGH = 1
    LOW = 2
    CLOSE = 3
    VOLUME = 4
    OHLC4 = 5
    HLC3 = 6
    HL2 = 7
