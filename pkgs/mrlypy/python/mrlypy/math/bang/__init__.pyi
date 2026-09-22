from typing import Any, Literal

from numpy.typing import NDArray
import mrlypy.math.name
from . import baseq, catalog, code, factory, universe, word

class Design:
    """A single design with its place in the orbit structure."""
    @property
    def i(self) -> int:
        """The design's code."""
    @property
    def dimension(self) -> int:
        """The design's dimension."""
    @property
    def canonical(self) -> bool:
        """Whether this code is the smallest in its orbit."""
    @property
    def class_rep(self) -> int:
        """The smallest code in the orbit."""
    @property
    def orbit_size(self) -> int:
        """The number of codes in the orbit."""
    def anf(self) -> str:
        """Returns the design's algebraic normal form as a string."""
    def degree(self) -> int:
        """Returns the design's algebraic degree, or -1 for the zero design."""
    def name(self) -> str:
        """Returns the design's name as a line of prose, `bang dim 2, code 7`."""
    def rule(self) -> list[bytes]:
        """Returns the design's filled corners in sorted order."""
    @staticmethod
    def from_dict(data: Any) -> Design:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

class Universe:
    """The complete enumeration of one dimension's designs and orbits."""
    def __init__(self, dimension: int) -> None: ...
    @property
    def dimension(self) -> int:
        """The universe's dimension."""
    @property
    def total(self) -> int:
        """The number of codes in the universe."""
    def all(self) -> list[Design]:
        """Returns every design in code order."""
    def canonical(self) -> list[Design]:
        """Returns the designs whose codes lead their orbits."""
    def design(self, code: int) -> Design:
        """Returns the design at a code with its precomputed orbit facts."""
    def distinct(self) -> int:
        """Returns the number of distinct orbits."""
    @staticmethod
    def new(dimension: int) -> Universe:
        """Enumerates every orbit of a dimension from 1 to 4."""
    @staticmethod
    def from_dict(data: Any) -> Universe:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

class MagicLayer:
    """One ordered layer of a magic composition: a coded design at its own side number."""
    @staticmethod
    def new(design: mrlypy.math.name.Bang, number: int) -> dict[str, Any]:
        """Pins a design to the side number it renders at."""

def bang(dimension: int) -> Universe:
    """Builds the universe of a dimension."""

def code_to_corners(code: int, dimension: int, base: int) -> list[bytes]:
    """Unpacks a code into its filled residue corners."""

def corners(dimension: int) -> list[bytes]:
    """Returns the binary corners of a dimension in code order."""

def corners_to_code(filled: list[bytes], dimension: int, base: int) -> int:
    """Packs filled residue corners back into their code."""

def levels_code(dimension: int, base: int, levels: list[int]) -> int:
    """Returns the code of the design filled wherever a corner's residue sum lands in the levels."""

def magic(layers: list[dict[str, Any]]) -> NDArray[Any]:
    """Composes the layers into one mixed-design cell by the ordered Kronecker product, first layer outermost."""

def magic_named(layers: list[tuple[str, int]]) -> NDArray[Any]:
    """Composes JSON-named layers in order."""

def sources(catalog: Any, dimension: int) -> list[Any]:
    """Builds the tile sources a catalog names at a dimension."""

def symmetries(dimension: int) -> list[tuple[list[int], bytes]]:
    """Returns the full symmetry group as axis permutations paired with flip patterns."""

def total_exposure(code: int, dimension: int) -> bool:
    """Returns whether no two filled corners of a code sit at Hamming distance one."""

def touches_every_corner(code: int, dimension: int) -> bool:
    """Returns whether a code fills the all-even corner, the rule that touches every grid corner at odd side."""

def universe_codes(dimension: int) -> list[int]:
    """Returns the canonical design codes of a dimension, computed once and cached for the process."""
