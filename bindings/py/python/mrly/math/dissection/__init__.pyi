from typing import Any, Literal

from numpy.typing import NDArray

BAR_A: float
BAR_B: float
CAP_BASE: int
CERTIFIED: list[tuple[int, int]]
DIGIT_WALL: int
FIRST_BELOW: int
GAMMA: float
WINDOW_WALL: int

def cap(y: int) -> int:
    """Returns `Q = floor(y^(3/5))`, the largest denominator the dissection admits, exact in integers."""

def chain_exponent(base: int) -> float:
    """Returns the chain's certificate exponent at one missing digit, `alpha_1 = log_base(z base/(base - 1))`, the same at every missing digit."""

def chain_margin(base: int) -> float:
    """Returns the chain's margin at the bar, the cubic cleared of denominators at `w = base^(1/5)(1 - 1/base)`, positive exactly when the root sits below `w` and `alpha_1 < 1/5`."""

def chain_root(base: int) -> float:
    """Returns the root `z > 1` of the digit-uniform chain at one missing digit, `(z-1)^3 = (2/pi)(log base) z + gamma'(z-1) + (2/pi)(z-1)^2/(base z - 1)`, by bisection."""

def chain_wall() -> int:
    """Returns the chain's wall, one past the last base up to [`CAP_BASE`] whose margin is not positive."""

def consecutive(digits: list[int]) -> bool:
    """Returns whether the digit set keeps two consecutive digits, the hypothesis region C1 reads."""

def fraction(a: int, y: int, cap: int) -> tuple[int, int]:
    """Returns the last continued-fraction convergent `l/d` of `a/y` whose denominator is at most the cap, the Dirichlet fraction of the dissection."""

def kappa(base: int, digits: list[int]) -> tuple[int, int]:
    """Returns `kappa_F = (base/phi(base)) #{f in F : gcd(f, base) = 1}/fill`, the main-term constant of the prime count, as a reduced fraction."""

def masses(base: int, digits: list[int], level: int) -> list[float]:
    """Returns the unshifted masses `c_j = sum_(a < base^j) |hat F_j(a/base^j)|` for `j = 0..=level`, the `l^1` mass region A pays, `c_0 = 1`."""

def reach(base: int, missing: int) -> str:
    """Returns how the theorem reaches the set missing one digit: `proof` from the chain's wall, `certificate` at every base from [`DIGIT_WALL`] below it and at the [`CERTIFIED`] sets, `none` elsewhere."""

def reading(base: int, fill: int, masses: list[float]) -> float:
    """Returns the `l^1` exponent the top two masses read, `log_base(c_j/(fill c_(j-1)))`: a reading of the growth region A pays, never a certificate."""

def regions(base: int, level: int, z: int) -> list[Literal["A", "B", "C1", "C2"]]:
    """Returns the region of every frequency `a/y`, `a < y = base^level`, at the cut `Z`, the fraction taken by [`fraction`] at [`cap`]."""

def tally(base: int, digits: list[int], level: int, samples: int) -> dict[str, Any]:
    """Tallies the set below `base^level` on a log grid of the given size, with the Mobius values and the primes sieved to the span."""

def weights(base: int, digits: list[int], level: int) -> list[float]:
    """Returns `|hat F_level(a/base^level)|` at every `a < base^level`, built one digit at a time from `hat F_j(t) = hat F(t) hat F_(j-1)(base t)`."""
