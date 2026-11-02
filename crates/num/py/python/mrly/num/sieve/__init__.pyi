from typing import Any, Literal

from numpy.typing import NDArray

PLANE_LIMIT: float

def cells(word: list[int], dimension: int) -> int:
    """Returns the cells the word leaves, the product of its letters' fills, one punctured tile a letter."""

def exponent(word: list[int], dimension: int) -> float:
    """Returns the box exponent the word reads at its own scale, the logarithm of its cells over the logarithm of its side, which walks up to the dimension on a schedule of distinct growing letters and stands still on any schedule that reuses its letters."""

def flat_word(side: int, levels: int) -> list[int]:
    """Returns the constant schedule, one odd side repeated to the count of levels, whose limit set is the fixed-ratio carpet."""

def holes(word: list[int], dimension: int) -> int:
    """Returns the punctures the word makes, one per surviving cell at every level."""

def limit(word: list[int], dimension: int) -> float | None:
    """Returns the limit the word's schedule walks to in the given dimension, when the word names a schedule at all."""

def odd_word(levels: int) -> list[int]:
    """Returns the classical Wallis schedule, the odd sides three, five, seven and on, to the count of levels."""

def parity_product(dimension: int, odd: bool) -> float:
    """Returns the Wallis sieve product of a parity design, `prod_(N odd >= 3) (1 - N^-dim)` for the corners with an odd count of odd coordinates and `prod (1 + N^-dim)` for the even count, at `dim >= 2`."""

def punctures(word: list[int], dimension: int) -> list[int]:
    """Lists every puncture the word makes in the given dimension: its corner along each axis and then its side, all in units of the word's finest cell, so a level-one hole is the widest block in the list."""

def raster(word: list[int]) -> tuple[int, bytes]:
    """Builds the plane sieve the word spells as a raster: its side, then one byte a site, row by row, one where the site survives and zero where a level punched it out."""

def ratio(word: list[int], dimension: int) -> float:
    """Returns the share of the whole the word leaves, the product of one minus the inverse of each letter's site count, exact as a product of the letters' fills."""

def row_fill(profile: list[int], side: int) -> int:
    """Returns the cells a design of this profile fills at one side, `sum_j a_j E^(dim-j) O^j` with `E` and `O` the even and the odd positions an axis holds: `P_F(n)` at side `2n - 1` and `w n^dim` at side `2n`."""

def row_law(profile: list[int]) -> dict[str, Any]:
    """Reads the design of this profile along the row word: its drift, its first correction, the roots of its fill polynomial, its constant from the Gamma form, the constant spelled, and the constant again by reflection, against the mirror and as a Wallis sieve product wherever those apply."""

def row_profile(code: int, dimension: int) -> list[int]:
    """Counts a base-2 design's corners by how many odd coordinates each holds, the profile the row word reads: the code is a bitmask over the corners, corner `i` is the binary digits of `i` as `math::bang::code_to_corners` reads it, so its odd coordinates are the ones of `i`."""

def row_settle(profile: list[int], stops: list[int], even: bool) -> list[float]:
    """Walks the renormalised fill `R_L (2^dim/w)^L / L^drift` of the design of this profile to every stop, `R_L` the product of the letters' fill ratios, on the odd sides `3, 5, ..., 2L+1` or on the even sides `2, 4, ..., 2L`."""

def side(word: list[int]) -> int:
    """Returns the side of the word, the product of its letters' sides."""

def solid_limit() -> float:
    """Returns the limit of the solid Wallis sieve's surviving volume, the product of one minus n to the minus three over the odd n from three, in closed form."""
