from typing import Any, Literal

from numpy.typing import NDArray

class Share:
    """The exact reading of a cut layer: how many cells were inked out of how many were read."""
    @property
    def inked(self) -> int:
        """The count of inked cells."""
    @property
    def cells(self) -> int:
        """The count of cells read."""
    def reduced(self) -> tuple[int, int]:
        """The share in lowest terms, numerator then denominator."""
    def value(self) -> float:
        """The share as a real number."""
    @staticmethod
    def from_dict(data: Any) -> Share:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

class Star:
    """The ghost star of a coded cube's hexagonal cut stack, read in the cell frame."""
    def __init__(self, code: int) -> None: ...
    def arm(self, number: int, half: int) -> Share:
        """The exact ink share of the band of half-width `W` cells about the arm `x = y` at odd `n`."""
    def cell(self, number: int, x: int, z: int) -> bool | None:
        """The ink of the cut cell at column `x` and even height `z` of the layer at odd `n`."""
    def excesses(self, layers: int, half: int) -> list[float]:
        """The per-layer excess of the star band over the hexagon across the first `L` odd layers."""
    def hexagon(self, number: int) -> Share:
        """The exact ink share of the whole hexagonal cut at odd `n`, the background the star is read against."""
    @staticmethod
    def new(code: int) -> Star:
        """Reads the star of a base-2 space code, the carpet being `23`."""
    @staticmethod
    def from_dict(data: Any) -> Star:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

class Branch:
    """The three classes of layer count the `1/L^2` term of the decay reads."""
    @staticmethod
    def constant(branch: Literal["Zero", "Two", "Odd"]) -> float:
        """The constant the ladder converges on, `C` at even `L` and `C + 1/8` at odd `L`."""
    @staticmethod
    def name(branch: Literal["Zero", "Two", "Odd"]) -> str:
        """The name of the branch."""
    @staticmethod
    def of(layers: int) -> Literal["Zero", "Two", "Odd"]:
        """The branch of a layer count."""
    @staticmethod
    def residual(branch: Literal["Zero", "Two", "Odd"]) -> float | None:
        """The exact `1/L^2` coefficient at even `L`, absent at odd `L`."""

def arm_law(number: int) -> Share:
    """The closed form of the star arm's ink at odd `n`, `1/2 + chi_8(n)/(2n)`, as `n + chi_8(n)` cells of `2n`."""

def chi8(number: int) -> int:
    """The real character mod 8 of `Q(sqrt 2)`: `+1` at `n = 1, 7`, `-1` at `n = 3, 5`, zero at even `n`."""

def constant() -> float:
    """The constant beside the decay, `ln(1 + sqrt 2)/(2 sqrt 2) - G/8 - gamma/4 - (ln 2)/2`."""

def decay(excesses: list[float], layers: int) -> dict[str, Any]:
    """The decay read off the per-layer excesses at a layer count, the slope taken from `L/2` to `L`."""

def width_law(half: int) -> float:
    """The cell-frame decay coefficient of a band of half-width `W` cells, `-(K + b)/(4(2K + 1))` for `K = floor(W/2)`."""
