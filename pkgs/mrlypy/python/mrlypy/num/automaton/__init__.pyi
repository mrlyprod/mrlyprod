from typing import Any, Literal

from numpy.typing import NDArray
import mrlypy.num.memory
import mrlypy.num.zeta

ROUNDING: float

class Automaton:
    """A memory design read as a matrix ladder: the rule, the transfer matrix on its `(k-1)`-window states, and the peel depth its Dirichlet series is continued from."""
    def __init__(self, rule: mrlypy.num.memory.Rule) -> None: ...
    def abscissa(self) -> float:
        """Returns the abscissa `alpha = log_q rho`, with `rho` the exact Perron root of [`crate::num::memory::perron`]."""
    def base(self) -> int:
        """Returns the base `q = 2^D`."""
    def cofactor(self, s: mrlypy.num.zeta.Complex, tolerance: float) -> tuple[mrlypy.num.zeta.Complex, float]:
        """Returns the matrix Lyndon cofactor `Z_W(s) = det(I - q^(-s) T) zeta_W(s)` and the bound it is known to."""
    def denominator(self) -> list[float]:
        """Returns the coefficients `c_0 .. c_n` of `det(I - x T) = sum c_i x^i`, the ladder denominator read as a polynomial in `x = q^(-s)`."""
    def matrix(self) -> list[list[float]]:
        """Returns the transfer matrix `T = Gamma_0` the ladder runs on, the transpose of [`crate::num::memory::transfer`], entry `(u', u)` counting the letters carrying `u` to `u'`."""
    @staticmethod
    def new(rule: mrlypy.num.memory.Rule) -> Automaton:
        """Builds the ladder of a rule, choosing the peel depth."""
    def peel(self) -> int:
        """Returns the peel depth `P`."""
    def period(self) -> float:
        """Returns the pole spacing `2 pi / log q`."""
    def perron(self) -> tuple[float, float]:
        """Returns the Collatz-Wielandt bracket `(low, high)` of the Perron root of the transfer matrix, the ratios the ladder divides with."""
    def residue(self, w0: mrlypy.num.zeta.Complex, tolerance: float) -> tuple[mrlypy.num.zeta.Complex, float]:
        """Returns the residue of `zeta_W` at a simple pole `w0` of the resolvent and the bound it is known to."""
    def rule(self) -> mrlypy.num.memory.Rule:
        """Returns the rule."""
    def states(self) -> int:
        """Returns the state count `q^(k-1)`."""
    @staticmethod
    def with_peel(rule: mrlypy.num.memory.Rule, peel: int) -> Automaton:
        """Builds the ladder at an explicit peel depth, at least the rule width and at least two."""
    def zeta(self, s: mrlypy.num.zeta.Complex, tolerance: float) -> tuple[mrlypy.num.zeta.Complex, float]:
        """Returns `zeta_W(s)` and the bound it is known to."""
    @staticmethod
    def from_dict(data: Any) -> Automaton:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""
