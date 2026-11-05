from typing import Any, Literal

from numpy.typing import NDArray
import mrly.core
import mrly.tile

class Config2d:
    @staticmethod
    def default() -> dict[str, Any]:
        ...

class Config3d:
    @staticmethod
    def default() -> dict[str, Any]:
        ...

def build_2d(tile: mrly.tile.Tile) -> dict[str, Any]:
    ...

def build_3d(tile: mrly.tile.Tile) -> dict[str, Any]:
    ...

def build_6d(hex: dict[str, Any]) -> dict[str, Any]:
    ...

def create_2d(config: dict[str, Any], rng: mrly.core.Rng) -> mrly.tile.Tile:
    ...

def create_3d(config: dict[str, Any], rng: mrly.core.Rng) -> mrly.tile.Tile:
    ...

def create_6d(config: dict[str, Any], rng: mrly.core.Rng) -> dict[str, Any]:
    ...

def random_tile_2d(max_size: int, rng: mrly.core.Rng) -> mrly.tile.Tile:
    ...

def random_tile_3d(max_size: int, rng: mrly.core.Rng) -> mrly.tile.Tile:
    ...

def random_tile_6d(max_size: int, rng: mrly.core.Rng) -> dict[str, Any]:
    ...
