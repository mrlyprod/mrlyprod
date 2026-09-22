from typing import Any, Literal

from numpy.typing import NDArray
import mrlypy.life

def sequence(seq: mrlypy.life.Source, limit: int) -> list[int]:
    """Generates the sequence's values up to the limit."""
