from typing import Any, Literal

from numpy.typing import NDArray

def color(colorizer: Any, value: int, max: int) -> tuple[int, int, int, int]:
    ...

def colors(colorizer: Any, values: list[int], max: int) -> NDArray[Any]:
    ...
