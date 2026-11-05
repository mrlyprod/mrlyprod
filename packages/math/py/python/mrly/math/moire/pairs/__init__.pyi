from typing import Any, Literal

from numpy.typing import NDArray

def correlation(m: int, n: int) -> float:
    ...

def sampled(m: int, n: int) -> float:
    ...

def witness(scale: int) -> dict[str, Any]:
    ...
