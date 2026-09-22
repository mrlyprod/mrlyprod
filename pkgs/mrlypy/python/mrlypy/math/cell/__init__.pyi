from typing import Any, Literal

from numpy.typing import NDArray
import mrlypy.core
from . import census, geometry, models, serializer

def grow(pattern: NDArray[Any], level: int) -> dict[str, Any]:
    """Grows a seed pattern into a cell, deepened to its fractal past level one."""

def paint(cell: dict[str, Any], custom: dict[int, list[tuple[int, int, int, int]]] | None = None, mode: Literal["Type", "Tag", "Index", "Enumerate", "Random", "Row", "Column", "Depth"] | None = None, rng: mrlypy.core.Rng | None = None) -> dict[str, Any]:
    """Colors the cell through the given mapping and mode, defaulting to the standard palette by type."""
