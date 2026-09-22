from typing import Any, Literal

from numpy.typing import NDArray

def is_balanced(code: int, n: int) -> bool:
    """Reports whether the packed function outputs one on exactly half of its inputs."""

def nonlinearity(code: int, n: int) -> int:
    """Returns how far the packed function sits from every affine function, zero when it is one."""

def sac(code: int, n: int) -> float:
    """Returns the mean chance that flipping one input bit flips the output, 0.5 at full avalanche."""

def walsh_spectrum(code: int, n: int) -> list[int]:
    """Returns the Walsh spectrum of an n-input boolean function packed as a truth-table code."""
