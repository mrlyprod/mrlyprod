from typing import Any, Literal

from numpy.typing import NDArray
import mrlypy.core

class Paint:
    """A complete coloring recipe for one cell."""
    def __init__(self, edition: Literal["Simple", "Index", "Layers", "Neighbors", "Rows", "Columns", "Random"]) -> None: ...
    @property
    def edition(self) -> Literal["Simple", "Index", "Layers", "Neighbors", "Rows", "Columns", "Random"]:
        """The coloring edition."""
    @property
    def scheme(self) -> Literal["Multicolor", "Multitone"]:
        """The secondary color scheme."""
    @property
    def target(self) -> Literal["Fill", "Void"]:
        """The side the primary ink lands on."""
    @property
    def primary(self) -> Literal["Black", "White", "Red", "Orange", "Yellow", "Green", "Mint", "Teal", "Cyan", "Blue", "Indigo", "Purple", "Pink", "Brown", "Gray"]:
        """The primary ink."""
    @property
    def secondary(self) -> list[Literal["Black", "White", "Red", "Orange", "Yellow", "Green", "Mint", "Teal", "Cyan", "Blue", "Indigo", "Purple", "Pink", "Brown", "Gray"]]:
        """The secondary inks."""
    @property
    def shades(self) -> list[int]:
        """The shade indices of a multitone ramp."""
    def is_simple(self) -> bool:
        """Returns true for the Simple edition."""
    @staticmethod
    def new(edition: Literal["Simple", "Index", "Layers", "Neighbors", "Rows", "Columns", "Random"]) -> Paint:
        """Builds a black-primary, fill-target, multicolor paint for an edition."""
    @staticmethod
    def from_dict(data: Any) -> Paint:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

class Edition:
    """The seven ways a paint distributes its colors over a cell."""
    @staticmethod
    def all() -> list[Literal["Simple", "Index", "Layers", "Neighbors", "Rows", "Columns", "Random"]]:
        """Returns every Edition in canonical order."""
    @staticmethod
    def mode(edition: Literal["Simple", "Index", "Layers", "Neighbors", "Rows", "Columns", "Random"]) -> Literal["Type", "Tag", "Index", "Enumerate", "Random", "Row", "Column", "Depth"] | None:
        """Returns the cell-painting mode this edition renders with, or None for Random, which scatters."""

class Ink:
    """The fifteen named inks a paint draws from."""
    @staticmethod
    def all() -> list[Literal["Black", "White", "Red", "Orange", "Yellow", "Green", "Mint", "Teal", "Cyan", "Blue", "Indigo", "Purple", "Pink", "Brown", "Gray"]]:
        """Returns every Ink in canonical order."""
    @staticmethod
    def color(ink: Literal["Black", "White", "Red", "Orange", "Yellow", "Green", "Mint", "Teal", "Cyan", "Blue", "Indigo", "Purple", "Pink", "Brown", "Gray"]) -> tuple[int, int, int, int]:
        """Returns the ink's color."""

class Scheme:
    """The two ways secondary colors are drawn."""
    @staticmethod
    def all() -> list[Literal["Multicolor", "Multitone"]]:
        """Returns every Scheme in canonical order."""

class Target:
    """The side of the figure the primary ink lands on."""
    @staticmethod
    def all() -> list[Literal["Fill", "Void"]]:
        """Returns every Target in canonical order."""

def apply(paint: Paint, cell: dict[str, Any], rng: mrlypy.core.Rng) -> None:
    """Colors the cell from the paint's inks under its edition mode, scattering the Random edition from the stream."""

def coat(cell: dict[str, Any], paint: Paint, mask: NDArray[Any] | None, rng: mrlypy.core.Rng) -> None:
    """Replays a stored paint onto a cell, tagging first and applying it from the stream."""

def paint(cell: dict[str, Any], config: dict[str, Any], mask: NDArray[Any] | None, rng: mrlypy.core.Rng) -> Paint:
    """Draws a random paint under the config, applies it to the cell, and returns the recipe."""

def prime(paint: Paint, cell: dict[str, Any], mask: NDArray[Any] | None, rng: mrlypy.core.Rng) -> Paint:
    """Tags the cell for Layers and Neighbors paints and sizes the palette to the tag count."""

def random_edition(editions: list[Literal["Simple", "Index", "Layers", "Neighbors", "Rows", "Columns", "Random"]] | None, rng: mrlypy.core.Rng) -> Literal["Simple", "Index", "Layers", "Neighbors", "Rows", "Columns", "Random"]:
    """Draws a random edition from the allowed list, or from all seven."""

def reroll(paint: Paint, rng: mrlypy.core.Rng) -> Paint:
    """Redraws the paint's secondary inks and shades under its scheme."""

def setup(paint: Paint, config: dict[str, Any], rng: mrlypy.core.Rng) -> Paint:
    """Draws the paint's scheme, target and primary under the config, then rerolls the rest."""

def tag(cell: dict[str, Any], edition: Literal["Simple", "Index", "Layers", "Neighbors", "Rows", "Columns", "Random"], target: Literal["Fill", "Void"], mask: NDArray[Any] | None = None) -> int:
    """Tags the cell for the Layers and Neighbors editions and returns the distinct tag count on the secondary side."""
