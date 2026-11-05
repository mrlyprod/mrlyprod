from typing import Any, Literal

from numpy.typing import NDArray

APERY: float
BASEL: float
CATALAN: float
EULER: float
VISIBLE: float

def basel(n: int) -> float:
    ...

def bernoulli(count: int) -> list[tuple[int, int]]:
    ...

def beta(s: float, terms: int) -> float:
    ...

def binary(limit: int) -> list[int]:
    ...

def catalan(limit: int) -> list[int]:
    ...

def chi3(number: int) -> int:
    ...

def chi4(number: int) -> int:
    ...

def chi8(number: int) -> int:
    ...

def dirichlet(s: float, rhythm: list[int], terms: int) -> float:
    ...

def e_partial(n: int) -> float:
    ...

def euler_gamma_partial(n: int) -> float:
    ...

def euler_product(s: float, limit: int) -> float:
    ...

def evens(limit: int) -> list[int]:
    ...

def fibonacci(limit: int) -> list[int]:
    ...

def harmonic(terms: int) -> float:
    ...

def lambda_(s: float, terms: int) -> float:
    ...

def leibniz(n: int) -> float:
    ...

def li(x: float) -> float:
    ...

def mertens(n: int) -> int:
    ...

def odds(limit: int) -> list[int]:
    ...

def visible(limit: int, dimension: int) -> int:
    ...

def wallis_half_pi(n: int) -> float:
    ...

def wallis_quarter_pi(factors: int) -> float:
    ...

def zeta(s: float, terms: int) -> float:
    ...
