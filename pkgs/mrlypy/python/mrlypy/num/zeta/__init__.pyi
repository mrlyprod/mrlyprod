from typing import Any, Literal

from numpy.typing import NDArray

JOIN: float

class Complex:
    """A complex number: a real and an imaginary part."""
    def __init__(self, re: float, im: float) -> None: ...
    @property
    def re(self) -> float:
        """The real part."""
    @property
    def im(self) -> float:
        """The imaginary part."""
    def abs(self) -> float:
        """Returns the modulus."""
    def arg(self) -> float:
        """Returns the principal argument."""
    def exp(self) -> Complex:
        """Returns the exponential."""
    def ln(self) -> Complex:
        """Returns the principal logarithm."""
    @staticmethod
    def new(re: float, im: float) -> Complex:
        """Builds a complex number from its parts."""
    @staticmethod
    def turn(angle: float) -> Complex:
        """Returns a unit complex number at the given angle."""
    @staticmethod
    def from_dict(data: Any) -> Complex:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

class Line:
    """The critical line: the Bernoulli numbers and the Euler-Maclaurin weights the two engines share, built once."""
    def __init__(self) -> None: ...
    def count(self, t: float) -> int:
        """Counts the zeros on the line below t."""
    def exact(self, t: float) -> float:
        """Returns Z(t) from the Euler-Maclaurin value turned onto the real axis."""
    def gram(self, n: int) -> float:
        """Returns the n-th Gram point, where theta is n pi, by Newton from the right."""
    def maclaurin(self, t: float) -> Complex:
        """Returns zeta at one half plus i t by the complex Euler-Maclaurin sum: t plus ten terms and seven Bernoulli corrections."""
    @staticmethod
    def new() -> Line:
        """Builds the line: the even Bernoulli numbers through the fourteenth and their Euler-Maclaurin weights."""
    def novelty_coefficients(self, gammas: list[float]) -> list[Complex]:
        """Returns the wave coefficient of every zero at the given ordinates: F(rho) zeta(rho - 1) over zeta'(rho) at rho one half plus i gamma, F the Mellin transform of the bump."""
    def pair(self, s: Complex) -> tuple[Complex, Complex]:
        """Returns zeta and its derivative together at any complex s but one, by the same Euler-Maclaurin sum: the modulus of t plus ten terms and seven Bernoulli corrections, each term differentiated in s."""
    def point(self, t: float) -> tuple[Complex, float]:
        """Returns zeta on the line and Z(t) together, from the engine that serves the t."""
    def seam(self, t0: float, t1: float, steps: int) -> float:
        """Returns the largest gap between the two engines over the t range on a grid."""
    def siegel(self, t: float) -> float:
        """Returns Z(t) by the Riemann-Siegel formula: the main sum and the first four corrections."""
    def theta(self, t: float) -> float:
        """Returns the Riemann-Siegel theta: the argument of gamma at one quarter plus i t over two, less t ln pi over two, by Stirling's series after a shift of ten."""
    def z(self, t: float) -> float:
        """Returns Z(t): Euler-Maclaurin below the join, Riemann-Siegel above."""
    def zeros(self, count: int) -> list[float]:
        """Returns the first zeros on the line: sign changes of Z between Gram points, refined by bisection on Euler-Maclaurin to a billionth."""
    @staticmethod
    def from_dict(data: Any) -> Line:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

def bump(u: float) -> float:
    """The smooth window on [1, 2]: exp(4 - 1/((u - 1)(2 - u))) inside, zero outside, every derivative vanishing at the ends and a peak of one at u = 3/2."""

def corrections(p: float) -> list[float]:
    """Returns the first four Riemann-Siegel corrections at the fractional part p: the kernel and its derivatives by central differences with one Richardson step."""

def kernel(p: float) -> float:
    """Returns the Riemann-Siegel kernel, the cosine ratio that leads the remainder, in the form that stays finite at its removable points."""

def mellin(s: Complex) -> Complex:
    """Returns the Mellin transform of the bump at a complex s, the integral of bump(u) u^(s - 1) over [1, 2], by a 4096-node midpoint rule."""

def novelty_main() -> float:
    """Returns the main term of the smoothed novelty: six over pi squared times the bump's transform at two."""

def novelty_wave(gammas: list[float], coef: list[Complex], log_y: float) -> float:
    """Sums the waves of the zeros at log y: twice the real part of the coefficients times y to the minus i gamma, the smoothed error over y to the three halves that the zeros predict."""

def psi_formula(x: float, gammas: list[float]) -> float:
    """Returns the von Mangoldt explicit formula at x over the zeros at the given ordinates and their mirrors: x less the sum of x to the rho over rho, less ln two pi, less half the ln of one minus x to the minus two."""

def psi_stair(x: int) -> list[float]:
    """Returns the Chebyshev staircase at every whole number from one to x: the sum of ln p over the prime powers up to each."""

def raise_(base: float, exponent: Complex) -> Complex:
    """Returns a positive real base raised to a complex exponent."""

def sharp_novelty(prefix: list[int], y: float) -> float:
    """Returns the sharp novelty error at y: y squared times the totient sum over the scales from 1 over y to 2 over y, both ends in, less nine over pi squared, from the prefix sums of the totients, which must reach 2 over y."""

def smoothed_novelty(phi: list[int], y: float, main: float) -> float:
    """Returns the smoothed novelty error at y: y squared times the totients weighed by the bump at n y, less the main term given; the totients must reach 2 over y."""
