from typing import Any, Literal

from numpy.typing import NDArray

def ladder(cap: int) -> list[dict[str, Any]]:
    """Lists every distinct plate figure with `0 <= n <= m <= cap` but `(0, 0)`, both signs when `n < m` and the sign 1 when `n == m`, by pitch, then `n`, then the sign 1 before -1."""

def lines(field: list[float], side: int) -> list[float]:
    """Traces the zero set of a plate raster by marching squares on the lattice of cell centres, its holes and its rim ghosted to the mean of their plate neighbours so the lines run to every free edge: segments `x0, y0, x1, y1` in `[0, 1]` back to back, pieces inside a NaN cell dropped, a saddle joined by the mean of its four corners."""

def plate(n: int, m: int, sign: int, size: int) -> list[float]:
    """Samples the Chladni figure `cos(n pi x) cos(m pi y) + sign cos(m pi x) cos(n pi y)` of a free square plate at the cell centres `(i + 1/2) / size` of a row-major `size` by `size` raster, `x` along a row, scaled so the largest magnitude is 1, or all zero when no centre rises past `1e-12`; when `n == m` it is the single product for either sign."""

def shake(field: list[float], side: int, grains: list[float], from_: int, steps: int, seed: int) -> list[float]:
    """Shakes sand on a plate raster: grains are `x, y` pairs in `[0, 1)` over the row-major `side` by `side` field, a NaN pair an empty slot that stays NaN, a NaN cell a hole. Each step moves a grain a random length `kick |f| + floor` in a random direction plus a drift `-0.3 f grad f / max|grad f|^2`, with `kick = 0.6 / max|grad f|` read off the raster by finite differences and a fixed kick on a flat field, the field read bilinearly with its holes and its rim ghosted to the mean of their plate neighbours. The rim reflects; a grain on a hole falls through and becomes NaN. Step `from + i` of grain `j` draws from a counter keyed by `(seed, from + i, j)`, so six steps are three and three."""
