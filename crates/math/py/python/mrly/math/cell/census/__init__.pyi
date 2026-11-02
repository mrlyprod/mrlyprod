from typing import Any, Literal

from numpy.typing import NDArray

def edges(cell: dict[str, Any]) -> int:
    """Counts the distinct unit edges the filled sites carry, the edge graph's branches."""

def exposure(cell: dict[str, Any]) -> int:
    """Counts the faces of filled sites open to emptiness or the border."""

def fills(cell: dict[str, Any]) -> int:
    """Counts the filled sites of the cell."""

def vertices(cell: dict[str, Any]) -> int:
    """Counts the distinct corners the filled sites touch, the edge graph's nodes."""

def voids(cell: dict[str, Any]) -> int:
    """Counts the empty sites of the cell."""
