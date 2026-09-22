from typing import Any, Literal

from numpy.typing import NDArray
import mrlypy.core
from . import payload

def capacity(cell: dict[str, Any]) -> int:
    """Returns the payload bytes the cell's filled sites can hold, its length header paid for."""

def carpet(number: int, level: int) -> dict[str, Any]:
    """Builds the carpet fractal, its seed pierced at every odd-odd site, deepened to the level."""

def census(cell: dict[str, Any]) -> dict[str, Any]:
    """Takes the cell's full census in one reading."""

def create(code: int, number: int, level: int, rotation: int, base: int) -> dict[str, Any]:
    """Builds the design a universe code names, deepened to the level and rotated by quarter-turns."""

def dust(number: int, level: int) -> dict[str, Any]:
    """Builds the dust fractal, its seed on at every even-even site, deepened to the level."""

def embed(cell: dict[str, Any], payload: bytes) -> dict[str, Any]:
    """Writes the payload over the cell's filled sites, repeating it until every site is spoken for."""

def euler(cell: dict[str, Any]) -> int:
    """Returns the Euler characteristic of the filled sites, vertices less edges plus faces."""

def extract(carrier: dict[str, Any], carried: dict[str, Any]) -> bytes:
    """Reads the payload back, the plain cell naming the sites the carried one wrote over."""

def fills(cell: dict[str, Any]) -> int:
    """Counts the filled sites of the cell."""

def from_corners(corners: list[bytes], number: int, level: int, rotation: int, base: int) -> dict[str, Any]:
    """Builds the design straight from its filled residue corners, deepened to the level and rotated by quarter-turns."""

def from_json(text: str) -> dict[str, Any]:
    """Restores a cell from its JSON string, colors and tags included."""

def from_strings(rows: list[str]) -> dict[str, Any]:
    """Builds a cell from rows of digits, the inverse of the text rendering."""

def hline(number: int, level: int) -> dict[str, Any]:
    """Builds the hline fractal, its seed striped along odd rows, deepened to the level."""

def htree(number: int, level: int) -> dict[str, Any]:
    """Builds the htree fractal, its seed striped along even rows, deepened to the level."""

def level_set(number: int, levels: list[int], level: int, rotation: int, base: int) -> dict[str, Any]:
    """Builds the level-set design, filling every residue corner whose digits sum to a named level."""

def mask(mask: NDArray[Any], shape: list[int]) -> NDArray[Any]:
    """Tiles the mask over the shape and crops it, the perforation pattern itself."""

def merge(cells: list[dict[str, Any]], width: int, height: int) -> dict[str, Any]:
    """Merges same-shaped cells into one block of the given width and height in cells, colors and tags kept."""

def named(design: Literal["Carpet", "Net", "Htree", "Vtree", "Void", "Xtree", "Ytree", "Ztree", "Point", "Dust", "Hline", "Vline", "Star", "Xline", "Yline", "Zline"], number: int, level: int, rotation: int) -> dict[str, Any]:
    """Builds the design the name picks, deepened to the level and rotated by quarter-turns."""

def net(number: int, level: int) -> dict[str, Any]:
    """Builds the net fractal, its seed on wherever a coordinate is odd, deepened to the level."""

def noise(number: int, level: int, density: float, rng: mrlypy.core.Rng) -> dict[str, Any]:
    """Builds a random cell, each seed site drawn on with probability density, deepened to the level."""

def ones(number: int, level: int) -> dict[str, Any]:
    """Builds an all-filled cell of the given size and level."""

def perimeter(cell: dict[str, Any]) -> int:
    """Counts the faces of filled sites open to emptiness or the border."""

def png(cell: dict[str, Any], scale: int, outline: tuple[int, int, int, int] | None, width: int, shape: Literal["Square", "Circle", "Diamond"]) -> bytes:
    """Renders the cell to PNG bytes at the given pixel scale, stroked and padded when an outline is given."""

def point(number: int, level: int) -> dict[str, Any]:
    """Builds the point fractal, its seed on at every odd-odd site, deepened to the level."""

def read(sheet: dict[str, Any], carrier: dict[str, Any]) -> bytes:
    """Reads the payload back from a framed sheet, the plain fourth cell naming the sites."""

def sheet(cells: list[dict[str, Any]], payload: bytes) -> dict[str, Any]:
    """Builds the framed sheet of four same-sized cells, the fourth carrying the payload."""

def special(mask: NDArray[Any], cell: dict[str, Any]) -> dict[str, Any]:
    """Tiles quarter-turned copies of the cell as the 2d mask directs."""

def star(number: int, level: int) -> dict[str, Any]:
    """Builds the star fractal, its seed on where exactly one coordinate is odd, deepened to the level."""

def svg(cell: dict[str, Any], scale: int, outline: tuple[int, int, int, int] | None, width: int, shape: Literal["Square", "Circle", "Diamond"]) -> str:
    """Renders the cell to an SVG string at the given scale, stroked and padded when an outline is given."""

def text(cell: dict[str, Any], glyphs: dict[int, str] | None = None) -> list[str]:
    """Renders the cell as rows of glyphs, or of digits where no glyph is mapped."""

def to_3d(cell: dict[str, Any]) -> dict[str, Any]:
    """Lifts the flat cell into a cube one site deep, colors and tags with it."""

def to_json(cell: dict[str, Any]) -> str:
    """Serializes the cell to a JSON string of its types, with colors and tags when present."""

def vline(number: int, level: int) -> dict[str, Any]:
    """Builds the vline fractal, its seed striped along odd columns, deepened to the level."""

def void(number: int, level: int) -> dict[str, Any]:
    """Builds the void fractal, its seed a checkerboard on even parity, deepened to the level."""

def voids(cell: dict[str, Any]) -> int:
    """Counts the empty sites of the cell."""

def vtree(number: int, level: int) -> dict[str, Any]:
    """Builds the vtree fractal, its seed striped along even columns, deepened to the level."""

def zeros(number: int, level: int) -> dict[str, Any]:
    """Builds an all-empty cell of the given size and level."""
