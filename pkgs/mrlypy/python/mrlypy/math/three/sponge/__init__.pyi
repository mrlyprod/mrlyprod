from typing import Any, Literal

from numpy.typing import NDArray

COVER: float
EDGE: float

def deep(radius: float) -> float:
    """The volume `Deep(radius)` of the points of one arm's quarter beyond the tubes of both walls it touches, at every radius, to double precision."""

def dimension() -> float:
    """The Minkowski dimension of the Menger sponge, `log(20)/log(3)`, the similarity dimension of its 20 maps of ratio `1/3`."""

def distance(point: list[float]) -> float:
    """The Euclidean distance from a point of the unit cube to the Menger sponge, exact to the last binary place."""

def exact(radius: float) -> float:
    """The volume `T(radius)` of the points of the plus within `radius` of the sponge at every radius, `Deep` included, to double precision."""

def profile(radius: float) -> float | None:
    """The periodic function `p` of Kombrink, Pearse and Winter at `radius`: the reading's limit profile, unchanged when the radius is multiplied by 3, or `None` on the phases `(1/6, sqrt(2)/6]` where `T` has no closed form."""

def reading(radius: float) -> float | None:
    """The Minkowski reading `radius^(D-3)` times the volume inside the cube, the number whose limit as the radius shrinks would be the sponge's Minkowski content."""

def tube(radius: float) -> float | None:
    """The volume `T(radius)` of the points of the plus of seven removed level-1 cubes within `radius` of the sponge, or `None` on `(1/6, sqrt(2)/6)`, where no closed form is known."""

def volume(radius: float) -> float | None:
    """The volume of the points of the unit cube within `radius` of the sponge, `sum_k (20/27)^k T(3^k radius)`, or `None` when some `3^k radius` falls where `T` has no closed form."""
