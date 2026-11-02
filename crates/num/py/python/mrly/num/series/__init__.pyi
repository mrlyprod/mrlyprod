from typing import Any, Literal

from numpy.typing import NDArray

APERY: float
BASEL: float
CATALAN: float
EULER: float
VISIBLE: float

def basel(n: int) -> float:
    """Returns the Basel sum of the reciprocal squares over n terms, walking to pi squared over six."""

def bernoulli(count: int) -> list[tuple[int, int]]:
    """Builds the first Bernoulli numbers as exact reduced fractions on the minus one half convention."""

def beta(s: float, terms: int) -> float:
    """Returns the Dirichlet beta value, the alternating odd-denominator sum averaged over its last two partial sums."""

def binary(limit: int) -> list[int]:
    """Returns the powers of two up to the limit."""

def catalan(limit: int) -> list[int]:
    """Returns the distinct Catalan numbers up to the limit."""

def chi3(number: int) -> int:
    """Returns the mod-three rhythm of the number: zero, one, minus one."""

def chi4(number: int) -> int:
    """Returns the mod-four rhythm of the number: zero, one, zero, minus one."""

def chi8(number: int) -> int:
    """Returns the mod-eight rhythm of the number, the discriminant minus-eight character: one on one and three, minus one on five and seven, zero on the evens."""

def dirichlet(s: float, rhythm: list[int], terms: int) -> float:
    """Returns the L-series partial sum with a periodic rhythm painted on the terms."""

def e_partial(n: int) -> float:
    """Returns one plus one over n raised to the n, walking to the natural base."""

def euler_gamma_partial(n: int) -> float:
    """Returns the harmonic sum of n terms less the logarithm of n, walking to the Euler-Mascheroni constant."""

def euler_product(s: float, limit: int) -> float:
    """Returns the Euler product of zeta, one over one minus p to the minus s over the primes up to the limit."""

def evens(limit: int) -> list[int]:
    """Returns the even numbers up to the limit."""

def fibonacci(limit: int) -> list[int]:
    """Returns the distinct Fibonacci numbers up to the limit."""

def harmonic(terms: int) -> float:
    """Returns the partial harmonic sum, the reciprocals of one through the term count."""

def lambda_(s: float, terms: int) -> float:
    """Returns the Dirichlet lambda value, one minus two to the minus s times zeta."""

def leibniz(n: int) -> float:
    """Returns the Leibniz alternating sum of the odd reciprocals over n terms, walking to pi over four."""

def li(x: float) -> float:
    """Returns the logarithmic integral of a positive x by the Ramanujan series, the smooth count of the primes below x."""

def mertens(n: int) -> int:
    """Returns the Mertens function at n, the Mobius values of one through n summed."""

def odds(limit: int) -> list[int]:
    """Returns the odd numbers up to the limit."""

def visible(limit: int, dimension: int) -> int:
    """Counts the lattice points of the dimension-cube of the limit whose coordinates share no divisor, by Mobius inversion."""

def wallis_half_pi(n: int) -> float:
    """Returns the Wallis product taken to n paired factors, four k squared over four k squared less one, walking to pi over two."""

def wallis_quarter_pi(factors: int) -> float:
    """Returns the Wallis product of one minus one over the odd squares taken to n factors, walking to pi over four."""

def zeta(s: float, terms: int) -> float:
    """Returns the zeta value above one, the partial sum closed by its Euler-Maclaurin tail."""
