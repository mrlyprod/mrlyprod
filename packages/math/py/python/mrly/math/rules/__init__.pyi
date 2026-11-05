from typing import Any, Literal

from numpy.typing import NDArray

BASE: int

def render(filled: list[bytes], number: int, dimension: int, base: int) -> NDArray[Any]:
    ...

def tree_axes(dimension: int, free_axis: int) -> list[int]:
    ...
