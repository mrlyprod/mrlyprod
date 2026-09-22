from typing import Any, Literal

from numpy.typing import NDArray

def all() -> list[tuple[str, list[str]]]:
    """Returns every pen in font order: uppers, lowers, digits, extras, specials."""
