from typing import Any, Literal

from numpy.typing import NDArray
import mrlypy.math.graph

def clusters(eigenvalues: list[float], tolerance: float) -> list[tuple[float, int]]:
    """Groups eigenvalues into runs split by consecutive gaps above the tolerance, each run its mean and its size."""

def laplacian(network: mrlypy.math.graph.Network, normalised: bool) -> list[list[float]]:
    """Builds the Laplacian of a network, the combinatorial `D - A` or the normalised `I - D^-1/2 A D^-1/2`."""

def laplacian_spectrum(network: mrlypy.math.graph.Network, normalised: bool) -> list[float]:
    """Returns the ascending Laplacian spectrum of a network, combinatorial or normalised."""

def multiplicity(eigenvalues: list[float], value: float, tolerance: float) -> int:
    """Counts the eigenvalues within the tolerance of a value."""

def spectral_exponent(eigenvalues: list[float], window: float) -> float | None:
    """Reads the spectral exponent: twice the log-log slope of the integrated density of states over its low window."""

def spectral_fit(eigenvalues: list[float], window: float) -> tuple[float, float, int] | None:
    """Fits the low window of the integrated density of states in log-log: the intercept, the slope and the fitted count."""

def spectral_points(eigenvalues: list[float]) -> list[tuple[float, float]]:
    """Builds the integrated density of states as points, each an eigenvalue and its rank fraction."""

def symmetric_eigenvalues(matrix: list[list[float]]) -> list[float]:
    """Returns the eigenvalues of a dense real symmetric matrix in ascending order."""
