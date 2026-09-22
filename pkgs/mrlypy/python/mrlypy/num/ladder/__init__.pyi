from typing import Any, Literal

from numpy.typing import NDArray
import mrlypy.num.zeta

ROUNDING: float

class Design:
    """A digit design: the base `q`, the digit set `F` its elements are written with, and the peel depth `P` its ladder starts at."""
    def __init__(self, base: int, digits: list[int]) -> None: ...
    def abscissa(self) -> float:
        """Returns the abscissa `alpha = log_q k`."""
    def base(self) -> int:
        """Returns the base."""
    def digits(self) -> list[int]:
        """Returns the digit set, ascending."""
    @staticmethod
    def new(base: int, digits: list[int]) -> Design:
        """Builds a design on the base and the digit set, choosing the peel depth."""
    def peel(self) -> int:
        """Returns the peel depth."""
    def period(self) -> float:
        """Returns the pole spacing `2 pi / log q`."""
    def pole(self, m: int, j: int) -> mrlypy.num.zeta.Complex:
        """Returns the pole `s_(m,j) = alpha - m + 2 pi i j / log q`."""
    @staticmethod
    def with_peel(base: int, digits: list[int], peel: int) -> Design:
        """Builds a design at an explicit peel depth, at least two."""
    @staticmethod
    def from_dict(data: Any) -> Design:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

def cofactor(design: Design, s: mrlypy.num.zeta.Complex, tolerance: float) -> tuple[mrlypy.num.zeta.Complex, float]:
    """Returns the Lyndon cofactor `Z(s) = zeta_F(s) (1 - k q^(-s))` and the bound it is known to."""

def residue(design: Design, m: int, j: int, tolerance: float) -> tuple[mrlypy.num.zeta.Complex, float]:
    """Returns the residue of `zeta_F` at `s_(m,j) = alpha - m + 2 pi i j / log q` and the bound it is known to."""

def zeta(design: Design, s: mrlypy.num.zeta.Complex, tolerance: float) -> tuple[mrlypy.num.zeta.Complex, float]:
    """Returns `zeta_F(s)` and the bound it is known to."""
