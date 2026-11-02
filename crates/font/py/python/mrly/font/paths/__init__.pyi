from typing import Any, Literal

from numpy.typing import NDArray

def penned(c: str) -> list[list[tuple[int, int]]] | None:
    """Returns the character's hand-penned strokes from the pen tables, or None outside the font."""
