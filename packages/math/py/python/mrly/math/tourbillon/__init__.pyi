from typing import Any, Literal

from numpy.typing import NDArray

def eyes(qmax: int) -> list[dict[str, Any]]:
    ...

def field(top: int, size: int, schedule: str, increment: float, set: str, weights: str, mode: str, blend: str, seed: int) -> list[float]:
    ...

def layers(top: int, schedule: str, increment: float, set: str, weights: str, seed: int) -> list[dict[str, Any]]:
    ...

def period(increment: float) -> int | None:
    ...

def sharing(list: list[dict[str, Any]]) -> tuple[int, int]:
    ...

def stack(list: list[dict[str, Any]], size: int, mode: str, blend: Literal["Mean", "Sum", "Union", "Meet", "Parity", "Difference"]) -> list[float]:
    ...

def stats(field: list[float], size: int, top: int, schedule: str, increment: float, set: str, weights: str, blend: str, seed: int) -> dict[str, Any]:
    ...
