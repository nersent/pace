import re
from typing import Optional, Tuple

Coordinate = Tuple[str, int]


def parse_coordinate(coordinate: str) -> str:
    digit_index = re.search(r"\d", coordinate).start()

    return coordinate[:digit_index], int(coordinate[digit_index:])


def format_coordinate(coordinate: Coordinate) -> str:
    return f"{coordinate[0]}{coordinate[1]}"


def get_cell_coordinate_below(coordinate: Coordinate) -> Coordinate:
    (column, row) = coordinate
    return column, row + 1
