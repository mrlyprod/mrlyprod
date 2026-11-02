from typing import Any, Literal

from numpy.typing import NDArray

def cap(base: int) -> int:
    """The widest dimension the exact carry arithmetic reaches at the base."""

def carry_matrix(base: int, dimension: int) -> list[list[int]]:
    """The carry matrix over the reachable carries `|c| <= (D-1)/2`, rows indexed by the carry out."""

def characteristic(rows: list[list[int]]) -> list[int]:
    """The monic characteristic polynomial of a square integer matrix, highest power first."""

def determinant(rows: list[list[int]]) -> int:
    """The determinant of a square integer matrix, read off its characteristic polynomial."""

def digit_polynomial(base: int, dimension: int) -> list[int]:
    """The digit polynomial of the base-`q` middle-digit design in dimension `D`, lowest power first."""

def even_block(base: int, dimension: int) -> list[list[int]]:
    """The reflection-even block of the carry matrix, of size `ceil(D/2)`."""

def fill(base: int, dimension: int) -> int:
    """The count of level-one cells the design keeps, `f_D = (q - 1)^(D-1) (q - 1 + D)`."""

def ladder(base: int, dimension: int, levels: int) -> list[int]:
    """The counts `a_D(L)` of level-`L` cells meeting the central diagonal hyperplane, from `L = 0`."""

def perron(rows: list[list[int]]) -> float:
    """The Perron root of a nonnegative square integer matrix."""

def sign(base: int, dimension: int) -> int:
    """The sign of `log_q rho_D - (log_q f_D - 1)`, the slice sign law's reading, in exact integers."""

def spectral_ratio(base: int, dimension: int) -> float | None:
    """The Perron root over the modulus of the second eigenvalue, or none where the block is one wide."""

def trace(rows: list[list[int]]) -> int:
    """The trace of a square integer matrix."""
