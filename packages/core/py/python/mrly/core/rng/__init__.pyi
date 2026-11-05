from typing import Any, Literal

from numpy.typing import NDArray
import mrly.core

def below(rng: mrly.core.Rng, n: int) -> int:
    ...

def boolean(rng: mrly.core.Rng) -> bool:
    ...

def chance(rng: mrly.core.Rng, p: float) -> bool:
    ...

def new(seed: int) -> mrly.core.Rng:
    ...

def range(rng: mrly.core.Rng, lo: int, hi: int) -> int:
    ...

def sample_indices(rng: mrly.core.Rng, length: int, amount: int) -> list[int]:
    ...

def unit(rng: mrly.core.Rng) -> float:
    ...
