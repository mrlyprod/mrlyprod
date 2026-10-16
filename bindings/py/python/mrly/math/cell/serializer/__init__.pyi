from typing import Any, Literal

from numpy.typing import NDArray

def byte_cube(value: Any) -> list[list[bytes]]:
    """Reads a triply nested JSON array into layers of byte rows."""

def byte_grid(value: Any) -> list[bytes]:
    """Reads a nested JSON array into rows of bytes."""

def color_grid(value: Any) -> list[NDArray[Any]]:
    """Reads a nested JSON array into rows of four-channel colors."""

def count_cube(value: Any) -> list[int]:
    """Reads a triply nested JSON array of counts into one flat run; a count must fit in thirty-two bits."""

def count_grid(value: Any) -> list[int]:
    """Reads a nested JSON array of counts into one flat run; a count must fit in thirty-two bits."""

def parse(text: str) -> Any:
    """Parses JSON text into a value tree."""

def tag_layer(counts: list[int], shape: list[int]) -> NDArray[Any]:
    """Packs a flat run of counts into a tensor of the shape, at the narrowest dtype that holds them."""

def types_field(data: Any) -> Any:
    """Returns the types field of the data."""
