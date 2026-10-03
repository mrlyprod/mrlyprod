from typing import Any, Literal

from numpy.typing import NDArray

DEEPEST: int
WIDEST: int

class Pair:
    """A pair of levels: the base-3 level `A_k = A meet [0, 3^k)` against the base-4 level `B_m = B meet [0, 4^m)`, `k` the field `three` and `m` the field `four`."""
    def __init__(self, three: int, four: int) -> None: ...
    @property
    def three(self) -> int:
        """The base-3 level `k`."""
    @three.setter
    def three(self, value: int) -> None: ...
    @property
    def four(self) -> int:
        """The base-4 level `m`."""
    @four.setter
    def four(self, value: int) -> None: ...
    def clean(self) -> bool:
        """Whether the pair is clean, `3^k > d(k, m)` and `4^m > d(k, m)`, so that `S meet [0, d] = A_k + B_m`."""
    def copy(self) -> bool:
        """Whether the pair is a gap copy, `2 4^m < 3^k + 5`: `A_k + B_m` is then two disjoint translates of `A_(k-1) + B_m` and its energy is twice theirs."""
    def energy(self) -> int:
        """The additive energy `E(k, m) = sum_x r(x)^2`, `r(x)` the number of ways `x = a + b` with `a` in `A_k` and `b` in `B_m`."""
    def gap(self) -> tuple[int, int] | None:
        """The first and the last integer of the open interval `(d(k, m), min(3^k, 4^m))`, which `S` misses, or `None` when it holds none."""
    def largest(self) -> int:
        """The largest element `d(k, m) = (3^k - 1)/2 + (4^m - 1)/3` of `A_k + B_m`."""
    @staticmethod
    def new(three: int, four: int) -> Pair:
        """Names the pair `(k, m)`."""
    def ratio(self, energy: int) -> float:
        """The energy ratio `Q(k, m) = E(k, m) (d + 1)/4^(k+m)` of the energy [`Pair::energy`] returns, the energy against its flat value, at least `1`; `card(A_k + B_m) >= (d + 1)/Q` by Cauchy-Schwarz."""
    def scale(self) -> float:
        """The scaling `tau = 4^m/3^k`."""
    @staticmethod
    def from_dict(data: Any) -> Pair:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

class Sumset:
    """The sumset `S = A + B` of Erdos problem 125 up to `3^level`: `A` the integers whose base-3 digits are all `0` or `1`, `B` those whose base-4 digits are."""
    def __init__(self, level: int) -> None: ...
    def contains(self, x: int) -> bool | None:
        """Whether `x` is in `S`, or `None` past the top."""
    def count(self, x: int) -> int | None:
        """Counts `card(S meet [1, x])`, or `None` past the top."""
    def density(self, x: int) -> float | None:
        """Reads the density `D(x) = card(S meet [1, x])/x`, or `None` at zero and past the top."""
    def extremes(self, edges: list[int]) -> list[tuple[float, float] | None]:
        """Reads the least and the greatest `D(x)` over each window `[edges[i], edges[i + 1])`, `None` for an empty window."""
    def fills(self, low: int, high: int, cells: int) -> list[float]:
        """Reads the share of members in each of `cells` equal runs of the integers `[low, high)`, the strip of `S` a page draws."""
    def level(self) -> int:
        """The level the array was built to."""
    @staticmethod
    def new(level: int) -> Sumset:
        """Builds `S meet [0, 3^level]`."""
    def top(self) -> int:
        """The largest integer the array holds, `3^level`."""

def pairs(level: int) -> list[Pair]:
    """Lists the pairs of the census: every `(k, m)` with `4^m` within a factor `3` of `3^k` and `d(k, m) <= 3^level`, by `d`."""
