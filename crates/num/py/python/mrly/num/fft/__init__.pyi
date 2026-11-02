from typing import Any, Literal

from numpy.typing import NDArray

def convolve(field: list[float], kernel: list[float], size: int) -> list[float]:
    """Circularly convolves a size-square field on the torus by a kernel of the same shape through fft2 both ways."""

def convolve_with(field: list[float], kernel_re: list[float], kernel_im: list[float], size: int) -> list[float]:
    """Convolves a size-square field on the torus by a kernel already transformed by fft2, the inverse scaled back by size squared."""

def embed_kernel(mask: bytes, side: int, size: int) -> list[float]:
    """Lays an odd-side mask into a size-square kernel with the mask centre at index (0, 0) and negative offsets wrapped; the cell at offset (dr, dc) lands at (-dr, -dc) modulo size, so convolving a field by the kernel reads at every site the mask-weighted sum over its neighbours, the neighbour count the life step counts."""

def log_spectrum(field: list[float], size: int) -> list[float]:
    """Returns the centred magnitude spectrum of a size-square field through log(1 + magnitude), the DC bin included at the centre."""

def magnitude_spectrum(field: list[float], size: int) -> list[float]:
    """Returns the magnitudes of a square field's transform, shifted so zero frequency sits at the centre."""

def peak_ring(profile: list[float]) -> int:
    """Finds the ring past the centre where a radial profile peaks, a tie broken at the smaller ring; zero when the profile holds no ring past ring 0."""

def peak_wavelength(profile: list[float], size: int) -> float:
    """Reads the wavelength in cells at a radial profile's peak, size over the peak ring with a tie broken at the smaller ring; zero when the profile holds no ring past ring 0."""

def radial_profile(spectrum: list[float], size: int) -> list[float]:
    """Averages a centred size-square spectrum over rings of integer radius from the centre bin, a bin joining the ring its distance rounds to, rings 0 through size over two; ring k holds the frequencies near k cycles per field."""

def transform(field: list[float], size: int) -> tuple[list[float], list[float]]:
    """Transforms a real size-square field forward by fft2, returning the real and imaginary parts."""
