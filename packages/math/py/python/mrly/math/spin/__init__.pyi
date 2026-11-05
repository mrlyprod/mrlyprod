from typing import Any, Literal

from numpy.typing import NDArray

class Blend:
    @staticmethod
    def fold(blend: Literal["Mean", "Sum", "Union", "Meet", "Parity", "Difference"], values: list[float]) -> float:
        ...
    @staticmethod
    def named(name: str) -> Literal["Mean", "Sum", "Union", "Meet", "Parity", "Difference"] | None:
        ...

def arcs(data: list[float], size: int, radius: float) -> list[tuple[float, float, float]]:
    ...

def harmonics(data: list[float], size: int, rings: int, orders: int) -> list[float]:
    ...

def mass(profile: list[float], size: int) -> float:
    ...

def mass_within(profile: list[float], size: int, radius: float) -> float:
    ...

def petals(copies: int, order: int) -> int:
    ...

def profile(data: list[float], size: int, steps: int) -> list[float]:
    ...

def radial(data: list[float], size: int, out: int, copies: int, step: float, blend: Literal["Mean", "Sum", "Union", "Meet", "Parity", "Difference"], samples: int) -> list[float]:
    ...

def reach(size: int) -> float:
    ...

def ring(data: list[float], size: int, radius: float) -> float:
    ...

def turns(power: list[float]) -> int:
    ...

def wheel(profile: list[float], size: int) -> list[float]:
    ...
