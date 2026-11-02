from typing import Any, Literal

from numpy.typing import NDArray

def aliquot(number: int) -> int:
    """Returns the sum of the proper divisors of the number, its divisor sum less itself, zero for zero and for one."""

def coprime(a: int, b: int) -> bool:
    """Returns whether two numbers share no divisor above one."""

def divisors(number: int) -> list[int]:
    """Builds every divisor of a wide number from its factorization, ascending, empty for zero."""

def factorial(number: int) -> int:
    """Returns the factorial of the number, the product of one through it, erring past thirty-four."""

def factorize(number: int) -> list[tuple[int, int]]:
    """Returns the prime and exponent pairs of the number in ascending primes, by trial division on the six-step wheel."""

def factorize_wide(number: int) -> list[tuple[int, int]]:
    """Returns the prime and exponent pairs of a wide number in ascending primes, by trial division on the six-step wheel."""

def gcd(a: int, b: int) -> int:
    """Returns the greatest common divisor of two numbers by the Euclidean algorithm, zero for two zeroes."""

def lcm(a: int, b: int) -> int:
    """Returns the least common multiple of two numbers, zero when either side is zero."""

def mobius(number: int) -> int:
    """Returns the Mobius value of the number: zero for zero or a squared factor, else minus one to the count of primes."""

def mobius_sieve(limit: int) -> list[int]:
    """Sieves the Mobius values of zero through the limit in one pass."""

def radical(number: int) -> int:
    """Returns the radical of the number, the product of its distinct primes, zero for zero and one for one."""

def reduce(numerator: int, denominator: int) -> tuple[int, int]:
    """Reduces a fraction to its lowest terms, a zero numerator and denominator reading as zero over one."""

def sigma(number: int, power: int) -> int:
    """Returns the sum of every divisor of the number raised to the power, so power zero counts them."""

def squarefree(number: int) -> bool:
    """Returns whether no prime squares into the number, true for one and false for zero."""

def totient(number: int) -> int:
    """Returns the Euler totient of the number from its factorization, zero for zero and one for one."""

def totients(n: int) -> list[int]:
    """Sieves the Euler totients of zero through n in one pass, the run beside the single value."""

def twisted(number: int, rhythm: list[int]) -> int:
    """Returns the divisor sum with a periodic rhythm painted on each divisor, zero for zero and for an empty rhythm."""
