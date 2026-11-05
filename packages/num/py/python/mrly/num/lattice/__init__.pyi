from typing import Any, Literal

from numpy.typing import NDArray

def coprime_pairs(n: int) -> int:
    ...

def farey(order: int) -> list[dict[str, Any]]:
    ...

def grid(n: int) -> list[dict[str, Any]]:
    ...

def new_nodes(n: int) -> int:
    ...

def pi_estimate(n: int) -> float:
    ...

def recovered(n: int, dimension: int) -> float:
    ...

def visible_density(dimension: int) -> float:
    ...

def zeta_factor(dimension: int) -> float | None:
    ...

def zeta_whole(s: int) -> float:
    ...
