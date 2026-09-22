from typing import Any, Literal

from numpy.typing import NDArray

class Nodes:
    """Every crossing of a traced roulette: the curves against themselves, the curves against one another, and how crowded the worst node is."""
    @property
    def curves(self) -> int:
        """The curves counted, in the order the pencils came in."""
    @property
    def selves(self) -> list[int]:
        """How often each curve crosses itself, curve by curve."""
    @property
    def pairs(self) -> list[int]:
        """How often each pair of curves crosses, the lower curve first, in lexicographic order."""
    @property
    def most(self) -> int:
        """The most crossings one node carries: one at a plain double point, and `n(n - 1)/2` where `n` branches meet."""
    @property
    def crowded(self) -> int:
        """The nodes more than one crossing clusters at."""
    @property
    def points(self) -> int:
        """The distinct points the crossings sit at, one for every cluster."""
    @property
    def branches(self) -> int:
        """The branches through every node added up, which is the edge count of the picture as a plane graph, `n` at a node where `n` branches meet and `2` times `points` when no node is crowded."""
    @property
    def touches(self) -> int:
        """The segment pairs that meet without crossing: collinear or end to end."""
    def pair(self, i: int, j: int) -> int:
        """How often the curves `i` and `j` cross, either order, and zero when they are one curve."""
    def paired(self) -> int:
        """Every crossing of two curves."""
    def selved(self) -> int:
        """Every self crossing."""
    def total(self) -> int:
        """Every crossing, self and pair together, which counts a node where `n` branches meet `n(n - 1)/2` times; `points` is the count of distinct nodes and the two agree exactly when `crowded` is zero."""
    @staticmethod
    def from_dict(data: Any) -> Nodes:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

def nodes(track: dict[str, Any], pencils: list[dict[str, Any]], samples: int, tol: float) -> Nodes:
    """Counts the nodes of the roulette the pencils draw on the track: `mrlyrs::math::spirograph::trace` at `samples` points a pencil, every pair of polyline segments tested for a proper crossing by orientation signs on a grid of buckets, and crossings within `tol` of the picture's longer side read as one node. A pair of segments is counted in one bucket alone, the first they share, so no crossing is counted twice; the sign of an orientation is `side`, exact for any endpoints whose two differences are exact, which two `f32` endpoints are while the picture's coordinates keep their exponents within 29 of one another, as these pictures do. A seat at the wheel's centre draws one circle `b` times over and the count is meaningless there, the passes crossing one another as the sampling wanders."""

def side(a: list[float], b: list[float], c: list[float]) -> int:
    """Which side of the line from `a` to `b` the point `c` lies: plus one to the left, minus one to the right, zero on it. The sign is exact whenever the two differences `b - a` and `c - a` are exact, whatever the size of the products: the determinant is taken by the fused multiply-add identity of Kahan, whose error is at most twice the rounding unit times the determinant itself, so it can neither flip a sign nor invent one."""

def spread(track: dict[str, Any], pencils: list[dict[str, Any]], exact: bool) -> list[dict[str, Any]]:
    """One pencil for every distinct curve, the coincidence law read on the exact seats when `exact` says the seats carry no jitter: the first pencil of each family, in the order they came in. On a circle the seats fall into classes under the rotation group of order `gcd(b, 4)`, which is the clause `mrlyrs::math::spirograph::distinct` and `mrlyrs::math::spirograph::representatives` read; on a line and on a polygon every distinct seat draws its own curve, two seats of one radius on a line drawing translates of one shape and never one curve."""
