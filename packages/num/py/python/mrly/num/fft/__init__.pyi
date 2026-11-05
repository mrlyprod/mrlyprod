from typing import Any, Literal

from numpy.typing import NDArray

def convolve(field: list[float], kernel: list[float], size: int) -> list[float]:
    ...

def convolve_with(field: list[float], kernel_re: list[float], kernel_im: list[float], size: int) -> list[float]:
    ...

def embed_kernel(mask: bytes, side: int, size: int) -> list[float]:
    ...

def log_spectrum(field: list[float], size: int) -> list[float]:
    ...

def magnitude_spectrum(field: list[float], size: int) -> list[float]:
    ...

def peak_ring(profile: list[float]) -> int:
    ...

def peak_wavelength(profile: list[float], size: int) -> float:
    ...

def radial_profile(spectrum: list[float], size: int) -> list[float]:
    ...

def transform(field: list[float], size: int) -> tuple[list[float], list[float]]:
    ...
