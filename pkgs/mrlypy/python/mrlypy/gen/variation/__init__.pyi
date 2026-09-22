from typing import Any, Literal

from numpy.typing import NDArray
import mrlypy.core
import mrlypy.core.paint
import mrlypy.gen

class Variation:
    """One seeded artwork, from tile recipe to rendered files."""
    @property
    def key(self) -> str:
        """The random hex identifier."""
    @property
    def seed(self) -> int:
        """The seed the variation is drawn under."""
    @property
    def edition(self) -> Literal["Simple", "Index", "Layers", "Neighbors", "Rows", "Columns", "Random"]:
        """The paint edition."""
    @property
    def primaries(self) -> list[Literal["Black", "White", "Red", "Orange", "Yellow", "Green", "Mint", "Teal", "Cyan", "Blue", "Indigo", "Purple", "Pink", "Brown", "Gray"]] | None:
        """The primary inks, when the config fixes them."""
    @property
    def tile(self) -> mrlypy.gen.Tile:
        """The tile recipe."""
    @property
    def mask(self) -> mrlypy.gen.Tile | None:
        """The mask tile, present only under the Neighbors edition."""
    @property
    def paint(self) -> mrlypy.core.paint.Paint | None:
        """The paint, set by generate."""
    @property
    def base(self) -> dict[str, Any] | None:
        """The built base cell, set by generate and left out of the json."""
    @property
    def files(self) -> list[dict[str, Any]]:
        """The renderings, filled by render."""
    def is_cover(self) -> bool:
        """Returns whether the edition paints the whole tiled canvas."""
    def is_prime(self) -> bool:
        """Returns whether the edition paints the base cell before tiling."""
    @staticmethod
    def from_dict(data: Any) -> Variation:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

class File:
    """One rendering of an artwork, sized in tile repetitions."""
    @staticmethod
    def new(width: int, height: int) -> dict[str, Any]:
        """Builds a file of the given repetition counts with no PNG bytes."""

def create(config: dict[str, Any], rng: mrlypy.core.Rng) -> Variation:
    """Draws a variation's seed from the stream, then the variation itself on that seed, with a
    mask when the edition is Neighbors."""

def generate(variation: Variation, config: dict[str, Any], rng: mrlypy.core.Rng) -> Variation:
    """Builds the variation's base cell and draws its paint from the stream, painting the base
    under a prime edition."""

def render(variation: Variation, scale: int, rng: mrlypy.core.Rng) -> Variation:
    """Renders every file of the variation to PNG at the given scale, scattering a Random edition
    from the stream."""
