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

def punctures(word: list[int], dimension: int) -> list[int]:
    """Lists every puncture the word makes in the given dimension: its corner along each axis and then its side, all in units of the word's finest cell, so a level-one hole is the widest block in the list."""

def raster(word: list[int]) -> tuple[int, bytes]:
    """Builds the plane sieve the word spells as a raster: its side, then one byte a site, row by row, one where the site survives and zero where a level punched it out."""

def ratio(word: list[int], dimension: int) -> float:
    """Returns the share of the whole the word leaves, the product of one minus the inverse of each letter's site count, exact as a product of the letters' fills."""

def side(word: list[int]) -> int:
    """Returns the side of the word, the product of its letters' sides."""

def solid_limit() -> float:
    """Returns the limit of the solid Wallis sieve's surviving volume, the product of one minus n to the minus three over the odd n from three, in closed form."""
