from typing import Any, Literal

from numpy.typing import NDArray

def is_balanced(code: int, n: int) -> bool:
    ...

def nonlinearity(code: int, n: int) -> int:
    ...

def sac(code: int, n: int) -> float:
    ...

def walsh_spectrum(code: int, n: int) -> list[int]:
    ...
