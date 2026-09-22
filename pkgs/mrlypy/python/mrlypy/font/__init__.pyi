from typing import Any, Literal

from numpy.typing import NDArray
from . import paths, pens

FPS: int
HOLD: int

class Glyph:
    """One character's pixel bitmap."""
    def __init__(self, char: str, rows: list[str]) -> None: ...
    @property
    def char(self) -> str:
        """The character the glyph draws."""
    @char.setter
    def char(self, value: str) -> None: ...
    @property
    def rows(self) -> list[str]:
        """The bitmap rows of '0' and '1' characters."""
    @rows.setter
    def rows(self, value: list[str]) -> None: ...
    def height(self) -> int:
        """Returns the number of rows."""
    @staticmethod
    def new(char: str, rows: list[str]) -> Glyph:
        """Builds a glyph from its character and rows."""
    def width(self) -> int:
        """Returns the cell width of the first row, or 0 for an empty glyph."""
    @staticmethod
    def from_dict(data: Any) -> Glyph:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

def all() -> list[Glyph]:
    """Builds every glyph in font order: uppers, lowers, digits, extras, specials."""

def animate(text: str, pad: int) -> dict[str, Any]:
    """Writes the text in stroke order, one cell per frame, from an empty padded board to the full raster."""

def cycle(write: dict[str, Any], merge: list[list[int]], hold: int) -> dict[str, Any]:
    """Chains the write, the merge and their reversals into one loop, resting hold frames after each movement that has any."""

def digits() -> list[Glyph]:
    """Builds the ten digit glyphs."""

def draft(rows: list[str]) -> list[list[tuple[int, int]]]:
    """Drafts a stroke order for a trimmed bitmap by walking its lit cells: start at a lowest-left free end, keep heading, lift when stuck."""

def extras() -> list[Glyph]:
    """Builds the punctuation, symbol and arrow glyphs."""

def floor(rows: list[str]) -> int:
    """Returns the least strokes that can write a trimmed bitmap: the minimum cover of its lit cells by 4-adjacent paths, zero for a blank."""

def glyph(c: str) -> Glyph | None:
    """Returns an owned copy of the character's glyph, or None outside the font."""

def lower(rows: list[str]) -> list[str]:
    """Blanks the four corner cells of an uppercase bitmap into its rounded lowercase form."""

def lowers() -> list[Glyph]:
    """Builds the twenty-six lowercase glyphs by rounding the uppers' corners."""

def map() -> dict[str, list[str]]:
    """Returns the whole font as a map from character to bitmap rows."""

def merge(text: str, pad: int) -> list[list[int]]:
    """Folds the written text's glyphs, frame by frame, into one centered stack."""

def name_of(c: str) -> str:
    """Returns the character's Unicode name, or a U+ code point label for a character outside the font."""

def path(c: str) -> list[tuple[int, int]]:
    """Flattens the character's strokes into one cell-by-cell drawing order."""

def raster(text: str) -> list[bytes]:
    """Returns the text as a 0/1 grid, its trimmed glyphs one blank column apart."""

def specials() -> list[Glyph]:
    """Builds the four seven-row glyphs: dollar, at, copyright and registered."""

def strokes(c: str) -> list[list[tuple[int, int]]]:
    """Returns the character's ordered strokes over its trimmed bitmap, or none for a character outside the font."""

def supported() -> list[str]:
    """Returns every character in the font, in font order."""

def trim(rows: list[str]) -> list[str]:
    """Cuts blank edge columns from a bitmap, collapsing an all-blank one to a single '0' column; a row shorter than the cut keeps what it has."""

def uppers() -> list[Glyph]:
    """Builds the twenty-six uppercase glyphs."""
