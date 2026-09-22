from typing import Any, Literal

from numpy.typing import NDArray
import mrlypy.core

def below(rng: mrlypy.core.Rng, n: int) -> int:
    """Draws an integer below n, or zero when n is zero."""

def boolean(rng: mrlypy.core.Rng) -> bool:
    """Draws a fair coin flip."""

def chance(rng: mrlypy.core.Rng, p: float) -> bool:
    """Returns true with probability p."""

def new(seed: int) -> mrlypy.core.Rng:
    """Builds the stream from a seed."""

def range(rng: mrlypy.core.Rng, lo: int, hi: int) -> int:
    """Draws an integer between lo and hi inclusive, or lo when hi is not above lo."""

def sample_indices(rng: mrlypy.core.Rng, length: int, amount: int) -> list[int]:
    """Draws amount distinct indices below length, or every index when amount is larger."""

def unit(rng: mrlypy.core.Rng) -> float:
    """Draws a float at or above zero and below one."""
