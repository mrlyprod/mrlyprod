from typing import Any, Literal

from numpy.typing import NDArray

ANTIS_2D: list[Literal["Carpet", "Net", "Htree", "Vtree", "Void", "Xtree", "Ytree", "Ztree", "Point", "Dust", "Hline", "Vline", "Star", "Xline", "Yline", "Zline"]]
ANTIS_3D: list[Literal["Carpet", "Net", "Htree", "Vtree", "Void", "Xtree", "Ytree", "Ztree", "Point", "Dust", "Hline", "Vline", "Star", "Xline", "Yline", "Zline"]]
CLASSICS_2D: list[Literal["Carpet", "Net", "Htree", "Vtree", "Void", "Xtree", "Ytree", "Ztree", "Point", "Dust", "Hline", "Vline", "Star", "Xline", "Yline", "Zline"]]
CLASSICS_3D: list[Literal["Carpet", "Net", "Htree", "Vtree", "Void", "Xtree", "Ytree", "Ztree", "Point", "Dust", "Hline", "Vline", "Star", "Xline", "Yline", "Zline"]]

class Design:
    """The named designs a source can point at: the four classics and their four antis."""
    @staticmethod
    def all() -> list[Literal["Carpet", "Net", "Htree", "Vtree", "Void", "Xtree", "Ytree", "Ztree", "Point", "Dust", "Hline", "Vline", "Star", "Xline", "Yline", "Zline"]]:
        """Returns every Design in canonical order."""

def antis(dimension: int) -> list[Literal["Carpet", "Net", "Htree", "Vtree", "Void", "Xtree", "Ytree", "Ztree", "Point", "Dust", "Hline", "Vline", "Star", "Xline", "Yline", "Zline"]]:
    """Returns the anti designs for a dimension."""

def classics(dimension: int) -> list[Literal["Carpet", "Net", "Htree", "Vtree", "Void", "Xtree", "Ytree", "Ztree", "Point", "Dust", "Hline", "Vline", "Star", "Xline", "Yline", "Zline"]]:
    """Returns the classic designs for a dimension."""
