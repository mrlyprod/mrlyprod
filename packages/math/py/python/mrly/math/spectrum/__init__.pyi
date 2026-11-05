from typing import Any, Literal

from numpy.typing import NDArray
import mrly.math.graph

def clusters(eigenvalues: list[float], tolerance: float) -> list[tuple[float, int]]:
    ...

def laplacian(network: mrly.math.graph.Network, normalised: bool) -> list[list[float]]:
    ...

def laplacian_spectrum(network: mrly.math.graph.Network, normalised: bool) -> list[float]:
    ...

def multiplicity(eigenvalues: list[float], value: float, tolerance: float) -> int:
    ...

def spectral_exponent(eigenvalues: list[float], window: float) -> float | None:
    ...

def spectral_fit(eigenvalues: list[float], window: float) -> tuple[float, float, int] | None:
    ...

def spectral_points(eigenvalues: list[float]) -> list[tuple[float, float]]:
    ...

def symmetric_eigenvalues(matrix: list[list[float]]) -> list[float]:
    ...
