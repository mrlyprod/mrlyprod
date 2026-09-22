from typing import Any, Literal

from numpy.typing import NDArray

class Base:
    """The base of a radix design: a ring and an element of norm at least two, the scale every word is read against."""
    def __init__(self, ring: Literal["Gaussian", "Eisenstein"], value: tuple[int, int]) -> None: ...
    def class_(self, z: tuple[int, int]) -> int:
        """Returns the index in the canonical residue system of the class of a point."""
    def congruent(self, z: tuple[int, int], w: tuple[int, int]) -> bool:
        """Returns whether two points are congruent modulo the base."""
    def group(self) -> list[list[int]]:
        """Returns the symmetry group of the base as permutations of the canonical residue indices: every unit multiplication, and every unit times conjugation when the conjugate of the base is an associate of the base."""
    def mirrored(self) -> bool:
        """Returns whether the conjugate of the base is an associate of the base, which is when the mirror joins the symmetry group."""
    @staticmethod
    def new(ring: Literal["Gaussian", "Eisenstein"], value: tuple[int, int]) -> Base:
        """Fixes a base in a ring."""
    def norm(self) -> int:
        """Returns the norm `q` of the base: the count of residue classes and the square of the scale."""
    def power(self, level: int) -> tuple[int, int]:
        """Returns the base raised to a level."""
    def residues(self) -> list[tuple[int, int]]:
        """Returns the canonical complete residue system modulo the base: the `q` representatives of least norm, ties broken by argument in `[0, 2 pi)`."""
    def ring(self) -> Literal["Gaussian", "Eisenstein"]:
        """Returns the ring."""
    def value(self) -> tuple[int, int]:
        """Returns the base element."""
    @staticmethod
    def from_dict(data: Any) -> Base:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

class Radix:
    """A radix design: a digit set inside one ring, placed by a base with a unit twist per digit."""
    def __init__(self, base: Base, digits: list[tuple[int, int]], twists: list[tuple[int, int]]) -> None: ...
    def base(self) -> Base:
        """Returns the base."""
    def canonical(self) -> bool:
        """Returns whether every digit is the canonical representative of its class."""
    def code(self) -> int:
        """Returns the code of the classes the digits occupy, which names the design only when the digits are the canonical representatives."""
    def digits(self) -> list[tuple[int, int]]:
        """Returns the digits."""
    def dimension(self) -> float:
        """Returns the similarity dimension `log |F| / log sqrt(q)`, the ratio of the digit count to the scale of the base."""
    def distinct(self, level: int) -> int:
        """Returns the count of distinct level-`L` points: the glue count, which is the fill exactly when no two words name one point."""
    def fill(self, level: int) -> int:
        """Returns the count of words of a level, `|F|^L`."""
    @staticmethod
    def from_code(base: Base, code: int) -> Radix:
        """Builds an untwisted design from a code over the canonical residue system, bit `i` of the code selecting residue `i`."""
    @staticmethod
    def new(base: Base, digits: list[tuple[int, int]], twists: list[tuple[int, int]]) -> Radix:
        """Builds a design from a base, a digit list and a unit twist per digit."""
    def plane(self, level: int) -> list[tuple[float, float]]:
        """Returns the level-`L` points in the plane, the scaled words divided by `b^L`."""
    def ring(self) -> Literal["Gaussian", "Eisenstein"]:
        """Returns the ring."""
    def size(self) -> int:
        """Returns the digit count `|F|`."""
    def twists(self) -> list[tuple[int, int]]:
        """Returns the twists."""
    def with_twists(self, units: list[int]) -> Radix:
        """Returns the design with the twists named by their index in the unit list, the units in turning order from one."""
    def words(self, level: int) -> list[tuple[int, int]]:
        """Returns the level-`L` points in exact ring coordinates scaled by `b^L`."""
    @staticmethod
    def from_dict(data: Any) -> Radix:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

def flowsnake() -> Radix:
    """Returns the flowsnake as a radix design: base `3 + omega` of norm seven on the hexagonal lattice, the full residue system, code `127`."""

def gasket() -> Radix:
    """Returns the Sierpinski gasket as a radix design: base `2` on the hexagonal lattice, three of the four residues, code `7`."""

def koch() -> Radix:
    """Returns the Koch curve as a radix design: base `3` on the hexagonal lattice, digits `0, 1, 2 + omega, 2`, twists `1, e^(i pi/3), e^(-i pi/3), 1`."""

def terdragon() -> Radix:
    """Returns the terdragon as a radix design: base `2 + omega` on the hexagonal lattice, the full residue system, code `7`, twisted by `1, omega, 1`."""

def tile(m: int, code: int) -> Radix:
    """Returns the plane design of a cell code as a radix design: base the rational integer `m`, of norm `m^2`, on the square lattice, no twist, digits the box residues `{x + y i : 0 <= x, y < m}`."""

def twindragon() -> Radix:
    """Returns the twindragon as a radix design: base `1 + i` on the square lattice, the full residue system, code `3`."""
