from typing import Any, Literal

from numpy.typing import NDArray

REFINE_LIMIT: int

class Frac:
    """An exact rational number with a positive, reduced denominator."""
    def __init__(self, num: int, den: int) -> None: ...
    @property
    def num(self) -> int:
        """The numerator, carrying the sign."""
    @num.setter
    def num(self, value: int) -> None: ...
    @property
    def den(self) -> int:
        """The denominator, always positive."""
    @den.setter
    def den(self, value: int) -> None: ...
    def minus(self, other: Frac) -> Frac:
        """Returns the exact difference."""
    @staticmethod
    def new(num: int, den: int) -> Frac:
        """Builds the reduced fraction num over den."""
    def plus(self, other: Frac) -> Frac:
        """Returns the exact sum."""
    def times(self, other: Frac) -> Frac:
        """Returns the exact product."""
    @staticmethod
    def whole(num: int) -> Frac:
        """Wraps an integer as a fraction over one."""
    @staticmethod
    def from_dict(data: Any) -> Frac:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

class Region:
    """Where one lattice cell sits relative to a shape."""
    @staticmethod
    def flip(region: Literal["Out", "Cut", "In"]) -> Literal["Out", "Cut", "In"]:
        """Swaps In and Out, keeping Cut."""

def census(shape: Any, types: NDArray[Any]) -> dict[str, Any]:
    """Tallies the design's cells and filled cells per region of the shape."""

def classify(shape: Any, side: int, index: list[int]) -> Literal["Out", "Cut", "In"]:
    """Places one lattice cell relative to the shape, exactly, with no floats."""

def crop(types: NDArray[Any], shape: Any, keep_cut: bool) -> NDArray[Any]:
    """Zeroes every cell of the design outside the shape, keeping Cut cells on request; anti-crop is Shape::Anti."""

def crossing_shell(radius: int, number: int, level: int) -> list[tuple[int, int]]:
    """Lists the level-`level` boxes the circle of radius `radius` crosses, in the arc's own order."""

def crossing_tree(radius: int, number: int, keep: list[bool]) -> dict[str, Any]:
    """Builds the whole crossing tree of one radius, pruned by the seats the design keeps."""

def named(name: str, dimension: int, radius: Frac) -> Any:
    """Builds a named shape of the dimension, centered at one half on every axis."""

def radial_census(types: NDArray[Any], centre: list[int], r_max: int) -> list[dict[str, Any]]:
    """Counts a design's filled cells against every integer radius about one centre, in exact integer arithmetic."""

def refine(types: NDArray[Any], shape: Any, base: int, extra: int, keep_cut: bool) -> NDArray[Any]:
    """Replicates each design cell base to the extra per axis and keeps a sub-cell only where its own region passes."""

def regions(shape: Any, dims: list[int]) -> NDArray[Any]:
    """Classifies every cell of the grid, packing Out, Cut and In as 0, 1 and 2; the first extent sets the lattice side."""

def shapes(dimension: int) -> list[str]:
    """Lists the named shapes of a dimension."""
