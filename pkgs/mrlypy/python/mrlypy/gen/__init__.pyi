from typing import Any, Literal

from numpy.typing import NDArray
import mrlypy.core
from . import build, name, recipe, variation

class Tile:
    """A complete recipe for one tile."""
    def __init__(self, group: Literal["General", "Fractal", "Magic", "Special", "Mosaic"]) -> None: ...
    @property
    def group(self) -> Literal["General", "Fractal", "Magic", "Special", "Mosaic"]:
        """The construction family."""
    @group.setter
    def group(self, value: Literal["General", "Fractal", "Magic", "Special", "Mosaic"]) -> None: ...
    @property
    def factor(self) -> int:
        """The base factor of the construction."""
    @factor.setter
    def factor(self, value: int) -> None: ...
    @property
    def sources(self) -> list[Any]:
        """The origin of each layer."""
    @sources.setter
    def sources(self, value: list[Any]) -> None: ...
    @property
    def numbers(self) -> list[int]:
        """The grid size of each source."""
    @numbers.setter
    def numbers(self, value: list[int]) -> None: ...
    @property
    def levels(self) -> list[int]:
        """The fractal level of each source."""
    @levels.setter
    def levels(self, value: list[int]) -> None: ...
    @property
    def rotations(self) -> list[int]:
        """The quarter-turn rotation of each source."""
    @rotations.setter
    def rotations(self, value: list[int]) -> None: ...
    @property
    def invert(self) -> bool:
        """Whether the finished tile inverts."""
    @invert.setter
    def invert(self, value: bool) -> None: ...
    @property
    def flip(self) -> bool:
        """Whether the finished tile flips."""
    @flip.setter
    def flip(self, value: bool) -> None: ...
    @property
    def width(self) -> int:
        """The tile's width in cells."""
    @width.setter
    def width(self, value: int) -> None: ...
    @property
    def height(self) -> int:
        """The tile's height in cells."""
    @height.setter
    def height(self, value: int) -> None: ...
    def check(self) -> None:
        """Checks that the slots, numbers and sizes agree."""
    def degenerate(self) -> bool:
        """Returns whether the recipe is a magic tile of one repeated source at one repeated number,
        the shape a fractal tile of the same factor and level already draws."""
    def max_size(self) -> int:
        """Returns the larger of width and height."""
    @staticmethod
    def new(group: Literal["General", "Fractal", "Magic", "Special", "Mosaic"]) -> Tile:
        """Builds an empty tile in a group."""
    def resize(self) -> None:
        """Recomputes the factor and side length the group and numbers imply, zero when they overflow."""
    def size(self, width: int, height: int) -> Tile:
        """Sets the tile's width and height."""
    @staticmethod
    def from_dict(data: Any) -> Tile:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

class Group:
    """The five construction families a tile can belong to."""
    @staticmethod
    def all() -> list[Literal["General", "Fractal", "Magic", "Special", "Mosaic"]]:
        """Returns every Group in canonical order."""

class Parity:
    """The parity filter over candidate sizes."""
    @staticmethod
    def all() -> list[Literal["Evens", "Odds", "Both"]]:
        """Returns every Parity in canonical order."""
    @staticmethod
    def keep(parity: Literal["Evens", "Odds", "Both"], n: int) -> bool:
        """Returns true when the number passes the filter."""

def background(seed: int, width: int, height: int) -> bytes:
    """Draws one seeded artwork and returns its PNG bytes: a random flat tile under the default recipe
    constraints and paint, repeated `width` across and `height` down at one pixel per cell."""

def classic_code(design: Literal["Carpet", "Net", "Htree", "Vtree", "Void", "Xtree", "Ytree", "Ztree", "Point", "Dust", "Hline", "Vline", "Star", "Xline", "Yline", "Zline"]) -> int | None:
    """Returns the plane's bang code of a classic design, or None for one outside the plane."""

def classic_code_nd(design: Literal["Carpet", "Net", "Htree", "Vtree", "Void", "Xtree", "Ytree", "Ztree", "Point", "Dust", "Hline", "Vline", "Star", "Xline", "Yline", "Zline"], dimension: int) -> int | None:
    """Returns the bang code of a named design in a dimension, or None where it has no design."""

def hex_key(length: int, rng: mrlypy.core.Rng) -> str:
    """Draws a hex key of the given length from the stream."""

def random_design(rng: mrlypy.core.Rng) -> Literal["Carpet", "Net", "Htree", "Vtree", "Void", "Xtree", "Ytree", "Ztree", "Point", "Dust", "Hline", "Vline", "Star", "Xline", "Yline", "Zline"]:
    """Draws one of the four flat classics from the stream: carpet, net, vertical tree or void."""

def random_rotation(design: Literal["Carpet", "Net", "Htree", "Vtree", "Void", "Xtree", "Ytree", "Ztree", "Point", "Dust", "Hline", "Vline", "Star", "Xline", "Yline", "Zline"], rng: mrlypy.core.Rng) -> int:
    """Draws a design's turn from the stream: a tree turns 0 or 1, every other design 0 to 3."""

def tree_mask(n: int) -> NDArray[Any]:
    """Builds the mask of a mosaic tile: the two trees of the side, two where they cross."""
