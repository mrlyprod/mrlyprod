from typing import Any, Literal

from numpy.typing import NDArray

LAPS_CAP: int
PENCIL_CAP: int
POINT_CAP: int
RADIUS_CAP: int
RASTER_CAP: int
SIDES: tuple[int, int]

def cell(width: int, height: int, reach: float) -> float:
    """The side of one cell in wheel radii at a reach, the number the page needs to draw the tile on the wheel."""

def cover(track: dict[str, Any], pencils: list[dict[str, Any]], exact: bool, samples: int, side: int) -> dict[str, Any]:
    """The shape between the walls of a circle roulette, on a raster of `side` by `side` pixels over the disc, row zero at the top and the ordinate falling down the rows. Every distinct curve under the coincidence law is drawn once as a polyline of at least `samples` points, and of enough points that consecutive points land in one pixel or in two of the eight that touch, so the polylines make a wall no four-connected flood crosses. One flood starts from every pixel of the raster's edge, the fluid poured from outside; one starts from the centre pixel, the fluid poured at the centre, and is empty when the centre is a wall or the outside already reached it; the shape is the rest of the disc, pockets included. `covered` is the shape's share of the disc's pixels, the wall's own pixels counted in and reported apart as `wall`, and `hole` is the centre flood's share. `winding` is the mean signed winding number of the disc's pixel centres, read off crossings of the same polylines by scanline and never off a flood, and `areas` is the closed form it converges to, the distinct curves' `signed_area` summed over the disc's area: the pair checks the polylines and the raster against Green's theorem and never the floods, which are guarded instead by the sample spacing of at most half a pixel, which makes the wall eight-connected and a four-connected flood unable to cross it. Every share carries a boundary error of the order of the polylines' length times the pixel side over the disc's area."""

def disc(track: dict[str, Any], pencils: list[dict[str, Any]]) -> dict[str, Any]:
    """The disc a circle roulette sits in: the wheel's centre turns on a circle of radius `rho`, and a seat `d` from the wheel's centre puts the pencil at `|z|^2 = rho^2 + d^2 + 2 rho d cos(a t / b -+ arg p)`, whose phase runs over `a` full turns, so that curve lies in the closed annulus from `abs(rho - d)` to `rho + d` and attains both bounds. The whole roulette therefore never leaves the disc of radius `rho + max d` and enters no disc of radius under `min abs(rho - d)`, the least over the seats and not the outermost seat's own, since seats on both sides of `rho` each keep their own inner radius. Refuses a line or a polygon track, whose roulette need not close and has no wall."""

def distinct(track: dict[str, Any], pencils: list[dict[str, Any]], exact: bool) -> int:
    """How many classes `representatives` finds: the distinct curves on a circle track, the shapes up to a shift along a line track, one class per pencil on a polygon and under jitter."""

def frame(track: dict[str, Any], pencils: list[dict[str, Any]]) -> list[float]:
    """The box the whole picture sits in: the centre path and the track, padded by the wheel's radius or the farthest seat, whichever reaches further."""

def nodes(track: dict[str, Any], pencils: list[dict[str, Any]], exact: bool) -> int | None:
    """The crossings of the whole roulette on a circle track, the generic count, with `R/r = a/b` in lowest terms. Write `|p|` for a seat's distance from the wheel's centre in wheel radii and `A` for the centre path's radius in the same units, `(a - b)/b` inside and `(a + b)/b` outside. Every seat must lie strictly inside the window `0 < |p| < min(1, A)`, which three hypotheses cut: `|p| > 0`, since a seat at the wheel's centre draws the centre circle `b` times over and never crosses; `|p| < 1`, the loop threshold, past which a curve loops; and `|p| < A`, the seat threshold, where the seat reaches the centre path, which comes before the loop threshold on every inside track with `a < 2b` and never bites outside. Inside that window two distinct curves cross exactly `2ab` times, one curve crosses itself `a(b - 1)` times, and `k` distinct curves cross `2ab k(k - 1) / 2 + k a (b - 1)` times, the design entering only through `k`. `exact` reads the coincidence law on the seats, as `distinct` does. `None` on a line or a polygon track, and `None` when any seat leaves the window, where neither count is the law's. At isolated reaches some crossings merge, so the count holds for the generic reach."""

def pencils(types: bytes, width: int, height: int, mode: str, reach: float, jitter: float, seed: int) -> list[dict[str, Any]]:
    """Seats one pencil per chosen site of a byte grid: `fill` the filled cells, `void` the empty ones, `both`, or `corners` the corners of the filled cells, each once. The tile is scaled so its circumradius is `reach` wheel radii, and `jitter` moves every seat by up to that fraction of a cell each way, seeded."""

def point(track: dict[str, Any], pencil: dict[str, Any], s: float) -> tuple[float, float]:
    """Where a pencil is after `s` of path length: the centre plus the seat turned with the wheel."""

def pose(track: dict[str, Any], s: float) -> tuple[float, float]:
    """The wheel's centre after `s` of path length."""

def representatives(track: dict[str, Any], pencils: list[dict[str, Any]], exact: bool) -> list[int]:
    """One pencil per class, the first index of every class in the order the pencils were seated. On a circle track the classes are the distinct curves, by the coincidence law on exact seats: two pencils draw one curve iff a rotation of a full turn over the ratio's denominator carries one seat to the other, which the square lattice allows only by half turns when the denominator is even and by quarter turns when four divides it. On a line track the classes are the seat radii, and those are shapes up to a shift, not curves: turning a seat by `gamma` slides its whole ribbon `gamma` wheel radii along the line while the ribbon's period is a full turn of the wheel, so two seats of one radius draw translates of one shape and share no point unless the seats are equal. On a polygon every pencil is its own class, and so is every pencil under jitter."""

def seats(pencils: list[dict[str, Any]]) -> dict[str, Any]:
    """Counts the pencils by kind."""

def signed_area(track: dict[str, Any], pencil: dict[str, Any]) -> float | None:
    """The signed area one pencil's closed trochoid sweeps over the whole track, counterclockwise positive and counted with multiplicity, so it is the winding number integrated over the plane: `pi b rho (rho - d^2/r)` inside and `pi b rho (rho + d^2/r)` outside, with `rho` the centre circle's radius `R -+ r`, `d = r |p|` the seat's distance from the wheel's centre and `R/r = a/b` in lowest terms. Green's theorem on `z(t) = rho e^(i t) + p r e^(-+ i (rho/r) t)` gives it, and the cross terms carry `e^(-+ i a t / b)` over `b` centre turns and integrate to zero. No hypothesis on the seat: loops are counted with their sign. `None` off a circle track, where the roulette need not close."""

def trace(track: dict[str, Any], pencils: list[dict[str, Any]], samples: int) -> list[float]:
    """Traces every pencil along the whole track at `samples` evenly spaced path lengths, first and last included: pencil by pencil, sample by sample, x then y."""

def track(kind: str, ring: int, wheel: int, sides: int, laps: int) -> dict[str, Any]:
    """Lays a track: `line` a straight line under the wheel for `laps` turns; `in` and `out` a circle of radius `ring` with the wheel inside or outside, closing after the reduced denominator of `ring` over `wheel` orbits; `polyin` and `polyout` a regular polygon of `sides` sides and circumradius `ring` for `laps` laps."""

def turn(track: dict[str, Any], s: float) -> float:
    """The wheel's turn after `s` of path length, in radians: `side` times `s` over the wheel's radius."""
