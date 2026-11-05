from typing import Any, Literal

from numpy.typing import NDArray

BAR_A: float
BAR_B: float
CAP_BASE: int
CERTIFIED: list[tuple[int, int]]
DIGIT_WALL: int
FIRST_BELOW: int
GAMMA: float
WINDOW_WALL: int

def cap(y: int) -> int:
    ...

def chain_exponent(base: int) -> float:
    ...

def chain_margin(base: int) -> float:
    ...

def chain_root(base: int) -> float:
    ...

def chain_wall() -> int:
    ...

def consecutive(digits: list[int]) -> bool:
    ...

def fraction(a: int, y: int, cap: int) -> tuple[int, int]:
    ...

def kappa(base: int, digits: list[int]) -> tuple[int, int]:
    ...

def masses(base: int, digits: list[int], level: int) -> list[float]:
    ...

def reach(base: int, missing: int) -> str:
    ...

def reading(base: int, fill: int, masses: list[float]) -> float:
    ...

def regions(base: int, level: int, z: int) -> list[Literal["A", "B", "C1", "C2"]]:
    ...

def tally(base: int, digits: list[int], level: int, samples: int) -> dict[str, Any]:
    ...

def weights(base: int, digits: list[int], level: int) -> list[float]:
    ...
