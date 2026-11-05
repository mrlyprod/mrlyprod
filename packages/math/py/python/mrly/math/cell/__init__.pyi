from typing import Any, Literal

from numpy.typing import NDArray
import mrly.core
from . import census, geometry, models, serializer

def grow(pattern: NDArray[Any], level: int) -> dict[str, Any]:
    ...

def paint(cell: dict[str, Any], custom: dict[int, list[tuple[int, int, int, int]]] | None = None, mode: Literal["Type", "Tag", "Index", "Enumerate", "Random", "Row", "Column", "Depth"] | None = None, rng: mrly.core.Rng | None = None) -> dict[str, Any]:
    ...
