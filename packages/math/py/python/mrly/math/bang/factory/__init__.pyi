from typing import Any, Literal

from numpy.typing import NDArray

def create(code: int, number: int, dimension: int, base: int, level: int) -> NDArray[Any]:
    ...

def create_from_corners(filled: list[bytes], number: int, dimension: int, base: int, level: int) -> NDArray[Any]:
    ...

def create_named(spec: str, number: int, level: int) -> NDArray[Any]:
    ...

def residue_corners(dimension: int, base: int) -> list[bytes]:
    ...

def total_codes(dimension: int, base: int) -> int:
    ...
