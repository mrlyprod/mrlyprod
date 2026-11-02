from typing import Any, Literal

from numpy.typing import NDArray

def add(a: list[int], b: list[int]) -> list[int]:
    """Adds two sequences term by term over their shared length."""

def cauchy(a: list[int], b: list[int]) -> list[int]:
    """Convolves two sequences, keeping the exact prefix their shared length affords."""

def characteristic(coefficients: list[tuple[int, int]]) -> list[tuple[int, int]]:
    """Returns the monic characteristic polynomial of a recurrence, highest power first."""

def decimate(a: list[int], step: int, offset: int) -> list[int]:
    """Keeps every step-th term from the offset onward."""

def delta(a: list[int]) -> list[int]:
    """Returns the first differences of a sequence, one term shorter."""

def growth(coefficients: list[tuple[int, int]]) -> float:
    """Returns the largest positive real root of a recurrence's characteristic polynomial, the growth rate, or a not-a-number where no real root lands."""

def hadamard(a: list[int], b: list[int]) -> list[int]:
    """Multiplies two sequences term by term over their shared length."""

def recurrence(terms: list[int]) -> list[tuple[int, int]] | None:
    """Finds the smallest linear constant-coefficient recurrence that fits every supplied term."""

def scale(a: list[int], factor: int) -> list[int]:
    """Multiplies every term of a sequence by the factor."""

def shift(a: list[int], count: int) -> list[int]:
    """Drops the first terms of a sequence."""

def sigma(a: list[int]) -> list[int]:
    """Returns the partial sums of a sequence."""

def sub(a: list[int], b: list[int]) -> list[int]:
    """Subtracts the second sequence from the first over their shared length."""
