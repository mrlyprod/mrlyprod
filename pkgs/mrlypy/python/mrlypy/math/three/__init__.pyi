from typing import Any, Literal

from numpy.typing import NDArray
import mrlypy.core
import mrlypy.math.graph

class Vec3:
    """A three-component vector of f32."""
    def __init__(self, x: float, y: float, z: float) -> None: ...
    @property
    def x(self) -> float:
        """The x component."""
    @x.setter
    def x(self, value: float) -> None: ...
    @property
    def y(self) -> float:
        """The y component."""
    @y.setter
    def y(self, value: float) -> None: ...
    @property
    def z(self) -> float:
        """The z component."""
    @z.setter
    def z(self, value: float) -> None: ...
    def cross(self, o: Vec3) -> Vec3:
        """Returns the cross product, perpendicular to both vectors."""
    def dot(self, o: Vec3) -> float:
        """Returns the dot product of the two vectors."""
    @staticmethod
    def new(x: float, y: float, z: float) -> Vec3:
        """Builds a vector from its components."""
    def scale(self, s: float) -> Vec3:
        """Multiplies every component by the scalar."""
    @staticmethod
    def from_dict(data: Any) -> Vec3:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

def carpet(number: int, level: int) -> dict[str, Any]:
    """Builds the Menger sponge, filled where at most one coordinate is odd, at the given level."""

def census(cell: dict[str, Any]) -> dict[str, Any]:
    """Tallies a cell's sites, its exposed surface and its Euler characteristic in one reading."""

def core_graph(cell: dict[str, Any]) -> mrlypy.math.graph.Network:
    """Extracts the network of filled sites joined to their axis neighbors."""

def create(code: int, number: int, level: int, base: int) -> dict[str, Any]:
    """Builds the cube the universe code names, deepened to the given fractal level."""

def diagonal_slice(code: int, number: int, level: int, base: int, height: int) -> list[list[int]]:
    """Lists the filled cells on the diagonal plane `x + y + z = height`, as `x, y, z` triples."""

def diagonal_svg(code: int, number: int, level: int, base: int, heights: list[int], scale: int) -> str:
    """Draws the given diagonal slices as one circle per cell, coloured by height slot and top-scale corner."""

def dust(number: int, level: int) -> dict[str, Any]:
    """Builds the dust cube, filled where every coordinate is even, at the given level."""

def edge_graph(cell: dict[str, Any]) -> mrlypy.math.graph.Network:
    """Extracts the network of corners and edges outlining every filled site."""

def euler(cell: dict[str, Any]) -> int:
    """Returns the Euler characteristic of the filled complex, vertices less edges plus faces less sites."""

def extrude(cell: dict[str, Any], axis: int, depth: int) -> dict[str, Any]:
    """Lifts a flat cell into a cube by repeating it depth times along a new axis, colors and tags with it."""

def extrude_cube(cell: dict[str, Any], depth: int) -> dict[str, Any]:
    """Repeats every plane of a cube depth times along its leading axis, colors and tags with it."""

def faces(cell: dict[str, Any]) -> int:
    """Returns the count of unit faces the filled sites touch, a face shared by two sites counted once."""

def fills(cell: dict[str, Any]) -> int:
    """Returns the count of filled sites."""

def from_corners(corners: list[bytes], number: int, level: int, base: int) -> dict[str, Any]:
    """Builds a cube from its corner patterns, deepened to the given fractal level."""

def from_json(text: str) -> dict[str, Any]:
    """Parses a cell from its JSON, colors and tags included."""

def from_strings(data: list[list[str]]) -> dict[str, Any]:
    """Builds a cube from one string of digits per row, grouped plane by plane."""

def hidden(cell: dict[str, Any]) -> int:
    """Returns the count of faces buried between two filled sites, six per site less the exposed surface."""

def level_set(number: int, levels: list[int], level: int, base: int) -> dict[str, Any]:
    """Builds the cube filled wherever the residue sum lands in the levels, at the given level."""

def magic(cells: list[dict[str, Any]]) -> dict[str, Any]:
    """Folds two or more cells into one by chained Kronecker combination."""

def manhattan_layers(cell: dict[str, Any]) -> dict[str, Any]:
    """Tags every site with its Manhattan distance from the cube's center, the diamond shells."""

def merge(cells: list[dict[str, Any]], width: int, height: int, depth: int) -> dict[str, Any]:
    """Merges the cells into one cube arranged width by height by depth."""

def mosaic(mask: NDArray[Any], cells: list[dict[str, Any]]) -> dict[str, Any]:
    """Builds a cell by placing at each mask site the cell its value indexes."""

def named(design: Literal["Carpet", "Net", "Htree", "Vtree", "Void", "Xtree", "Ytree", "Ztree", "Point", "Dust", "Hline", "Vline", "Star", "Xline", "Yline", "Zline"], number: int, level: int) -> dict[str, Any]:
    """Builds the cube the name picks, deepened to the given fractal level."""

def net(number: int, level: int) -> dict[str, Any]:
    """Builds the net cube, filled where at least two coordinates are odd, at the given level."""

def noise(number: int, level: int, density: float, rng: mrlypy.core.Rng) -> dict[str, Any]:
    """Builds a cube whose every site turns on with probability density, at the given level."""

def ones(number: int, level: int) -> dict[str, Any]:
    """Builds the solid cube at the given size and level."""

def orientations() -> list[tuple[int, int, int]]:
    """Returns the 24 rotation triples that reach each distinct cube orientation."""

def point(number: int, level: int) -> dict[str, Any]:
    """Builds the point cube, filled where every coordinate is odd, at the given level."""

def profile(code: int, number: int, level: int, base: int) -> list[int]:
    """Counts the filled cells on every diagonal plane `x + y + z = s`, for `s` in `0..=3*(side - 1)`."""

def project(point: list[int]) -> tuple[float, float]:
    """Projects a cell down the `(1,1,1)` axis: `u = (x - y)/sqrt 2`, `v = (x + y - 2z)/sqrt 6`."""

def quads(cell: dict[str, Any]) -> list[dict[str, Any]]:
    """Returns one outward quad per exposed face, scaled into the unit box."""

def shadow(point: list[int]) -> tuple[int, int]:
    """Returns the integer shadow `(x - y, x + y - 2z)`, the projection with its irrational scales dropped."""

def slice(cell: dict[str, Any], axis: int, index: int) -> dict[str, Any]:
    """Takes the flat cell left when one axis of the cube is fixed at an index, colors and tags with it."""

def special(mask: NDArray[Any], cell: dict[str, Any]) -> dict[str, Any]:
    """Orients a copy of the cell by each mask value and merges them in the mask's shape."""

def star(number: int, level: int) -> dict[str, Any]:
    """Builds the star cube, filled where exactly one coordinate is odd, at the given level."""

def support(counts: list[int]) -> tuple[int, int] | None:
    """Returns the first and last height a profile fills, or none when the design is empty."""

def surface(cell: dict[str, Any]) -> int:
    """Returns the count of filled faces exposed to void or the outside."""

def text(cell: dict[str, Any], glyphs: dict[int, str] | None = None) -> list[str]:
    """Renders the cube as rows of glyphs, plane after plane, or of digits where no glyph is mapped."""

def to_json(cell: dict[str, Any]) -> str:
    """Serializes the cell's shape and types to JSON, with colors and tags when present."""

def to_obj(cell: dict[str, Any]) -> str:
    """Writes the cube's exposed quads as a Wavefront OBJ, one shared vertex per corner."""

def to_strings(cell: dict[str, Any]) -> list[list[str]]:
    """Unrolls the cube into one string of digits per row, grouped plane by plane."""

def tunnel_graph(cell: dict[str, Any]) -> mrlypy.math.graph.Network:
    """Extracts the network of empty sites joined to their axis neighbors."""

def void(number: int, level: int) -> dict[str, Any]:
    """Builds the checkerboard cube, filled where all coordinate parities agree, at the given level."""

def voids(cell: dict[str, Any]) -> int:
    """Returns the count of empty sites."""

def volume(cell: dict[str, Any]) -> int:
    """Returns the filled-site count, the cube's volume."""

def wires(cell: dict[str, Any]) -> list[list[Vec3]]:
    """Returns the cell's edge-graph segments, scaled into the unit box."""

def xline(number: int, level: int) -> dict[str, Any]:
    """Builds the cube of rods along the x axis at the given size and level."""

def xtree(number: int, level: int) -> dict[str, Any]:
    """Builds the cube of beams along the x axis at the given size and level."""

def yline(number: int, level: int) -> dict[str, Any]:
    """Builds the cube of rods along the y axis at the given size and level."""

def ytree(number: int, level: int) -> dict[str, Any]:
    """Builds the cube of beams along the y axis at the given size and level."""

def zeros(number: int, level: int) -> dict[str, Any]:
    """Builds the all-void cube at the given size and level."""

def zline(number: int, level: int) -> dict[str, Any]:
    """Builds the cube of rods along the z axis at the given size and level."""

def ztree(number: int, level: int) -> dict[str, Any]:
    """Builds the cube of beams along the z axis at the given size and level."""
