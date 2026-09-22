from typing import Any, Literal

from numpy.typing import NDArray

ANTIS_2D: list[Literal["Carpet", "Net", "Htree", "Vtree", "Void", "Xtree", "Ytree", "Ztree", "Point", "Dust", "Hline", "Vline", "Star", "Xline", "Yline", "Zline"]]
ANTIS_3D: list[Literal["Carpet", "Net", "Htree", "Vtree", "Void", "Xtree", "Ytree", "Ztree", "Point", "Dust", "Hline", "Vline", "Star", "Xline", "Yline", "Zline"]]

def antis(dimension: int) -> list[Literal["Carpet", "Net", "Htree", "Vtree", "Void", "Xtree", "Ytree", "Ztree", "Point", "Dust", "Hline", "Vline", "Star", "Xline", "Yline", "Zline"]]:
    """Returns the anti designs for a dimension."""
