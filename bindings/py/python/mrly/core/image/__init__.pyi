from typing import Any, Literal

from numpy.typing import NDArray

def blur(pixels: NDArray[Any], width: int, height: int, radius: int) -> NDArray[Any]:
    """Box-blurs rgba pixels by radius, each channel the mean of its edge-padded window."""

def new(width: int, height: int, colors: NDArray[Any]) -> NDArray[Any]:
    """Builds an image from its width, its height and its colors."""

def resample(image: NDArray[Any], width: int, height: int, filter: Literal["Nearest", "Linear", "Box"]) -> NDArray[Any]:
    """Resamples the image to a new size."""

def scale(image: NDArray[Any], scale: int) -> NDArray[Any]:
    """Draws every pixel as a scale by scale block, growing both sides by scale."""
