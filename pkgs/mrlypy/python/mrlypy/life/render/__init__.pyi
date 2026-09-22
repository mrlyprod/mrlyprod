from typing import Any, Literal

from numpy.typing import NDArray

def frame(grid: dict[str, Any], scale: int) -> bytes:
    """Renders one grid to white-on-black PNG bytes at a pixel scale."""
