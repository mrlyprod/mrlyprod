from typing import Any, Literal

from numpy.typing import NDArray
import mrlypy.core

def anti(cell: dict[str, Any]) -> dict[str, Any]:
    """Inverts the cell."""

def binarize(cell: dict[str, Any], threshold: int) -> dict[str, Any]:
    """Maps each site to one at or above the threshold, zero below."""

def binarize_otsu(cell: dict[str, Any]) -> dict[str, Any]:
    """Binarizes the cell at the threshold Otsu's method picks."""

def blur(cell: dict[str, Any], mask: NDArray[Any], wrap: bool) -> dict[str, Any]:
    """Rounds each site to the mean of its masked neighborhood, wrapping on request."""

def combine(cell: dict[str, Any], other: dict[str, Any]) -> dict[str, Any]:
    """Returns the Kronecker product of the two cells."""

def counting_dtype(mask: NDArray[Any]) -> Literal["U8", "U16", "U32", "I32"]:
    """Returns the narrowest count dtype that fits the mask's popcount."""

def depth(cell: dict[str, Any]) -> int:
    """Returns the size of axis 0, the cube's leading axis."""

def dtype_for(peak: int) -> Literal["U8", "U16", "U32", "I32"]:
    """Returns the narrowest unsigned dtype that holds the peak value."""

def fractal(cell: dict[str, Any], level: int) -> dict[str, Any]:
    """Deepens the cell into its level-fold fractal."""

def height(cell: dict[str, Any]) -> int:
    """Returns the size of the axis before the last."""

def invert(cell: dict[str, Any]) -> dict[str, Any]:
    """Swaps filled and empty sites."""

def layers(cell: dict[str, Any]) -> dict[str, Any]:
    """Tags each site with its ring distance from the center."""

def neighbors(cell: dict[str, Any], mask: NDArray[Any], target: int, wrap: bool) -> dict[str, Any]:
    """Tags each site with its count of masked neighbors matching the target, wrapping on request."""

def new(types: NDArray[Any]) -> dict[str, Any]:
    """Builds a cell from an N-dimensional tensor of types."""

def orient(cell: dict[str, Any], index: int) -> dict[str, Any]:
    """Turns the cell into one of the 24 cube orientations."""

def pad(cell: dict[str, Any], count: int, value: int) -> dict[str, Any]:
    """Wraps the cell in count layers of the given value on every side."""

def paint(cell: dict[str, Any], mapping: dict[int, list[tuple[int, int, int, int]]], mode: Literal["Type", "Tag", "Index", "Enumerate", "Random", "Row", "Column", "Depth"], rng: mrlypy.core.Rng | None = None) -> dict[str, Any]:
    """Colors each site by its type through the mapping in the given mode."""

def perforate(cell: dict[str, Any], mask: NDArray[Any], value: int) -> dict[str, Any]:
    """Writes the value wherever the tiled mask is nonzero."""

def rotate(cell: dict[str, Any], k: int, axes: tuple[int, int] | None = None) -> dict[str, Any]:
    """Rotates the cell k quarter turns in the plane.
    
    Rotates the cell k quarter turns about the given pair of axes."""

def tile(cell: dict[str, Any], width: int, height: int, depth: int | None = None) -> dict[str, Any]:
    """Repeats the cell into a width-by-height array of copies.
    
    Repeats the cell into a width-by-height-by-depth array of copies."""

def types(cell: dict[str, Any]) -> NDArray[Any]:
    """Returns the tensor of types."""

def width(cell: dict[str, Any]) -> int:
    """Returns the size of the last axis."""
