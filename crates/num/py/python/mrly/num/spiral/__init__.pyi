from typing import Any, Literal

from numpy.typing import NDArray

class Growth:
    """Which cells of the square winding grow into a tile."""
    @staticmethod
    def all() -> list[Literal["Prime", "Every"]]:
        """Returns every Growth in canonical order."""

class Lattice:
    """The two lattices a spiral of the whole numbers is wound on, one at the centre and two to its right."""
    @staticmethod
    def all() -> list[Literal["Square", "Hex"]]:
        """Returns every Lattice in canonical order."""
    @staticmethod
    def count(lattice: Literal["Square", "Hex"], side: int) -> int:
        """Returns the count of numbers a sheet the odd side wide holds: the side squared, or the hexagon of that many cells across."""
    @staticmethod
    def n(lattice: Literal["Square", "Hex"], x: int, y: int) -> int:
        """Returns the number at a cell, one at the origin."""
    @staticmethod
    def radius(lattice: Literal["Square", "Hex"], side: int) -> int:
        """Returns the outermost ring of a sheet the odd side wide, half the side rounded down."""
    @staticmethod
    def ring(lattice: Literal["Square", "Hex"], n: int) -> int:
        """Returns the ring a number sits on, zero for one."""
    @staticmethod
    def ring_of(lattice: Literal["Square", "Hex"], x: int, y: int) -> int:
        """Returns the ring of a cell: the larger of the coordinates on the square, the hex distance on the hexagon."""
    @staticmethod
    def xy(lattice: Literal["Square", "Hex"], n: int) -> tuple[int, int]:
        """Returns the cell of a number: x right and y up on the square, axial q and r on the hexagon."""

class Mark:
    """What a cell is painted for."""
    @staticmethod
    def all() -> list[Literal["Prime", "Twin", "Squarefree", "Mobius"]]:
        """Returns every Mark in canonical order."""

def diagonal(lattice: Literal["Square", "Hex"], side: int, a: int, b: int, c: int) -> dict[str, Any]:
    """Reads the quadratic a k^2 + b k + c, a at least one, over the sheet the odd side wide: every value from one through the top, its cell, the prime hits and the opening streak."""

def level_of(n: int, base: int) -> int:
    """Returns the level of a number in a base, the count of its digits less one, so zero below the base and one at the base itself."""

def marks(mark: Literal["Prime", "Twin", "Squarefree", "Mobius"], limit: int) -> list[int]:
    """Marks every number from zero through the limit: one when marked, minus one for a Mobius value of minus one, else zero."""

def snail(base: int, top: int, growth: Literal["Prime", "Every"]) -> dict[str, Any]:
    """Winds one to the top on the square spiral and lays a square tile on every cell, the snail."""
