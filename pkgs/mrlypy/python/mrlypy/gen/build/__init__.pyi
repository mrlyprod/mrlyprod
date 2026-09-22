from typing import Any, Literal

from numpy.typing import NDArray
import mrlypy.core
import mrlypy.gen

def build_2d(tile: mrlypy.gen.Tile) -> dict[str, Any]:
    """Builds the flat cell the tile describes."""

def build_3d(tile: mrlypy.gen.Tile) -> dict[str, Any]:
    """Builds the cube the tile describes."""

def build_6d(hex: dict[str, Any]) -> dict[str, Any]:
    """Builds the tile's cube and flattens it through its projection."""

def create_2d(config: dict[str, Any], rng: mrlypy.core.Rng) -> mrlypy.gen.Tile:
    """Draws a random flat tile from the stream, rotations from the four quarter-turns."""

def create_3d(config: dict[str, Any], rng: mrlypy.core.Rng) -> mrlypy.gen.Tile:
    """Draws a cube tile from the config with cube orientations drawn from the stream."""

def create_6d(config: dict[str, Any], rng: mrlypy.core.Rng) -> dict[str, Any]:
    """Draws a cube tile from the config under a projection drawn from the stream."""

def random_tile_2d(max_size: int, rng: mrlypy.core.Rng) -> mrlypy.gen.Tile:
    """Draws a random flat tile up to the given size under the default config."""

def random_tile_3d(max_size: int, rng: mrlypy.core.Rng) -> mrlypy.gen.Tile:
    """Draws a random cube tile up to the given size."""

def random_tile_6d(max_size: int, rng: mrlypy.core.Rng) -> dict[str, Any]:
    """Draws a random cube tile up to the given size under a random projection."""
