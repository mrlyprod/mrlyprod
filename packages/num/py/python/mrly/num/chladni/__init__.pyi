from typing import Any, Literal

from numpy.typing import NDArray

def ladder(cap: int) -> list[dict[str, Any]]:
    ...

def lines(field: list[float], side: int) -> list[float]:
    ...

def plate(n: int, m: int, sign: int, size: int) -> list[float]:
    ...

def shake(field: list[float], side: int, grains: list[float], from_: int, steps: int, seed: int) -> list[float]:
    ...
