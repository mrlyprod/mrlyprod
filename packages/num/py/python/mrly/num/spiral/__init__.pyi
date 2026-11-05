from typing import Any, Literal

from numpy.typing import NDArray

class Growth:
    @staticmethod
    def all() -> list[Literal["Prime", "Every"]]:
        ...

class Lattice:
    @staticmethod
    def all() -> list[Literal["Square", "Hex"]]:
        ...
    @staticmethod
    def count(lattice: Literal["Square", "Hex"], side: int) -> int:
        ...
    @staticmethod
    def n(lattice: Literal["Square", "Hex"], x: int, y: int) -> int:
        ...
    @staticmethod
    def radius(lattice: Literal["Square", "Hex"], side: int) -> int:
        ...
    @staticmethod
    def ring(lattice: Literal["Square", "Hex"], n: int) -> int:
        ...
    @staticmethod
    def ring_of(lattice: Literal["Square", "Hex"], x: int, y: int) -> int:
        ...
    @staticmethod
    def xy(lattice: Literal["Square", "Hex"], n: int) -> tuple[int, int]:
        ...

class Mark:
    @staticmethod
    def all() -> list[Literal["Prime", "Twin", "Squarefree", "Mobius"]]:
        ...

def diagonal(lattice: Literal["Square", "Hex"], side: int, a: int, b: int, c: int) -> dict[str, Any]:
    ...

def level_of(n: int, base: int) -> int:
    ...

def marks(mark: Literal["Prime", "Twin", "Squarefree", "Mobius"], limit: int) -> list[int]:
    ...

def snail(base: int, top: int, growth: Literal["Prime", "Every"]) -> dict[str, Any]:
    ...
