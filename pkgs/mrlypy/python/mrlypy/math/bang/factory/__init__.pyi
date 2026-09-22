from typing import Any, Literal

from numpy.typing import NDArray

def create(code: int, number: int, dimension: int, base: int, level: int) -> NDArray[Any]:
    """Renders a coded design to a tensor at its side number, dimension, base and fractal level."""

def create_from_corners(filled: list[bytes], number: int, dimension: int, base: int, level: int) -> NDArray[Any]:
    """Renders a design straight from its filled residue corners."""

def create_named(spec: str, number: int, level: int) -> NDArray[Any]:
    """Renders a design from its canonical JSON name."""

def residue_corners(dimension: int, base: int) -> list[bytes]:
    """Returns every base-q residue corner of a dimension in row-major order."""

def total_codes(dimension: int, base: int) -> int:
    """Returns the code count of a dimension and base, two to the number of corners."""
