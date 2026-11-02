from typing import Any, Literal

from numpy.typing import NDArray

GASKET_LEVELS: int

def gasket(level: int) -> int:
    """Counts the coprime points of the Sierpinski gasket at a level: the pairs `0 <= i, j < 2^level` with `i AND j = 0` and `gcd(i, j) = 1`, the sequence [A396934](https://oeis.org/A396934)."""
