from typing import Any, Literal

from numpy.typing import NDArray
import mrlypy.core
import mrlypy.math.graph
from . import star

FILL: int
GRID: int
LEFT: int
RIGHT: int
UP: int
VOID: int

def anti(cell: dict[str, Any]) -> dict[str, Any]:
    """Swaps every fill triangle for a void and back."""

def binarize(cell: dict[str, Any], threshold: int) -> dict[str, Any]:
    """Maps each triangle to one at or above the threshold, zero below."""

def binarize_otsu(cell: dict[str, Any]) -> dict[str, Any]:
    """Binarizes the triangles at the threshold Otsu's method picks."""

def blank(radius: int, orient: Literal["Horizontal", "Vertical"], fill: int, void: int) -> dict[str, Any]:
    """Builds a hexagon of the given radius, fill inside and void outside."""

def blur(cell: dict[str, Any], mask: NDArray[Any], wrap: bool) -> dict[str, Any]:
    """Rounds each triangle to the mean of its masked neighborhood, wrapping on request."""

def census(cell: dict[str, Any], include_grid: bool) -> dict[str, Any]:
    """Tallies a cell's triangles, corners and edges, counting the backdrop only on request."""

def components(cell: dict[str, Any]) -> int:
    """Counts the connected pieces of the fill, triangles joined across shared edges."""

def cut(cell: dict[str, Any]) -> dict[str, Any]:
    """Slices a cube through its center across the main diagonal into a hexagon."""

def cut_design(code: int, number: int, level: int, base: int) -> dict[str, Any]:
    """Builds the coded 3d design and slices its central hexagon."""

def east(x: int, y: int) -> list[tuple[int, int]]:
    """The three corners of the east-pointing triangle at the grid column and row."""

def euler(cell: dict[str, Any], include_grid: bool) -> int:
    """Returns the Euler characteristic of the cell's mesh, counting the backdrop only on request."""

def fills(cell: dict[str, Any]) -> int:
    """Counts the filled triangles of the cell."""

def fills_only(cell: dict[str, Any]) -> dict[str, Any]:
    """Tallies only the filled triangles, leaving the voids and the backdrop out of the mesh."""

def framed(cell: dict[str, Any]) -> dict[str, Any]:
    """Backs a cell onto a backdrop whose longer axis matches its orientation, leaving every triangle where it stood."""

def from_json(text: str) -> dict[str, Any]:
    """Parses a cell from JSON, defaulting any missing projection metadata."""

def giant(cell: dict[str, Any]) -> int:
    """Returns the triangle count of the fill's largest connected piece."""

def giant_network(cell: dict[str, Any]) -> mrlypy.math.graph.Network:
    """Returns the largest connected piece of the filled-triangle network as a network of its own."""

def height(cell: dict[str, Any]) -> int:
    """Returns the grid height in triangles."""

def holes(cell: dict[str, Any]) -> int:
    """Counts the holes of the fill, its piece count less the Euler number of the filled sub-mesh."""

def is_cube(cell: dict[str, Any]) -> bool:
    """Returns whether the cell's three sides are equal."""

def is_hex(cell: dict[str, Any]) -> bool:
    """Returns whether the cell's width, height and parity frame a hexagon."""

def iso(cell: dict[str, Any]) -> dict[str, Any]:
    """Projects a cube into the isometric hexagon of top, left and right faces."""

def iso_design(code: int, number: int, level: int, base: int) -> dict[str, Any]:
    """Builds the coded 3d design and projects it isometrically."""

def new(cell: dict[str, Any], projection: Literal["Iso", "Pro", "Cut"], orientation: Literal["Horizontal", "Vertical"], start: int) -> dict[str, Any]:
    """Builds a cell from its four parts."""

def north(x: int, y: int) -> list[tuple[int, int]]:
    """The three corners of the north-pointing triangle at the grid column and row."""

def orientation(width: int, height: int) -> Literal["Horizontal", "Vertical"]:
    """Returns the orientation a hexagon's width and height imply."""

def pad(cell: dict[str, Any], k: int, value: int) -> dict[str, Any]:
    """Wraps a hexagonal cell in k rings of the given value, carrying colors and tags along."""

def paint(cell: dict[str, Any], custom: dict[int, list[tuple[int, int, int, int]]] | None = None, mode: Literal["Type", "Tag", "Index", "Enumerate", "Random", "Row", "Column", "Depth"] | None = None, rng: mrlypy.core.Rng | None = None) -> dict[str, Any]:
    """Colors each triangle by its type through the custom or default mapping in the given or type mode."""

def perforate(cell: dict[str, Any], mask: NDArray[Any], value: int) -> dict[str, Any]:
    """Writes the value wherever the tiled mask is nonzero."""

def png(cell: dict[str, Any], scale: int, outline: tuple[int, int, int, int] | None, width: int) -> bytes:
    """Rasters a cell's triangles to PNG bytes at the given scale, stroked and padded when an outline is given."""

def pro(cell: dict[str, Any]) -> dict[str, Any]:
    """Projects a cube's three facing sides into a hexagon of fills and voids."""

def pro_design(code: int, number: int, level: int, base: int) -> dict[str, Any]:
    """Builds the coded 3d design and projects its facing sides."""

def radial(cell: dict[str, Any], radius: int) -> dict[str, Any]:
    """Tessellates a hexagonal cell over the disc mask of the given radius."""

def radial_crop(cell: dict[str, Any], radius: int, size: tuple[int, int]) -> dict[str, Any]:
    """Crops the interlocking overhang off a disc tiled at the given radius and tile size."""

def radial_mask(radius: int, orient: Literal["Horizontal", "Vertical"]) -> NDArray[Any]:
    """Builds the disc mask of cells within hex distance radius of the center."""

def raster(cell: dict[str, Any], size: int) -> list[float]:
    """Rasterizes a hex cell's fills on a square of the side at the true hex aspect, one for a fill triangle and zero elsewhere."""

def rect_png(cell: dict[str, Any], scale: int, start: int | None = None) -> bytes:
    """Rasters the hexagon tiled three by three and cropped to one interlocking rectangle to PNG bytes."""

def rect_svg(cell: dict[str, Any], scale: int, start: int | None = None) -> str:
    """Renders the hexagon tiled three by three and cropped to one interlocking rectangle as an SVG string."""

def rim_holes(cell: dict[str, Any]) -> int:
    """Counts the void regions the rim never reaches, the second route to the hole count."""

def skin(cell: dict[str, Any]) -> dict[str, Any]:
    """Recodes an isometric projection's top, left and right faces as plain fills, so a census reads its visible skin as one figure."""

def slice_core_graph(cell: dict[str, Any]) -> mrlypy.math.graph.Network:
    """Builds the network of filled triangles joined by shared edges."""

def slice_dual_graph(cell: dict[str, Any]) -> mrlypy.math.graph.Network:
    """Builds the network of fill and void triangles joined by shared edges."""

def slice_edge_graph(cell: dict[str, Any], value: int | None = None) -> mrlypy.math.graph.Network:
    """Builds the corner-and-edge network of the triangles matching the value, or of every fill and void."""

def slice_tunnel_graph(cell: dict[str, Any]) -> mrlypy.math.graph.Network:
    """Builds the network of void triangles joined by shared edges, the pore network of the slice."""

def south(x: int, y: int) -> list[tuple[int, int]]:
    """The three corners of the south-pointing triangle at the grid column and row."""

def spectral_exponent(cell: dict[str, Any], window: float) -> float:
    """Reads the spectral dimension of the giant piece: twice the low-window log-log slope of the normalised Laplacian's integrated density of states."""

def svg(cell: dict[str, Any], scale: int, outline: tuple[int, int, int, int] | None, width: int, start: int | None = None) -> str:
    """Renders a cell's triangles to an SVG string at the given scale, stroked and padded when an outline is given."""

def tessellate(cell: dict[str, Any], mask: NDArray[Any]) -> dict[str, Any]:
    """Stamps a hexagonal cell at every set mask entry into one interlocking sheet, colors and tags included."""

def tile(cell: dict[str, Any], width: int, height: int) -> dict[str, Any]:
    """Tessellates a hexagonal cell over a full width-by-height mask."""

def tile_cell(cell: dict[str, Any], width: int, height: int, crop: bool) -> dict[str, Any]:
    """Tessellates a hexagon over a full width-by-height mask and returns the sheet as a projected cell, cropped to the interlocking rectangle on request."""

def tile_crop(cell: dict[str, Any], size: tuple[int, int]) -> dict[str, Any]:
    """Crops one interlocking step off each side of a sheet tiled at the given size."""

def tile_step(size: tuple[int, int]) -> tuple[int, int]:
    """Returns the interlocking step, in triangle columns and rows, that a sheet of hexagons of the given width and height loses off each side when cropped."""

def to_json(cell: dict[str, Any]) -> str:
    """Serializes a cell and its projection metadata to JSON."""

def triangles(cell: dict[str, Any], start: int | None = None) -> list[tuple[list[tuple[int, int]], tuple[int, int, int, int]]]:
    """Folds a cell into colored screen triangles, dropping the transparent ones, at the cell's start parity or the given override."""

def west(x: int, y: int) -> list[tuple[int, int]]:
    """The three corners of the west-pointing triangle at the grid column and row."""

def width(cell: dict[str, Any]) -> int:
    """Returns the grid width in triangles."""
