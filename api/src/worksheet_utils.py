from enum import IntEnum
import re
from typing import Optional, Tuple

Coordinate = Tuple[str, int]


def parse_coordinate(coordinate: str) -> str:
    digit_index = re.search(r"\d", coordinate).start()

    return coordinate[:digit_index], int(coordinate[digit_index:])


def format_coordinate(coordinate: Coordinate) -> str:
    return f"{coordinate[0]}{coordinate[1]}"


# def get_cell_coordinate_bottom(coordinate: Coordinate) -> Coordinate:
#     (column, row) = coordinate
#     return column, row + 1

# def get_cell_coordinate_right(coordinate: Coordinate) -> Coordinate:
#     (column, row) = coordinate
#     return column, row + 1

class CellPostion(IntEnum):
    Top = 0
    Bottom = 1
    Left = 2
    Right = 3


def get_positioned_cell(coordinate: Coordinate, position: CellPostion) -> Coordinate:
    (column, row) = coordinate

    if position == CellPostion.Top:
        return column, row - 1

    if position == CellPostion.Bottom:
        return column, row + 1

    if position == CellPostion.Left:
        return chr(ord(column) - 1), row

    if position == CellPostion.Right:
        return chr(ord(column) + 1), row
