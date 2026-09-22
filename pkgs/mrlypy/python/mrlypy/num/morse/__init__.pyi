from typing import Any, Literal

from numpy.typing import NDArray

LIFTS: list[Literal["Parity", "And", "Xor", "Sum"]]

class Lift:
    """The four ways the word lifts from a line to the plane, one sign at every site."""
    @staticmethod
    def all() -> list[Literal["Parity", "And", "Xor", "Sum"]]:
        """Returns every Lift in canonical order."""
    @staticmethod
    def at(lift: Literal["Parity", "And", "Xor", "Sum"], i: int, j: int) -> int:
        """Returns the sign at a site, zero for plus one and one for minus one."""
    @staticmethod
    def formula(lift: Literal["Parity", "And", "Xor", "Sum"]) -> str:
        """Returns the lift's formula, written the way the page prints it."""

def boundary(word: bytes) -> bytes:
    """Returns the run-boundary word, one wherever a letter differs from the next."""

def difference(a: bytes, b: bytes) -> bytes:
    """Exclusive-ors two grids of the same length, site by site."""

def digits(length: int) -> bytes:
    """Builds the first letters of the Thue-Morse word by the digit rule."""

def doubling(length: int) -> bytes:
    """Builds the period-doubling word by the substitution `1 -> 10`, `0 -> 11`, from the seed 1."""

def faults(a: bytes, b: bytes) -> int:
    """Counts the sites where two grids of the same length differ."""

def fold(grid: bytes, side: int, number: int) -> dict[str, Any]:
    """Tests a grid against the Kronecker power of its own corner tile."""

def letter(place: int) -> int:
    """Returns the Thue-Morse letter at the place, the parity of its binary digit sum."""

def lift(kind: Literal["Parity", "And", "Xor", "Sum"], side: int) -> bytes:
    """Builds a lift as a row-major sign grid of the side, zero for plus one and one for minus one."""

def power(tile: bytes, number: int, level: int) -> bytes:
    """Folds a tile of the side into its Kronecker power at the level, one bit per site."""

def repeat(tile: bytes, number: int, side: int) -> bytes:
    """Repeats a tile until it fills a grid of the side."""

def runs(word: bytes) -> list[int]:
    """Returns the lengths of the maximal blocks of one repeated letter, in order."""

def stage(rounds: int) -> bytes:
    """Returns the substitution stage after the rounds, a word of length two to the rounds."""

def substitution(length: int) -> bytes:
    """Builds the first letters of the Thue-Morse word by the substitution `0 -> 01`, `1 -> 10`."""

def upsample(grid: bytes, side: int, scale: int) -> bytes:
    """Blows a grid up by the scale, every site becoming a scale-by-scale block."""
