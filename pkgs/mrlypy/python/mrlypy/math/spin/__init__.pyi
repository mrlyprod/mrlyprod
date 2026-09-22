from typing import Any, Literal

from numpy.typing import NDArray

class Blend:
    """The way radial copies merge: their mean, their sum, their union, their meet, their parity or what the first keeps that no other has."""
    @staticmethod
    def fold(blend: Literal["Mean", "Sum", "Union", "Meet", "Parity", "Difference"], values: list[float]) -> float:
        """Merges one site's copies into the blended value."""
    @staticmethod
    def named(name: str) -> Literal["Mean", "Sum", "Union", "Meet", "Parity", "Difference"] | None:
        """Reads a blend by name: mean, sum, union, meet, parity or difference."""

def arcs(data: list[float], size: int, radius: float) -> list[tuple[float, float, float]]:
    """The arcs of the circle of the radius about the raster's centre: each as its start angle, end angle and the value of the one cell it lies in, zero outside."""

def harmonics(data: list[float], size: int, rings: int, orders: int) -> list[float]:
    """The circular-harmonic power of a raster: for every order `m` up to the last, the energy `sum |c_m(r)|^2 2 pi r dr` of its `m`-th harmonic over rings radii, each ring's coefficient exact from its arcs."""

def mass(profile: list[float], size: int) -> float:
    """The mass a profile carries, the trapezoid integral of `2 pi r F(r)` in cells of the raster it came from."""

def mass_within(profile: list[float], size: int, radius: float) -> float:
    """The mass a profile carries inside the radius, the trapezoid integral of `2 pi r F(r)` from the centre out, in cells of the raster it came from."""

def petals(copies: int, order: int) -> int:
    """The petals a full radial stack of the copies shows on a design of the rotation order: their least common multiple."""

def profile(data: list[float], size: int, steps: int) -> list[float]:
    """The ring profile: the circle means at steps radii spaced evenly from the centre to the corner circle."""

def radial(data: list[float], size: int, out: int, copies: int, step: float, blend: Literal["Mean", "Sum", "Union", "Meet", "Parity", "Difference"], samples: int) -> list[float]:
    """Stacks a raster radially: copies turned by multiples of the step, in turns, about the centre and merged by the blend, on an output raster of the side whose inscribed circle is the source's corner circle, every pixel the mean of samples by samples points."""

def reach(size: int) -> float:
    """The radius of the corner circle of a square raster of the side, the last radius a profile reads."""

def ring(data: list[float], size: int, radius: float) -> float:
    """The exact mean of a square raster over the circle of the radius about its centre, each cell read as a constant and the outside as zero."""

def turns(power: list[float]) -> int:
    """The rotation order a harmonic power spectrum reveals: the gcd of the orders carrying more than a ten-thousandth of the power, the share pixel aliasing stays under, or zero when none does."""

def wheel(profile: list[float], size: int) -> list[float]:
    """The wheel: a profile spread over a square raster of the side, the corner circle it ends on drawn as the inscribed circle, every pixel reading the profile at its own radius."""
