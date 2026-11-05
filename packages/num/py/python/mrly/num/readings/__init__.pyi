from typing import Any, Literal

from numpy.typing import NDArray

READINGS: list[Literal["N", "Square", "SquarePlusN", "Cube", "Primes"]]

class Design:
    @staticmethod
    def digits(base: int, mask: int) -> dict[str, Any]:
        ...
    @staticmethod
    def rudin() -> dict[str, Any]:
        ...
    @staticmethod
    def ternary() -> dict[str, Any]:
        ...
    @staticmethod
    def thue() -> dict[str, Any]:
        ...

class Reading:
    @staticmethod
    def all() -> list[Literal["N", "Square", "SquarePlusN", "Cube", "Primes"]]:
        ...
    @staticmethod
    def formula(reading: Literal["N", "Square", "SquarePlusN", "Cube", "Primes"]) -> str:
        ...

def arcs(design: dict[str, Any], cut: float) -> list[dict[str, Any]]:
    ...

def battery(stream: list[int]) -> dict[str, Any]:
    ...

def bias(stream: list[int], exponents: list[int]) -> list[int]:
    ...

def designs() -> list[dict[str, Any]]:
    ...

def model(design: dict[str, Any], cut: float, exponents: list[int]) -> list[float]:
    ...

def name(design: dict[str, Any]) -> str:
    ...

def near_heads(design: dict[str, Any], reading: Literal["N", "Square", "SquarePlusN", "Cube", "Primes"]) -> list[dict[str, Any]]:
    ...

def sign(design: dict[str, Any], n: int) -> int:
    ...

def stream(design: dict[str, Any], reading: Literal["N", "Square", "SquarePlusN", "Cube", "Primes"], count: int) -> list[int]:
    ...
