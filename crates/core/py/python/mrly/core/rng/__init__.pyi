from typing import Any, Literal

from numpy.typing import NDArray
import mrly.core

def below(rng: mrly.core.Rng, n: int) -> int:
    """Draws an integer below n, or zero when n is zero."""

def boolean(rng: mrly.core.Rng) -> bool:
    """Draws a fair coin flip."""

def chance(rng: mrly.core.Rng, p: float) -> bool:
    """Returns true with probability p."""

def new(seed: int) -> mrly.core.Rng:
    """Builds the stream from a seed."""

def range(rng: mrly.core.Rng, lo: int, hi: int) -> int:
    """Draws an integer between lo and hi inclusive, or lo when hi is not above lo."""

def sample_indices(rng: mrly.core.Rng, length: int, amount: int) -> list[int]:
    """Draws amount distinct indices below length, or every index when amount is larger."""

def unit(rng: mrly.core.Rng) -> float:
    """Draws a float at or above zero and below one."""
