from typing import Any, Literal

from numpy.typing import NDArray

def eyes(qmax: int) -> list[dict[str, Any]]:
    """The angles a quarter turn shares with itself: ninety a over q for every q up to the cap and every a from zero to four q coprime to it, sorted by angle."""

def field(top: int, size: int, schedule: str, increment: float, set: str, weights: str, mode: str, blend: str, seed: int) -> list[float]:
    """Spins the odd parity carpets at the scales one, three, five up to the top into one stack on a square of the size, every layer turned about the centre by its own angle and masked to the inscribed disc, so every pixel sees every layer."""

def layers(top: int, schedule: str, increment: float, set: str, weights: str, seed: int) -> list[dict[str, Any]]:
    """The layers of a stack: every scale one, three, five up to the top the set keeps, each with its weight and its angle."""

def period(increment: float) -> int | None:
    """The least whole number of increments that closes a quarter turn, none once the count passes the cap."""

def sharing(list: list[dict[str, Any]]) -> tuple[int, int]:
    """The angle classes of a stack read a quarter turn apart: how many the layers fall in, and how many layer pairs share one."""

def stack(list: list[dict[str, Any]], size: int, mode: str, blend: Literal["Mean", "Sum", "Union", "Meet", "Parity", "Difference"]) -> list[float]:
    """Rasters the layers onto a square of the size, every one turned about the centre by its own angle and masked to the inscribed disc, then merged site by site."""

def stats(field: list[float], size: int, top: int, schedule: str, increment: float, set: str, weights: str, blend: str, seed: int) -> dict[str, Any]:
    """Reads a spun stack against the schedule that made it: the layer count, the first eight scales and angles, the mean and RMS contrast over the disc, that contrast times the root of the layer count, the exact centre value, whether the blend carries the weights, the span the raster covers and the brightest three sites."""
