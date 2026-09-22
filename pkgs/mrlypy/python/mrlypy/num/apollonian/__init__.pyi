from typing import Any, Literal

from numpy.typing import NDArray

CIRCLE_CAP: int
CURVATURE_CAP: int
ORDER_CAP: int
ROOTS: list[str]

class Circle:
    """A circle in the integer coordinates `(k, k x, k y)`: a line is `k = 0` with `(k x, k y)` its outward unit normal, and the curvature is negative on the circle that contains a bounded packing."""
    @property
    def k(self) -> int:
        """The curvature."""
    @property
    def x(self) -> int:
        """The curvature times the centre's abscissa."""
    @property
    def y(self) -> int:
        """The curvature times the centre's ordinate."""
    def centre(self) -> tuple[float, float] | None:
        """The centre, none on a line."""
    def is_line(self) -> bool:
        """Whether the circle is a line."""
    def radius(self) -> float | None:
        """The radius, none on a line."""
    @staticmethod
    def from_dict(data: Any) -> Circle:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

def form(u: list[int], v: list[int]) -> int:
    """The bilinear form `B(u, v) = (sum u)(sum v) - 2 sum u v` that the reflection preserves."""

def frame(p: dict[str, Any]) -> list[float]:
    """The box the packing is drawn in: one period of the strip, or the box of the circle that contains a bounded packing."""

def grow(name: str, cap: int) -> dict[str, Any]:
    """Grows the named packing to the curvature cap, one circle per node of the reflection tree and the root quadruple excluded, so `circles.len()` is the census `N(T)`. On the strip only the two root swaps that replace a line are taken, which are exactly the two that stay inside one period."""

def is_ford(c: Circle) -> bool:
    """Whether the circle is the Ford circle over its own tangency point: curvature `2 b^2` and abscissa `2 a b` at the reduced `a/b`."""

def on_line(c: Circle) -> bool:
    """Whether the circle has positive curvature and is tangent to the line `y = 0`, which in these coordinates reads `k > 0` and `k y = 1`: the curvature guard is what excludes the line `y = 1`, which is `(0, 0, 1)`."""

def reflect(q: list[Circle], at: int) -> Circle:
    """Reflects the circle at the seat through the other three, `v' = 2(v_1 + v_2 + v_3) - v` on all three coordinates at once, which is the second root of the Descartes quadratic and needs no square root."""

def root(name: str) -> list[Circle]:
    """The named root quadruple: `strip` is the two lines a unit apart holding the circles at `0` and `1`, and the rest are bounded packings named by their four curvatures."""

def shadow(p: dict[str, Any], order: int) -> dict[str, Any]:
    """Reads the Farey stack of the order against the packing: the nodes lit inside the open period against the tangency points of the line-tangent circles of curvature at most `2 Q^2`, and the brightness `floor(Q/b)` summed on the nodes against `Q(Q + 1)/2`. Off the strip there is no line and every count is zero."""

def sound(q: list[Circle]) -> bool:
    """Whether the quadruple carries all six exact invariants: Descartes `B(k, k) = 0`, the position half `B(k, kx) = B(k, ky) = B(kx, ky) = 0`, and the frame `B(kx, kx) = B(ky, ky) = -4`."""

def swap(q: list[Circle], at: int) -> list[Circle]:
    """The quadruple with the circle at the seat replaced by its reflection."""

def touches(p: dict[str, Any]) -> list[dict[str, Any]]:
    """The tangency points on the line `y = 0`, ascending: one per circle of the packing with `k y = 1`, the root excluded. Empty off the strip."""
