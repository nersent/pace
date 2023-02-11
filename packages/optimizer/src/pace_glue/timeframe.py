from enum import Enum


class Timeframe(int, Enum):
    ONE_DAY = 0
    FOUR_HOURS = 1
    ONE_HOUR = 2
