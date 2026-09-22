from typing import Any, Literal

from numpy.typing import NDArray

def color(colorizer: Any, value: int, max: int) -> tuple[int, int, int, int]:
    """Returns the color for one value against the range maximum: the background at zero, the top of the ramp from the maximum up."""

def colors(colorizer: Any, values: list[int], max: int) -> NDArray[Any]:
    """Maps a slice of values to rgba pixels against the range maximum."""
