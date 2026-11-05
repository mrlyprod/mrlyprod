from typing import Any, Literal

from numpy.typing import NDArray

LIFTS: list[Literal["Parity", "And", "Xor", "Sum"]]

class Lift:
    @staticmethod
    def all() -> list[Literal["Parity", "And", "Xor", "Sum"]]:
        ...
    @staticmethod
    def at(lift: Literal["Parity", "And", "Xor", "Sum"], i: int, j: int) -> int:
        ...
    @staticmethod
    def formula(lift: Literal["Parity", "And", "Xor", "Sum"]) -> str:
        ...

def boundary(word: bytes) -> bytes:
    ...

def difference(a: bytes, b: bytes) -> bytes:
    ...

def digits(length: int) -> bytes:
    ...

def doubling(length: int) -> bytes:
    ...

def faults(a: bytes, b: bytes) -> int:
    ...

def fold(grid: bytes, side: int, number: int) -> dict[str, Any]:
    ...

def letter(place: int) -> int:
    ...

def lift(kind: Literal["Parity", "And", "Xor", "Sum"], side: int) -> bytes:
    ...

def mask(a: bytes, b: bytes) -> bytes:
    ...

def power(tile: bytes, number: int, level: int) -> bytes:
    ...

def repeat(tile: bytes, number: int, side: int) -> bytes:
    ...

def runs(word: bytes) -> list[int]:
    ...

def signs(types: bytes) -> bytes:
    ...

def stage(rounds: int) -> bytes:
    ...

def substitution(length: int) -> bytes:
    ...

def upsample(grid: bytes, side: int, scale: int) -> bytes:
    ...
