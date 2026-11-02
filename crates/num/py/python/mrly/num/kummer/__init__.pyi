from typing import Any, Literal

from numpy.typing import NDArray

MOST: int
TABLE: int

def carry(prime: int, top: int, bottom: int) -> dict[str, Any]:
    """Builds the carry automaton of `K_(a,b)` at the prime, `a` the top and `b` the bottom."""

def columns(k: int, prime: int, top: int, bottom: int) -> list[dict[str, Any]]:
    """Returns the columns the automaton reads for `k`, units first: one per digit of `k`, then the zeros that spend the carries, ending at the first column that carries or once every digit is read and the state is `(0, 0)` again."""

def count(prime: int, top: int, bottom: int, level: int) -> int:
    """Returns the count of `k < p^level` in `K_(a,b)`: the words of length `level` the automaton allows, summed by the state they end in over the states that close."""

def member(k: int, prime: int, top: int, bottom: int) -> bool:
    """Returns whether `k` lies in `K_(a,b)`, read by the carry automaton: whether the prime does not divide `C(a k, b k)`."""

def valuation(k: int, prime: int, top: int, bottom: int) -> int:
    """Returns `v_p(C(a k, b k))` by Legendre's digit sums, `(s_p(b k) + s_p((a - b) k) - s_p(a k))/(p - 1)` with `s_p` the digit sum in base `p`: the carries of `b k + (a - b) k`."""

def witness(prime: int, top: int, bottom: int) -> tuple[int, int] | None:
    """Returns the least witness that `K_(a,b)` is no digit design, as the pair `(u, v p)` of elements: digits `u, v < p` with `u` and `v p` in the set and `u + v p` outside it, least `u + v p` first; a digit design holding `u` and `v p` holds `u + v p`. `None` when no two such digits exist, as on the half interval `K_(2,1)`."""
