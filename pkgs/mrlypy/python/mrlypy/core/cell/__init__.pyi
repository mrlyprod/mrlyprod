from typing import Any, Literal

from numpy.typing import NDArray
import mrlypy.core

def anti(cell: dict[str, Any]) -> dict[str, Any]:
    """Flips every type to one minus itself, same as invert."""

def binarize(cell: dict[str, Any], threshold: int) -> dict[str, Any]:
    """Maps every type to one at or above the threshold and zero below, dropping colors."""

def binarize_otsu(cell: dict[str, Any]) -> dict[str, Any]:
    """Binarizes the types at Otsu's threshold, dropping colors."""

def blur(cell: dict[str, Any], mask: NDArray[Any], wrap: bool) -> dict[str, Any]:
    """Replaces every type with the rounded mean of its masked neighborhood, dropping colors."""

def color_at(cell: dict[str, Any], flat: int) -> tuple[int, int, int, int]:
    """Returns the painted color at a flat index, or transparent while unpainted or past the end."""

def combine(cell: dict[str, Any], other: dict[str, Any]) -> dict[str, Any]:
    """Builds the Kronecker product of the two cells' types."""

def fractal(cell: dict[str, Any], level: int) -> dict[str, Any]:
    """Grows the types to the level-fold Kronecker power of themselves, dropping colors and tags."""

def invert(cell: dict[str, Any]) -> dict[str, Any]:
    """Flips every type to one minus itself."""

def layers(cell: dict[str, Any], dtype: Literal["U8", "U16", "U32", "I32"]) -> dict[str, Any]:
    """Tags every cell with its concentric shell distance from the center."""

def magic(cells: list[dict[str, Any]]) -> dict[str, Any]:
    """Folds at least two cells into one by chained Kronecker products."""

def mapping() -> dict[int, list[tuple[int, int, int, int]]]:
    """Returns the default mapping of the first six types to white, black, alpha, red, green, and blue."""

def merge(cells: list[dict[str, Any]], reps: list[int]) -> dict[str, Any]:
    """Stitches same-shaped cells into one grid of reps blocks per axis."""

def moore(dimension: int) -> NDArray[Any]:
    """Builds the 3-wide Moore mask of the dimension, every site on but the center."""

def mosaic(mask: NDArray[Any], cells: list[dict[str, Any]]) -> dict[str, Any]:
    """Lays the cell each mask entry indexes into that entry's place and merges the lot."""

def neighbors(cell: dict[str, Any], mask: NDArray[Any], target: int, wrap: bool, dtype: Literal["U8", "U16", "U32", "I32"]) -> dict[str, Any]:
    """Tags every cell with its count of target-valued neighbors under the mask."""

def new(types: NDArray[Any]) -> dict[str, Any]:
    """Wraps a tensor of types in a bare cell, colorless and tagless."""

def pad(cell: dict[str, Any], count: int, value: int) -> dict[str, Any]:
    """Wraps the cell in count layers of value on every side, dropping colors."""

def paint(cell: dict[str, Any], mapping: dict[int, list[tuple[int, int, int, int]]], mode: Literal["Type", "Tag", "Index", "Enumerate", "Random", "Row", "Column", "Depth"], rng: mrlypy.core.Rng | None = None) -> dict[str, Any]:
    """Colors every mapped cell, picking within each type's palette by the mode."""

def perforate(cell: dict[str, Any], mask: NDArray[Any], value: int) -> dict[str, Any]:
    """Stamps value wherever the tiled mask is on, dropping colors."""

def remap(cell: dict[str, Any], map: list[int], shape: list[int]) -> dict[str, Any]:
    """Rebuilds a cell's types, colors and tags at the new shape from one destination-to-source index map."""

def rgba(cell: dict[str, Any]) -> bytes:
    """Returns the flat rgba bytes of the cells, four to a cell, the stored color where there is one and opaque black everywhere else."""

def rot90_map(shape: list[int], k: int, axes: tuple[int, int]) -> list[int]:
    """Builds the flat source index of every destination cell after k quarter turns in the plane of the axes."""

def rotate(cell: dict[str, Any], k: int, axes: tuple[int, int]) -> dict[str, Any]:
    """Rotates the cell k quarter turns in the plane of the given axes, carrying colors and tags along."""

def shape(cell: dict[str, Any]) -> list[int]:
    """Returns the shape of the type tensor."""

def size(cell: dict[str, Any]) -> int:
    """Returns the number of cells."""

def tile(cell: dict[str, Any], reps: list[int]) -> dict[str, Any]:
    """Repeats the cell reps times along each axis, carrying colors and tags along."""

def tile_map(shape: list[int], reps: list[int]) -> list[int]:
    """Builds the flat source index of every destination cell after tiling reps copies per axis."""
