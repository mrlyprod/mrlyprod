from typing import Any, Literal

from numpy.typing import NDArray

def draw(code: int, base: int, level: int) -> dict[str, Any]:
    """Draws level `level` of `bang dim 2, base b, code c` in arcs: cell `(x, y)` of the `b x b` mask is filled when bit `b y + x` of the code is set, level `level` is its Kronecker power built by [`crate::math::bang::factory::create`], and level 0 is one filled cell."""

def law(code: int, base: int, level: int) -> dict[str, Any] | None:
    """Returns the proved loop law of `bang dim 2, base b, code c` at the level, or none where no law is proved: the carpet, base 3 code 495, has `(8^n - 1)/7 - 3^n + n + 1` loops; at base 2 codes 7 and 14 have `3^(n-1) - 2^n + 1`, codes 11 and 13 have `3^(n-1) - 2^(n-1)`, both from level 1 on, code 9 has `2^n - 1`, and the other eleven codes never loop."""

def trace(side: int, on: list[bool]) -> dict[str, Any]:
    """Draws a `side x side` grid of filled and deleted cells in arcs and counts its curves by union-find over the edge midpoints, a curve being a loop when no midpoint of it lies on the boundary."""
