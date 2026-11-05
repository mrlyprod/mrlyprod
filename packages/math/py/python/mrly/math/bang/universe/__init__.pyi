from typing import Any, Literal

from numpy.typing import NDArray

def anf(code: int, dimension: int) -> bytes:
    ...

def anf_string(code: int, dimension: int) -> str:
    ...

def apply(element: tuple[list[int], bytes], corner: bytes) -> bytes:
    ...

def corner_index(corner: bytes) -> int:
    ...

def degree(code: int, dimension: int) -> int:
    ...

def orbit(code: int, dimension: int) -> list[int]:
    ...

def permutations(n: int) -> list[list[int]]:
    ...
