from typing import Any, Literal

from numpy.typing import NDArray

def correlation(m: int, n: int) -> float:
    """Returns the exact Pearson correlation of the flat carpet layers at two scales, area-weighted on their lcm grid."""

def sampled(m: int, n: int) -> float:
    """Returns the Pearson correlation of two rendered carpet layers on their lcm grid, sampled rather than integrated."""

def witness(scale: int) -> dict[str, Any]:
    """Puts an odd scale of three or more on trial against every earlier odd scale."""
