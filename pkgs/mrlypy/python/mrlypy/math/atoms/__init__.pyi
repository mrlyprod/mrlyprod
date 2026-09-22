from typing import Any, Literal

from numpy.typing import NDArray
import mrlypy.core

def carpet_2d(n: int) -> NDArray[Any]:
    """Builds an n by n carpet, on where at most one coordinate is odd."""

def carpet_3d(n: int) -> NDArray[Any]:
    """Builds an n by n by n carpet, on where at most one coordinate is odd."""

def carpet_nd(n: int, rank: int) -> NDArray[Any]:
    """Builds a carpet of the given side at any rank, on where at most one coordinate is odd."""

def dust_2d(n: int) -> NDArray[Any]:
    """Builds an n by n dust, on where both coordinates are even."""

def dust_3d(n: int) -> NDArray[Any]:
    """Builds an n by n by n dust, on where all three coordinates are even."""

def dust_nd(n: int, rank: int) -> NDArray[Any]:
    """Builds a dust of the given side at any rank, on where every coordinate is even."""

def hline_2d(n: int) -> NDArray[Any]:
    """Builds an n by n line, free on axis 1, on along the odd rows."""

def htree_2d(n: int) -> NDArray[Any]:
    """Builds an n by n tree, free on axis 1, on along the even rows."""

def line_nd(n: int, rank: int, axis: int) -> NDArray[Any]:
    """Builds a line of the given side at any rank, odd on every axis but the free one; an axis past the rank frees none."""

def net_2d(n: int) -> NDArray[Any]:
    """Builds an n by n net, on where at least one coordinate is odd."""

def net_3d(n: int) -> NDArray[Any]:
    """Builds an n by n by n net, on where at least two coordinates are odd."""

def net_nd(n: int, rank: int) -> NDArray[Any]:
    """Builds a net of the given side at any rank, on where the odd coordinates plus one reach the rank."""

def noise_2d(n: int, density: float, rng: mrlypy.core.Rng) -> NDArray[Any]:
    """Builds an n by n tensor where each cell turns on with probability density, drawn from the stream."""

def noise_3d(n: int, density: float, rng: mrlypy.core.Rng) -> NDArray[Any]:
    """Builds an n by n by n tensor where each cell turns on with probability density, drawn from the stream."""

def ones_2d(n: int) -> NDArray[Any]:
    """Builds an n by n tensor of ones."""

def ones_3d(n: int) -> NDArray[Any]:
    """Builds an n by n by n tensor of ones."""

def point_2d(n: int) -> NDArray[Any]:
    """Builds an n by n point, on where both coordinates are odd."""

def point_3d(n: int) -> NDArray[Any]:
    """Builds an n by n by n point, on where all three coordinates are odd."""

def point_nd(n: int, rank: int) -> NDArray[Any]:
    """Builds a point of the given side at any rank, on where every coordinate is odd."""

def star_2d(n: int) -> NDArray[Any]:
    """Builds an n by n star, on where exactly one coordinate is odd."""

def star_3d(n: int) -> NDArray[Any]:
    """Builds an n by n by n star, on where exactly one coordinate is odd."""

def star_nd(n: int, rank: int) -> NDArray[Any]:
    """Builds a star of the given side at any rank, on where exactly one coordinate is odd."""

def tree_nd(n: int, rank: int, axis: int) -> NDArray[Any]:
    """Builds a tree of the given side at any rank, even on every axis but the free one; an axis past the rank frees none."""

def vline_2d(n: int) -> NDArray[Any]:
    """Builds an n by n line, free on axis 0, on along the odd columns."""

def void_2d(n: int) -> NDArray[Any]:
    """Builds an n by n void, on where both coordinates share one parity."""

def void_3d(n: int) -> NDArray[Any]:
    """Builds an n by n by n void, on where all three coordinates share one parity."""

def void_nd(n: int, rank: int) -> NDArray[Any]:
    """Builds a void of the given side at any rank, on where every coordinate shares one parity."""

def vtree_2d(n: int) -> NDArray[Any]:
    """Builds an n by n tree, free on axis 0, on along the even columns."""

def xline_3d(n: int) -> NDArray[Any]:
    """Builds an n by n by n line, free on axis 0, its rods running along x."""

def xtree_3d(n: int) -> NDArray[Any]:
    """Builds an n by n by n tree, free on axis 0, its beams running along x."""

def yline_3d(n: int) -> NDArray[Any]:
    """Builds an n by n by n line, free on axis 1, its rods running along y."""

def ytree_3d(n: int) -> NDArray[Any]:
    """Builds an n by n by n tree, free on axis 1, its beams running along y."""

def zeros_2d(n: int) -> NDArray[Any]:
    """Builds an n by n tensor of zeros."""

def zeros_3d(n: int) -> NDArray[Any]:
    """Builds an n by n by n tensor of zeros."""

def zline_3d(n: int) -> NDArray[Any]:
    """Builds an n by n by n line, free on axis 2, its rods running along z."""

def ztree_3d(n: int) -> NDArray[Any]:
    """Builds an n by n by n tree, free on axis 2, its beams running along z."""
