from typing import Any, Literal

from numpy.typing import NDArray

MOST: int

def essential(sensors: list[int]) -> list[bool]:
    ...

def fractal(generator: list[int], base: int, level: int) -> list[int]:
    ...

def holes(sensors: list[int]) -> list[int]:
    ...

def law(generator: list[int], level: int) -> tuple[int, int] | None:
    ...

def lost(sensors: list[int], sensor: int) -> list[int]:
    ...

def paired(sensors: list[int]) -> list[int]:
    ...

def weights(sensors: list[int]) -> list[int]:
    ...
