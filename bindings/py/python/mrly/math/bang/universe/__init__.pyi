from typing import Any, Literal

from numpy.typing import NDArray

def anf(code: int, dimension: int) -> bytes:
    """Returns the algebraic normal form coefficients of a code, one per corner."""

def anf_string(code: int, dimension: int) -> str:
    """Formats the algebraic normal form of a code as a sum of monomials."""

def apply(element: tuple[list[int], bytes], corner: bytes) -> bytes:
    """Applies a symmetry element to a corner."""

def corner_index(corner: bytes) -> int:
    """Returns the bit position a binary corner occupies in a code."""

def degree(code: int, dimension: int) -> int:
    """Returns the algebraic degree of a code, or -1 for the zero design."""

def orbit(code: int, dimension: int) -> list[int]:
    """Returns every code a design reaches under the full symmetry group."""

def permutations(n: int) -> list[list[int]]:
    """Returns every permutation of 0..n in sorted order."""
