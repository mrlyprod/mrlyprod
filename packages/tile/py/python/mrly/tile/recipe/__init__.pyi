from typing import Any, Literal

from numpy.typing import NDArray

CLASSICS_2D: list[Literal["Carpet", "Net", "Htree", "Vtree", "Void", "Xtree", "Ytree", "Ztree", "Point", "Dust", "Hline", "Vline", "Star", "Xline", "Yline", "Zline"]]
CLASSICS_3D: list[Literal["Carpet", "Net", "Htree", "Vtree", "Void", "Xtree", "Ytree", "Ztree", "Point", "Dust", "Hline", "Vline", "Star", "Xline", "Yline", "Zline"]]
MAX_LEVEL: int
MAX_SIDE: int
MAX_SLOTS: int
MIN_SIDE: int

class Design:
    @staticmethod
    def all() -> list[Literal["Carpet", "Net", "Htree", "Vtree", "Void", "Xtree", "Ytree", "Ztree", "Point", "Dust", "Hline", "Vline", "Star", "Xline", "Yline", "Zline"]]:
        ...

def classics(dimension: int) -> list[Literal["Carpet", "Net", "Htree", "Vtree", "Void", "Xtree", "Ytree", "Ztree", "Point", "Dust", "Hline", "Vline", "Star", "Xline", "Yline", "Zline"]]:
    ...

def generals(min_size: int, max_size: int, parity: Literal["Evens", "Odds", "Both"]) -> list[int]:
    ...

def nestings(min_size: int, max_size: int, parity: Literal["Evens", "Odds", "Both"]) -> list[list[int]]:
    ...

def powers(min_size: int, max_size: int, parity: Literal["Evens", "Odds", "Both"]) -> list[tuple[int, int]]:
    ...

def products(min_size: int, max_size: int, count: int, parity: Literal["Evens", "Odds", "Both"]) -> list[list[int]]:
    ...

def size(number: int, level: int) -> int | None:
    ...
