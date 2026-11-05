from typing import Any, Literal

from numpy.typing import NDArray

LAPS_CAP: int
PENCIL_CAP: int
POINT_CAP: int
RADIUS_CAP: int
RASTER_CAP: int
SIDES: tuple[int, int]

class Seats:
    @staticmethod
    def default() -> dict[str, Any]:
        ...

def cell(width: int, height: int, reach: float) -> float:
    ...

def cover(track: dict[str, Any], pencils: list[dict[str, Any]], exact: bool, samples: int, side: int) -> dict[str, Any]:
    ...

def disc(track: dict[str, Any], pencils: list[dict[str, Any]]) -> dict[str, Any]:
    ...

def distinct(track: dict[str, Any], pencils: list[dict[str, Any]], exact: bool) -> int:
    ...

def frame(track: dict[str, Any], pencils: list[dict[str, Any]]) -> list[float]:
    ...

def nodes(track: dict[str, Any], pencils: list[dict[str, Any]], exact: bool) -> int | None:
    ...

def pencils(types: bytes, width: int, height: int, mode: str, reach: float, jitter: float, seed: int) -> list[dict[str, Any]]:
    ...

def point(track: dict[str, Any], pencil: dict[str, Any], s: float) -> tuple[float, float]:
    ...

def pose(track: dict[str, Any], s: float) -> tuple[float, float]:
    ...

def representatives(track: dict[str, Any], pencils: list[dict[str, Any]], exact: bool) -> list[int]:
    ...

def seats(pencils: list[dict[str, Any]]) -> dict[str, Any]:
    ...

def signed_area(track: dict[str, Any], pencil: dict[str, Any]) -> float | None:
    ...

def trace(track: dict[str, Any], pencils: list[dict[str, Any]], samples: int) -> list[float]:
    ...

def track(kind: str, ring: int, wheel: int, sides: int, laps: int) -> dict[str, Any]:
    ...

def turn(track: dict[str, Any], s: float) -> float:
    ...
