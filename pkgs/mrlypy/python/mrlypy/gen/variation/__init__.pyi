from typing import Any, Literal

from numpy.typing import NDArray
import mrlypy.core
import mrlypy.core.paint
import mrlypy.gen

class File:
    """One rendering of an artwork, sized in tile repetitions."""
    def __init__(self, width: int, height: int) -> None: ...
    @property
    def width(self) -> int:
        """The count of tile repetitions across."""
    @width.setter
    def width(self, value: int) -> None: ...
    @property
    def height(self) -> int:
        """The count of tile repetitions down."""
    @height.setter
    def height(self, value: int) -> None: ...
    @property
    def png(self) -> bytes:
        """The encoded PNG bytes, empty until rendered and left out of the json."""
    @png.setter
    def png(self, value: bytes) -> None: ...
    @staticmethod
    def new(width: int, height: int) -> File:
        """Builds a file of the given repetition counts with no PNG bytes."""
    @staticmethod
    def from_dict(data: Any) -> File:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

class Variation:
    """One seeded artwork, from tile recipe to rendered files."""
    @property
    def key(self) -> str:
        """The random hex identifier."""
    @key.setter
    def key(self, value: str) -> None: ...
    @property
    def seed(self) -> int:
        """The seed the variation is drawn under."""
    @seed.setter
    def seed(self, value: int) -> None: ...
    @property
    def edition(self) -> Literal["Simple", "Index", "Layers", "Neighbors", "Rows", "Columns", "Random"]:
        """The paint edition."""
    @edition.setter
    def edition(self, value: Literal["Simple", "Index", "Layers", "Neighbors", "Rows", "Columns", "Random"]) -> None: ...
    @property
    def primaries(self) -> list[Literal["Black", "White", "Red", "Orange", "Yellow", "Green", "Mint", "Teal", "Cyan", "Blue", "Indigo", "Purple", "Pink", "Brown", "Gray"]] | None:
        """The primary inks, when the config fixes them."""
    @primaries.setter
    def primaries(self, value: list[Literal["Black", "White", "Red", "Orange", "Yellow", "Green", "Mint", "Teal", "Cyan", "Blue", "Indigo", "Purple", "Pink", "Brown", "Gray"]] | None) -> None: ...
    @property
    def tile(self) -> mrlypy.gen.Tile:
        """The tile recipe."""
    @tile.setter
    def tile(self, value: mrlypy.gen.Tile) -> None: ...
    @property
    def mask(self) -> mrlypy.gen.Tile | None:
        """The mask tile, present only under the Neighbors edition."""
    @mask.setter
    def mask(self, value: mrlypy.gen.Tile | None) -> None: ...
    @property
    def paint(self) -> mrlypy.core.paint.Paint | None:
        """The paint, set by generate."""
    @paint.setter
    def paint(self, value: mrlypy.core.paint.Paint | None) -> None: ...
    @property
    def base(self) -> dict[str, Any] | None:
        """The built base cell, set by generate and left out of the json."""
    @base.setter
    def base(self, value: dict[str, Any] | None) -> None: ...
    @property
    def files(self) -> list[File]:
        """The renderings, filled by render."""
    @files.setter
    def files(self, value: list[File]) -> None: ...
    def is_cover(self) -> bool:
        """Returns whether the edition paints the whole tiled canvas."""
    def is_prime(self) -> bool:
        """Returns whether the edition paints the base cell before tiling."""
    @staticmethod
    def from_dict(data: Any) -> Variation:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

class Config:
    """The settings an artwork is drawn under."""
    @staticmethod
    def default() -> dict[str, Any]:
        """Returns the default Config."""

def create(config: dict[str, Any], rng: mrlypy.core.Rng) -> Variation:
    """Draws a variation's seed from the stream, then the variation itself on that seed, with a
    mask when the edition is Neighbors."""

def generate(variation: Variation, config: dict[str, Any], rng: mrlypy.core.Rng) -> Variation:
    """Builds the variation's base cell and draws its paint from the stream, painting the base
    under a prime edition."""

def render(variation: Variation, scale: int, rng: mrlypy.core.Rng) -> Variation:
    """Renders every file of the variation to PNG at the given scale, scattering a Random edition
    from the stream."""
