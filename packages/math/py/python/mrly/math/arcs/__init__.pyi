from typing import Any, Literal

from numpy.typing import NDArray

def curves(code: int, base: int, level: int) -> list[dict[str, Any]]:
    ...

def draw(code: int, base: int, level: int) -> dict[str, Any]:
    ...

def law(code: int, base: int, level: int) -> dict[str, Any] | None:
    ...

def trace(side: int, on: list[bool]) -> dict[str, Any]:
    ...
