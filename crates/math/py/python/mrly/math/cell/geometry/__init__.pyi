from typing import Any, Literal

from numpy.typing import NDArray

def merge_reps(cells: list[dict[str, Any]], reps: list[int]) -> dict[str, Any]:
    """Merges same-shaped cells into one block laid out by the per-axis repetition counts."""

def perforate(mask: NDArray[Any], cell: dict[str, Any], value: int) -> dict[str, Any]:
    """Writes the value into the cell wherever the tiled mask is nonzero."""
