from typing import Any, Literal

from numpy.typing import NDArray

def kernel(mask: NDArray[Any], size: int) -> dict[str, Any]:
    """Draws a mask centred in a size-square picture, the mask centre at size over two."""

def next(cell: dict[str, Any], mask: NDArray[Any], birth: tuple[float, float], survive: tuple[float, float]) -> dict[str, Any]:
    """Advances a square grid one generation on a wrapped torus: a dead cell is born and a live one kept when its mask count over the mask budget lies in the closed birth or survive window, the count read by FFT convolution."""

def profile(cell: dict[str, Any]) -> dict[str, Any]:
    """Reads the ring means of a square 0/1 grid's log spectrum, its peak ring past the centre and that ring's wavelength in cells."""

def run(mask: NDArray[Any], birth: tuple[float, float], survive: tuple[float, float], size: int, steps: int, density: float, seed: int) -> dict[str, Any]:
    """Runs a seeded soup of the given density for the given steps on a size-square torus under the closed birth and survive windows."""

def spectrum(cell: dict[str, Any]) -> list[float]:
    """Returns the centred log magnitude spectrum of a square 0/1 grid, scaled to at most one by its peak."""
