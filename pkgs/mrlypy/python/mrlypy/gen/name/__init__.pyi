from typing import Any, Literal

from numpy.typing import NDArray
import mrlypy.gen

class Tile:
    """A tile recipe folded to its one canonical object."""
    @property
    def code(self) -> int | None:
        """The one design of a flat or fractal tile."""
    @property
    def special(self) -> int | None:
        """The mask code of a special tile."""
    @property
    def magic(self) -> list[int]:
        """The letters of a magic tile, first letter outermost."""
    @property
    def mosaic(self) -> list[int]:
        """The three codes of a mosaic tile."""
    @property
    def factor(self) -> int | None:
        """The side of the mask of a special or mosaic tile."""
    @property
    def side(self) -> Any:
        """The side each slot renders at, one per letter for a magic tile."""
    @property
    def level(self) -> int | None:
        """The power a fractal tile is raised to, absent at one."""
    @property
    def turn(self) -> Any:
        """The quarter turns of each slot, absent when nothing turns."""
    @property
    def flip(self) -> bool:
        """Whether a special tile flips its mask."""
    @property
    def invert(self) -> bool:
        """Whether the finished tile inverts."""
    def checked(self) -> Tile:
        """Folds a decoded value to its canonical form, or an error for one outside the kind."""
    @staticmethod
    def from_file(text: str) -> Tile:
        """Reads a filename back into the value, or an error."""
    @staticmethod
    def from_json(text: str) -> Tile:
        """Reads a JSON object into its canonical value, or an error naming the broken key."""
    @staticmethod
    def from_url(text: str) -> Tile:
        """Reads a path and query string back into the value, or an error."""
    @staticmethod
    def of(recipe: mrlypy.gen.Tile) -> Tile:
        """Folds a recipe to its name."""
    def recipe(self) -> mrlypy.gen.Tile:
        """Builds the recipe the name folds, resized and checked."""
    def to_file(self) -> str:
        """Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back."""
    def to_id(self) -> str:
        """Prints the first eight hex digits of the sha256 of the canonical JSON."""
    def to_json(self) -> str:
        """Prints the canonical JSON object."""
    def to_mrly(self) -> str:
        """Prints the kind and the keys as a line of prose for pages, or an error when the name does not read back."""
    def to_url(self) -> str:
        """Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back."""
    @staticmethod
    def from_dict(data: Any) -> Tile:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""
