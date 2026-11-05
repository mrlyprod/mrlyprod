from typing import Any, Literal

from numpy.typing import NDArray

def merge_reps(cells: list[dict[str, Any]], reps: list[int]) -> dict[str, Any]:
    ...

def perforate(mask: NDArray[Any], cell: dict[str, Any], value: int) -> dict[str, Any]:
    ...
