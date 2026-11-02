from typing import Any, Literal

from numpy.typing import NDArray

READINGS: list[Literal["N", "Square", "SquarePlusN", "Cube", "Primes"]]

class Design:
    """A digit-parity design `b:D`, or Rudin-Shapiro: the sign `x(n)` is `-1` to the count of base-`b` digits of `n` in `D`."""
    @staticmethod
    def digits(base: int, mask: int) -> dict[str, Any]:
        """Builds the design `b:D` from its base and its digit mask."""
    @staticmethod
    def rudin() -> dict[str, Any]:
        """Returns Rudin-Shapiro."""
    @staticmethod
    def ternary() -> dict[str, Any]:
        """Returns the ternary parity, `3:2`, the parity of the ternary digits equal to 2."""
    @staticmethod
    def thue() -> dict[str, Any]:
        """Returns Thue-Morse, `2:1`."""

class Reading:
    """The sequences a design is read along: `y(n) = x(P(n))` for a polynomial `P`, or `x(p_n)` along the primes."""
    @staticmethod
    def all() -> list[Literal["N", "Square", "SquarePlusN", "Cube", "Primes"]]:
        """Returns every Reading in canonical order."""
    @staticmethod
    def formula(reading: Literal["N", "Square", "SquarePlusN", "Cube", "Primes"]) -> str:
        """Returns the reading's formula, written the way the page prints it."""

def arcs(design: dict[str, Any], cut: float) -> list[dict[str, Any]]:
    """Lists a design's major arcs along the squares below the cut, best first, each with its amplitude."""

def battery(stream: list[int]) -> dict[str, Any]:
    """Runs the battery on a stream of signs."""

def bias(stream: list[int], exponents: list[int]) -> list[int]:
    """Measures the bias `S(N) = sum_(n<N) y(n)` of a stream at `N = 2^e` for each exponent; a checkpoint past the stream's end sums the whole stream."""

def designs() -> list[dict[str, Any]]:
    """Lists the 57 digit designs, base by base and mask by mask, then Rudin-Shapiro."""

def model(design: dict[str, Any], cut: float, exponents: list[int]) -> list[float]:
    """Predicts the bias `S(N) = sum_(n<N) x(n^2)` at `N = 2^e` for each exponent, from the arcs below the cut: `1 + Re sum_gamma A(gamma) U(gamma; N^2)`."""

def name(design: dict[str, Any]) -> str:
    """Returns the design's name in the `b:D` form, the digits of `D` ascending, or `rudin-shapiro`."""

def near_heads(design: dict[str, Any], reading: Literal["N", "Square", "SquarePlusN", "Cube", "Primes"]) -> list[dict[str, Any]]:
    """Lists the battery pairs that are near-heads of a polynomial reading at the design's base, the 44 dilation pairs first and the two kernel pairs after; the primes have none."""

def sign(design: dict[str, Any], n: int) -> int:
    """Returns the design's sign at a number, plus or minus one."""

def stream(design: dict[str, Any], reading: Literal["N", "Square", "SquarePlusN", "Cube", "Primes"], count: int) -> list[int]:
    """Builds a reading's first signs: `y(n) = x(P(n))` for `n < count`, or `x(p_n)` along the first `count` primes from a segmented sieve."""
