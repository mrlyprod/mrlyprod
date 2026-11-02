from typing import Any, Literal

from numpy.typing import NDArray

def output(rule: int, l: int, c: int, r: int) -> int:
    """Returns the bit a rule sends the neighbourhood to, reading bit `4l + 2c + r` in Wolfram's numbering off the low bit of each cell."""
