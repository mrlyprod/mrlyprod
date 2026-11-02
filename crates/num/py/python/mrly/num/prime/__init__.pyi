from typing import Any, Literal

from numpy.typing import NDArray

class Sieve:
    """The sieve of Eratosthenes taken one prime at a time, each number remembering which prime struck it."""
    def __init__(self, limit: int) -> None: ...
    def count(self) -> int:
        """Returns the count of numbers marked prime so far."""
    def done(self) -> bool:
        """Returns whether every number is settled."""
    def finish(self) -> None:
        """Runs the sieve to the end."""
    @staticmethod
    def new(limit: int) -> Sieve:
        """Starts a sieve over zero through the limit with every number untouched; it is done at once when no prime has its square inside."""
    def rank(self) -> int:
        """Returns the count of primes used so far."""
    def step(self) -> int:
        """Uses the next prime: marks it prime, strikes its untouched multiples from its square with its rank plus one, and returns it; zero once done."""
    def struck(self) -> int:
        """Returns the count of numbers the last step struck."""
    def types(self) -> bytes:
        """Returns the type of every number from zero: zero untouched, one prime, and one past the rank of the prime that struck it."""
    @staticmethod
    def from_dict(data: Any) -> Sieve:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

def chart(top: int, bins: int) -> list[dict[str, Any]]:
    """Reads the prime count against x over ln x and li at evenly spaced points from two up to the top, at most the given count of them, the top always last."""

def flags(limit: int) -> list[bool]:
    """Returns whether every number from zero through the limit is prime, the finished sieve read flag by flag."""

def goldbach(number: int) -> int:
    """Returns the count of unordered pairs of primes summing to the number, zero below four."""

def goldbach_record(top: int) -> list[int]:
    """Returns the count of prime pairs at every even number from four up to the top, one entry per even number."""

def is_prime(number: int) -> bool:
    """Returns whether the number is prime, by trial division on the six-step wheel."""

def pile(number: int) -> dict[str, Any]:
    """Reads a wide number as a pile of stones, its rectangles built from the divisors of its factorization."""

def prime_count(n: int) -> int:
    """Returns the count of primes at or below n."""

def prime_from(number: int) -> int:
    """Returns the smallest prime at or above the number."""

def primes(limit: int) -> list[int]:
    """Returns the primes up to the limit, the finished sieve read as a list."""

def rectangles(number: int) -> list[tuple[int, int]]:
    """Returns every rectangle of the number as a pair of sides, the shorter first, ascending: the divisors at or below the root."""

def splits(number: int) -> list[tuple[int, int]]:
    """Returns every pair of primes summing to the number, odd numbers included, the smaller first, ascending."""

def squares(number: int) -> tuple[int, int] | None:
    """Returns the smallest pair of positive sides whose squares sum to the number, when one exists."""

def study(limit: int) -> list[dict[str, Any]]:
    """Returns one prime object for every prime up to and including the limit."""
