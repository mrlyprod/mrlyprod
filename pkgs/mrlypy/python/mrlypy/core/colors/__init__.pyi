from typing import Any, Literal

from numpy.typing import NDArray
import mrlypy.core

ALPHA: tuple[int, int, int, int]
BLACK: tuple[int, int, int, int]
BLUE: tuple[int, int, int, int]
BROWN: tuple[int, int, int, int]
CYAN: tuple[int, int, int, int]
DARK: Theme
GRAY: tuple[int, int, int, int]
GREEN: tuple[int, int, int, int]
INDIGO: tuple[int, int, int, int]
LIGHT: Theme
MINT: tuple[int, int, int, int]
NAMES: list[str]
ORANGE: tuple[int, int, int, int]
PALETTE: list[tuple[int, int, int, int]]
PINK: tuple[int, int, int, int]
PURPLE: tuple[int, int, int, int]
RED: tuple[int, int, int, int]
TEAL: tuple[int, int, int, int]
WHITE: tuple[int, int, int, int]
YELLOW: tuple[int, int, int, int]

class Theme:
    """One theme: the surfaces of a dark or a light ground and the thirteen inks, the same on both."""
    @property
    def ground(self) -> tuple[int, int, int, int]:
        """The ground every figure is painted on."""
    @ground.setter
    def ground(self, value: tuple[int, int, int, int]) -> None: ...
    @property
    def bg(self) -> tuple[int, int, int, int]:
        """The page background, one step off the ground."""
    @bg.setter
    def bg(self, value: tuple[int, int, int, int]) -> None: ...
    @property
    def panel(self) -> tuple[int, int, int, int]:
        """The raised panel."""
    @panel.setter
    def panel(self, value: tuple[int, int, int, int]) -> None: ...
    @property
    def deep(self) -> tuple[int, int, int, int]:
        """The sunken well."""
    @deep.setter
    def deep(self, value: tuple[int, int, int, int]) -> None: ...
    @property
    def line(self) -> tuple[int, int, int, int]:
        """The hairline between things."""
    @line.setter
    def line(self, value: tuple[int, int, int, int]) -> None: ...
    @property
    def fg(self) -> tuple[int, int, int, int]:
        """The foreground, the strongest tone."""
    @fg.setter
    def fg(self, value: tuple[int, int, int, int]) -> None: ...
    @property
    def dim(self) -> tuple[int, int, int, int]:
        """The dimmed foreground, for anything secondary."""
    @dim.setter
    def dim(self, value: tuple[int, int, int, int]) -> None: ...
    @property
    def accent(self) -> tuple[int, int, int, int]:
        """The interactive accent."""
    @accent.setter
    def accent(self, value: tuple[int, int, int, int]) -> None: ...
    @property
    def on_accent(self) -> tuple[int, int, int, int]:
        """The tone written on the accent."""
    @on_accent.setter
    def on_accent(self, value: tuple[int, int, int, int]) -> None: ...
    @property
    def red(self) -> tuple[int, int, int, int]:
        """The red ink."""
    @red.setter
    def red(self, value: tuple[int, int, int, int]) -> None: ...
    @property
    def orange(self) -> tuple[int, int, int, int]:
        """The orange ink."""
    @orange.setter
    def orange(self, value: tuple[int, int, int, int]) -> None: ...
    @property
    def yellow(self) -> tuple[int, int, int, int]:
        """The yellow ink."""
    @yellow.setter
    def yellow(self, value: tuple[int, int, int, int]) -> None: ...
    @property
    def green(self) -> tuple[int, int, int, int]:
        """The green ink."""
    @green.setter
    def green(self, value: tuple[int, int, int, int]) -> None: ...
    @property
    def mint(self) -> tuple[int, int, int, int]:
        """The mint ink."""
    @mint.setter
    def mint(self, value: tuple[int, int, int, int]) -> None: ...
    @property
    def teal(self) -> tuple[int, int, int, int]:
        """The teal ink."""
    @teal.setter
    def teal(self, value: tuple[int, int, int, int]) -> None: ...
    @property
    def cyan(self) -> tuple[int, int, int, int]:
        """The cyan ink."""
    @cyan.setter
    def cyan(self, value: tuple[int, int, int, int]) -> None: ...
    @property
    def blue(self) -> tuple[int, int, int, int]:
        """The blue ink."""
    @blue.setter
    def blue(self, value: tuple[int, int, int, int]) -> None: ...
    @property
    def indigo(self) -> tuple[int, int, int, int]:
        """The indigo ink."""
    @indigo.setter
    def indigo(self, value: tuple[int, int, int, int]) -> None: ...
    @property
    def purple(self) -> tuple[int, int, int, int]:
        """The purple ink."""
    @purple.setter
    def purple(self, value: tuple[int, int, int, int]) -> None: ...
    @property
    def pink(self) -> tuple[int, int, int, int]:
        """The pink ink."""
    @pink.setter
    def pink(self, value: tuple[int, int, int, int]) -> None: ...
    @property
    def brown(self) -> tuple[int, int, int, int]:
        """The brown ink."""
    @brown.setter
    def brown(self, value: tuple[int, int, int, int]) -> None: ...
    @property
    def gray(self) -> tuple[int, int, int, int]:
        """The gray ink."""
    @gray.setter
    def gray(self, value: tuple[int, int, int, int]) -> None: ...
    def hues(self) -> list[tuple[int, int, int, int]]:
        """The thirteen inks in name order."""
    def inks(self) -> list[tuple[int, int, int, int]]:
        """The six inks a figure cycles through: blue, orange, yellow, green, pink, indigo."""
    @staticmethod
    def from_dict(data: Any) -> Theme:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

def alpha(color: tuple[int, int, int, int], level: int) -> tuple[int, int, int, int]:
    """Returns the color with its alpha set to level."""

def board(dark: bool) -> tuple[int, int, int, int]:
    """Returns the ground rgba of the dark or the light theme."""

def css(color: tuple[int, int, int, int]) -> str:
    """Formats the color as a css rgb or rgba call."""

def from_hex(hex: str) -> tuple[int, int, int, int]:
    """Parses a #RRGGBB or #RRGGBBAA code, hash optional."""

def gradient(colors: list[tuple[int, int, int, int]], steps: int) -> list[tuple[int, int, int, int]]:
    """Builds a gradient of steps colors sweeping evenly through the given stops."""

def ink(dark: bool) -> tuple[int, int, int, int]:
    """Returns the foreground rgba of the dark or the light theme."""

def invert(color: tuple[int, int, int, int]) -> tuple[int, int, int, int]:
    """Returns the color with every channel flipped and the alpha kept."""

def lightness(color: tuple[int, int, int, int], level: int) -> tuple[int, int, int, int]:
    """Returns the color scaled toward black below level 50 and toward white above."""

def luma_types(pixels: NDArray[Any], width: int, height: int, level: int) -> NDArray[Any]:
    """Reads rgba pixels as a type grid, one wherever the rgb mean falls below the level."""

def mix(color_1: tuple[int, int, int, int], color_2: tuple[int, int, int, int], ratio: float) -> tuple[int, int, int, int]:
    """Blends two colors linearly by ratio."""

def named(name: str) -> tuple[int, int, int, int]:
    """Returns the palette color a name spells."""

def random(alpha: bool, rng: mrlypy.core.Rng) -> tuple[int, int, int, int]:
    """Draws a color from the stream, opaque unless alpha is asked for."""

def rgb(r: int, g: int, b: int) -> tuple[int, int, int, int]:
    """Builds an opaque color."""

def rgba(r: int, g: int, b: int, a: int) -> tuple[int, int, int, int]:
    """Builds a color with an explicit alpha."""

def shades(hue: tuple[int, int, int, int]) -> list[tuple[int, int, int, int]]:
    """Returns a hue one shade lighter, itself, and one shade darker."""

def snap(pixels: NDArray[Any], palette: list[tuple[int, int, int, int]]) -> NDArray[Any]:
    """Snaps every pixel to the palette color nearest it in squared rgba distance, first on a tie."""

def to_hex(color: tuple[int, int, int, int]) -> str:
    """Formats the color as lowercase hex, appending the alpha pair only when not opaque."""
