from typing import Any, Literal

from numpy.typing import NDArray

def blur(pixels: NDArray[Any], width: int, height: int, radius: int) -> NDArray[Any]:
    """Box-blurs rgba pixels by radius, each channel the mean of its edge-padded window."""
