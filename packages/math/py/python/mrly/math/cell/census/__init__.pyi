from typing import Any, Literal

from numpy.typing import NDArray

def edges(cell: dict[str, Any]) -> int:
    ...

def exposure(cell: dict[str, Any]) -> int:
    ...

def fills(cell: dict[str, Any]) -> int:
    ...

def vertices(cell: dict[str, Any]) -> int:
    ...

def voids(cell: dict[str, Any]) -> int:
    ...
