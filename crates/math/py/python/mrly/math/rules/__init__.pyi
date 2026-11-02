from typing import Any, Literal

from numpy.typing import NDArray

BASE: int

def render(filled: list[bytes], number: int, dimension: int, base: int) -> NDArray[Any]:
    """Builds a hypercube of the given side and rank, marking each cell whose coordinate residues are in the filled list."""

def tree_axes(dimension: int, free_axis: int) -> list[int]:
    """Returns every axis but the free one."""
