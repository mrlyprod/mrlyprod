from typing import Any, Literal

from numpy.typing import NDArray

COVER: float
EDGE: float

def deep(radius: float) -> float:
    ...

def dimension() -> float:
    ...

def distance(point: list[float]) -> float:
    ...

def exact(radius: float) -> float:
    ...

def profile(radius: float) -> float | None:
    ...

def reading(radius: float) -> float | None:
    ...

def tube(radius: float) -> float | None:
    ...

def volume(radius: float) -> float | None:
    ...
