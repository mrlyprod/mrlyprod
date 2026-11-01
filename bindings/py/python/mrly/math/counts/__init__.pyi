from typing import Any, Literal

from numpy.typing import NDArray
from . import diagonal, ladder, six

class Exposure:
    """The counts the exposure recurrence runs on: the filled cells and exposed faces of the tile, and per axis its adjacent pairs and spanning positions."""
    @property
    def occupancy(self) -> int:
        """The filled cells of the tile."""
    @occupancy.setter
    def occupancy(self, value: int) -> None: ...
    @property
    def exposed(self) -> int:
        """The exposed faces of the tile."""
    @exposed.setter
    def exposed(self, value: int) -> None: ...
    @property
    def axes(self) -> list[tuple[int, int]]:
        """Per axis, the adjacent filled pairs and the spanning positions."""
    @axes.setter
    def axes(self, value: list[tuple[int, int]]) -> None: ...
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

class Placements:
    """The placement counts the face recurrence runs on: for each pattern `P` of cells inside a `2^d` window and each lowest digit `w` of the tile, whether `w + P` fits the tile with wrap-around, and which pattern `P ^ W` the carried digits must then form one level up."""
    def at(self, level: int) -> Skeleton | None:
        """Returns the face counts of the level-fold Kronecker power, or none past a `u128`."""
    @staticmethod
    def of_tile(tile: NDArray[Any]) -> Placements:
        """Reads the placement counts off a rendered tile of two to four dimensions."""
    def recurrence(self, k: int | None = None) -> list[int] | None:
        """Returns the coefficients `c` of the recurrence `a(L) = c[0] a(L-1) + c[1] a(L-2) + ...` the `k`-face count obeys from `L = 0`, every face when `k` is none, or none past an `i128`."""
    def roots(self, k: int | None = None) -> list[tuple[int, int]]:
        """Returns the roots of the recurrence the `k`-face count obeys, every face when `k` is none, each with the multiplicity the recurrence needs: the spanning counts of the patterns the count reaches, a root repeated once per pattern of that count on one carry chain."""

class Skeleton:
    """The face counts of the union of closed unit cubes a design's level power fills: `counts[k]` is its distinct unit `k`-faces, the cells last, and `euler` their alternating sum."""
    @property
    def counts(self) -> list[int]:
        """The distinct unit `k`-faces for `k = 0..=dimension`."""
    @counts.setter
    def counts(self, value: list[int]) -> None: ...
    @property
    def euler(self) -> int:
        """The Euler characteristic, `sum (-1)^k counts[k]`."""
    @euler.setter
    def euler(self, value: int) -> None: ...
    def cells(self) -> int:
        """The filled cells."""
    def edges(self) -> int:
        """The distinct unit edges."""
    def faces(self) -> int:
        """The distinct unit squares, the cells themselves in the plane."""
    def vertices(self) -> int:
        """The distinct corners."""

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

def skeleton(code: int, number: int, dimension: int, base: int, level: int) -> Skeleton:
    """Returns the face counts of the code's design at the given level, from the tile's placement counts alone."""

def skeleton_of_tile(tile: NDArray[Any], level: int) -> Skeleton | None:
    """Returns the face counts of the tile's level-fold Kronecker power, or none past a `u128`."""

def skeleton_recurrence(code: int, number: int, dimension: int, base: int, k: int | None = None) -> list[int]:
    """Returns the coefficients of the recurrence the code's `k`-face count obeys along the level, every face and the Euler characteristic when `k` is none."""

def surface(code: int, number: int, level: int, base: int) -> int:
    """Returns the exposed face count of the code's 3D fractal at the given level."""

def void(code: int, number: int, dimension: int, level: int, base: int) -> int:
    """Returns the empty cell count, grid minus fill."""
