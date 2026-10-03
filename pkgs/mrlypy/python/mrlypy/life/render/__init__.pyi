from typing import Any, Literal

from numpy.typing import NDArray

def frame(grid: dict[str, Any], scale: int) -> NDArray[Any]:
    """Renders one grid to a white-on-black image at a pixel scale."""
