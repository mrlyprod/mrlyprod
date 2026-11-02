from typing import Any, Literal

from numpy.typing import NDArray

CLASSICS_2D: list[Literal["Carpet", "Net", "Htree", "Vtree", "Void", "Xtree", "Ytree", "Ztree", "Point", "Dust", "Hline", "Vline", "Star", "Xline", "Yline", "Zline"]]
CLASSICS_3D: list[Literal["Carpet", "Net", "Htree", "Vtree", "Void", "Xtree", "Ytree", "Ztree", "Point", "Dust", "Hline", "Vline", "Star", "Xline", "Yline", "Zline"]]
MAX_LEVEL: int
MAX_SIDE: int
MAX_SLOTS: int
MIN_SIDE: int

class Design:
    """The named designs a source can point at: the four classics and their four antis."""
    @staticmethod
    def all() -> list[Literal["Carpet", "Net", "Htree", "Vtree", "Void", "Xtree", "Ytree", "Ztree", "Point", "Dust", "Hline", "Vline", "Star", "Xline", "Yline", "Zline"]]:
        """Returns every Design in canonical order."""

def classics(dimension: int) -> list[Literal["Carpet", "Net", "Htree", "Vtree", "Void", "Xtree", "Ytree", "Ztree", "Point", "Dust", "Hline", "Vline", "Star", "Xline", "Yline", "Zline"]]:
    """Returns the classic designs for a dimension."""

def generals(min_size: int, max_size: int, parity: Literal["Evens", "Odds", "Both"]) -> list[int]:
    """Returns every flat size in the range that passes the parity filter."""

def nestings(min_size: int, max_size: int, parity: Literal["Evens", "Odds", "Both"]) -> list[list[int]]:
    """Returns every factor list of depth two and beyond whose product lands in the size range."""

def powers(min_size: int, max_size: int, parity: Literal["Evens", "Odds", "Both"]) -> list[tuple[int, int]]:
    """Returns every factor and level whose power lands in the size range."""

def products(min_size: int, max_size: int, count: int, parity: Literal["Evens", "Odds", "Both"]) -> list[list[int]]:
    """Returns every count-long factor list whose product lands in the size range."""

def size(number: int, level: int) -> int | None:
    """Returns the side a factor raised to a level makes, or None when no usize holds it."""
