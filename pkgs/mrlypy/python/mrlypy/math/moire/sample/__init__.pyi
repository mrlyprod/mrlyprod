from typing import Any, Literal

from numpy.typing import NDArray

def axes(size: int, lattice: Literal["Square", "Hex"], row: int) -> tuple[list[float], list[float]]:
    """Returns the two lattice coordinates of each pixel centre along a row."""

def membership(code: int, base: int, dimension: int) -> list[bool]:
    """Unpacks a code into its residue-corner truth table."""

def pack(residues: list[int], base: int) -> int:
    """Folds residues into a base-q index of the truth table."""
