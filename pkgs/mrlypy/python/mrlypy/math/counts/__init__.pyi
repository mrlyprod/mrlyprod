from typing import Any, Literal

from numpy.typing import NDArray
from . import diagonal, ladder, six

class Exposure:
    """The counts the exposure recurrence runs on: the filled cells and exposed faces of the tile, and per axis its adjacent pairs and spanning positions."""
    @property
    def occupancy(self) -> int:
        """The filled cells of the tile."""
    @property
    def exposed(self) -> int:
        """The exposed faces of the tile."""
    @property
    def axes(self) -> list[tuple[int, int]]:
        """Per axis, the adjacent filled pairs and the spanning positions."""
    def at(self, level: int) -> int | None:
        """Returns the exposed faces of the level-fold Kronecker power, or none past a u128."""
    @staticmethod
    def from_corners(filled: list[bytes], number: int, dimension: int, base: int) -> Exposure:
        """Folds the counts from the filled residue corners at a side number, without rendering the tile."""
    @staticmethod
    def of_tile(tile: NDArray[Any]) -> Exposure:
        """Reads the counts off a rendered tile."""
    def recurrence(self) -> list[int]:
        """Returns the coefficients `c` of the recurrence `a(L) = c[0] a(L-1) + c[1] a(L-2) + ...` the exposure obeys."""
    @staticmethod
    def from_dict(data: Any) -> Exposure:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

def centered_hexagonal(m: int) -> int:
    """Returns the centered hexagonal number at the index, the lattice points of a hexagon of side m-1."""

def cut_fills(code: int, number: int, level: int) -> int:
    """Returns the filled triangle count of the code's cut section at the given level, without rendering it."""

def cut_voids(code: int, number: int, level: int) -> int:
    """Returns the empty triangle count of the code's cut section at the given level."""

def dimension(code: int, number: int, base_dimension: int, base: int) -> float:
    """Returns the code's fractal dimension, the log of its one-level fill over the log of number."""

def edges_of_tile(tile: NDArray[Any], level: int) -> int | None:
    """Returns the branch count of the tile's level-fold Kronecker power, fitted to its two-term
    recurrence over four powers, or none past a u128."""

def exposure(code: int, number: int, dimension: int, level: int, base: int) -> int:
    """Returns the exposed face count of the code's fractal in any dimension at the given level, folded from its corners."""

def exposure_of_tile(tile: NDArray[Any], level: int) -> int | None:
    """Returns the exposed face count of the tile's level-fold Kronecker power in closed form, or none past a u128."""

def exposure_recurrence(tile: NDArray[Any]) -> list[int]:
    """Returns the coefficients of the recurrence the tile's exposure obeys."""

def fill(code: int, number: int, dimension: int, level: int, base: int) -> int:
    """Returns the filled cell count of the code's fractal at the given level, without rendering it."""

def fill_from_corners(filled: list[bytes], number: int, _dimension: int, level: int, base: int) -> int:
    """Sums each corner's position products into a base fill and raises it to the level."""

def grid(number: int, dimension: int, level: int) -> int:
    """Returns the total cells of the grid, number to the dimension, to the level."""

def limit(code: int, dimension: int, level: int, base: int) -> tuple[int, int]:
    """Returns the fill ratio the code walks toward as the side number grows, reduced."""

def pairs(tile: NDArray[Any]) -> list[tuple[int, int]]:
    """Counts, per axis, the adjacent filled pairs and the cross positions whose two end cells are both filled."""

def positions(residue: int, number: int, base: int) -> int:
    """Counts the indices below number that equal residue modulo base."""

def pro_fills(code: int, number: int, level: int) -> int:
    """Returns the filled triangle count of the code's pro projection at the given level, without rendering it."""

def pro_voids(code: int, number: int, level: int) -> int:
    """Returns the empty triangle count of the code's pro projection at the given level."""

def profile_of_tile(tile: NDArray[Any], level: int) -> list[int]:
    """Counts the filled cells of the tile's level-fold power on every diagonal plane `x_1 + ... + x_D = s`."""

def ratio(code: int, number: int, dimension: int, level: int, base: int) -> float:
    """Returns the filled fraction of the grid, or 0.0 for an empty grid."""

def rational(code: int, number: int, dimension: int, level: int, base: int) -> tuple[int, int]:
    """Returns the exact filled fraction as a fraction of fill over grid, reduced."""

def surface(code: int, number: int, level: int, base: int) -> int:
    """Returns the exposed face count of the code's 3D fractal at the given level."""

def void(code: int, number: int, dimension: int, level: int, base: int) -> int:
    """Returns the empty cell count, grid minus fill."""
