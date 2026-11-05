from typing import Any, Literal

from numpy.typing import NDArray

MOST: int
TABLE: int

def carry(prime: int, top: int, bottom: int) -> dict[str, Any]:
    ...

def columns(k: int, prime: int, top: int, bottom: int) -> list[dict[str, Any]]:
    ...

def count(prime: int, top: int, bottom: int, level: int) -> int:
    ...

def member(k: int, prime: int, top: int, bottom: int) -> bool:
    ...

def valuation(k: int, prime: int, top: int, bottom: int) -> int:
    ...

def witness(prime: int, top: int, bottom: int) -> tuple[int, int] | None:
    ...
