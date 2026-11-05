from typing import Any, Literal

from numpy.typing import NDArray

ZETA_ORDINATES: list[float]

def digits_of(mask: int, base: int) -> list[int]:
    ...

def echo_series(values: list[int], log_x: list[float], exponent: float) -> list[float]:
    ...

def elements(base: int, digits: list[int], depth: int) -> list[int]:
    ...

def log_grid(values: list[int], samples: int) -> list[float]:
    ...

def median_floor(power: list[float], width: int) -> list[float]:
    ...

def meter(mu: list[int]) -> list[int]:
    ...

def nearest(value: float, list: list[float]) -> float:
    ...

def peaks(gamma: list[float], score: list[float], band: tuple[float, float], threshold: float) -> list[int]:
    ...

def pole_lattice(base: int, top: float) -> list[float]:
    ...

def resample(values: list[int], running: list[int], exponent: float, log_x: list[float]) -> list[float]:
    ...

def score(power: list[float], width: int) -> list[float]:
    ...

def size(digits: list[int], depth: int) -> int:
    ...

def spectrum(log_x: list[float], series: list[float]) -> tuple[list[float], list[float]]:
    ...

def upper_rms(series: list[float]) -> float:
    ...
