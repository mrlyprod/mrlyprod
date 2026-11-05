from typing import Any, Literal

from numpy.typing import NDArray

def axes(size: int, lattice: Literal["Square", "Hex"], row: int) -> tuple[list[float], list[float]]:
    ...

def membership(code: int, base: int, dimension: int) -> list[bool]:
    ...

def pack(residues: list[int], base: int) -> int:
    ...
