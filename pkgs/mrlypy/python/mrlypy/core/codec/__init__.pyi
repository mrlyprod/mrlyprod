from typing import Any, Literal

from numpy.typing import NDArray

def gif(frames: list[bytes], palette: NDArray[Any], width: int, height: int, scale: int, delay: int) -> bytes:
    """Encodes indexed frames as an animated gif89a, each source pixel a scale by scale block."""

def png(colors: NDArray[Any], width: int, height: int, scale: int) -> bytes:
    """Encodes rgba colors as a png, drawing each source pixel as a scale by scale block."""
