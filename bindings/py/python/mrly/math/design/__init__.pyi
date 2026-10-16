from typing import Any, Literal

from numpy.typing import NDArray

ZETA_ORDINATES: list[float]

def digits_of(mask: int, base: int) -> list[int]:
    """Returns the digits a bitmask names inside the base, ascending."""

def echo_series(values: list[int], log_x: list[float], exponent: float) -> list[float]:
    """Returns the density echo, the sum of mu(n) A_F(n)/n over the whole numbers up to each grid point divided by x to the exponent, sieving the Mobius values to the largest element."""

def elements(base: int, digits: list[int], depth: int) -> list[int]:
    """Returns the elements of the digit design below the base raised to the depth, ascending: the whole numbers of at most that many base digits, every digit drawn from the set and the leading digit nonzero."""

def log_grid(values: list[int], samples: int) -> list[float]:
    """Returns the log grid uniform over the span of the elements, from the log of the first to the log of the last."""

def median_floor(power: list[float], width: int) -> list[float]:
    """Returns the running median of the power over a window of the given width, the window clamped at the ends."""

def meter(mu: list[int]) -> list[int]:
    """Returns the running design Mobius meter, the partial sums of the Mobius values along the elements."""

def nearest(value: float, list: list[float]) -> float:
    """Returns the distance from the ordinate to the nearest entry of the list, infinite when the list is empty."""

def peaks(gamma: list[float], score: list[float], band: tuple[float, float], threshold: float) -> list[int]:
    """Returns the bins inside the band that rise above both neighbours and clear the score threshold, strongest first."""

def pole_lattice(base: int, top: float) -> list[float]:
    """Returns the design's pole lattice below the top, the ordinates 2 pi j over log q of the poles its Dirichlet series carries."""

def resample(values: list[int], running: list[int], exponent: float, log_x: list[float]) -> list[float]:
    """Reads the running meter at every point of the log grid and divides by x to the exponent."""

def score(power: list[float], width: int) -> list[float]:
    """Returns the power over its local median floor, the score a peak is read against."""

def size(digits: list[int], depth: int) -> int:
    """Returns the count of elements the design holds at the depth, the length [`elements`] returns without building them."""

def spectrum(log_x: list[float], series: list[float]) -> tuple[list[float], list[float]]:
    """Returns the frequency axis and the power spectrum of the series: the mean removed, a Hann window laid on, a real transform taken, and bin j read as the ordinate 2 pi j over the log range."""

def upper_rms(series: list[float]) -> float:
    """Returns the root mean square of the upper half of the series, the size the echo and the meter are compared at."""
