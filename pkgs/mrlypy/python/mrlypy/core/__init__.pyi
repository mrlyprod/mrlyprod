from typing import Any, Literal

from numpy.typing import NDArray
from . import cell, codec, colors, error, image, paint, ramp, rng, tensor

HEX_RATIO: float
PNG_MAGIC: bytes

class Rng:
    """The seeded random stream, one class, passed wherever Rust takes a mutable stream."""
    def __init__(self, seed: int) -> None:
        """Builds the stream from a seed."""
    def below(self, n: int) -> int:
        """Draws an integer below n, or zero when n is zero."""
    def boolean(self) -> bool:
        """Draws a fair coin flip."""
    def chance(self, p: float) -> bool:
        """Returns true with probability p."""
    def range(self, lo: int, hi: int) -> int:
        """Draws an integer between lo and hi inclusive, or lo when hi is not above lo."""
    def sample_indices(self, length: int, amount: int) -> list[int]:
        """Draws amount distinct indices below length, or every index when amount is larger."""
    def unit(self) -> float:
        """Draws a float at or above zero and below one."""

class Image:
    """A paletted image: rows of palette indices and the palette they point into, hex strings in json."""
    def __init__(self, width: int, height: int, rows: list[list[int]], palette: list[tuple[int, int, int, int]]) -> None: ...
    @property
    def width(self) -> int:
        """The width in pixels."""
    @property
    def height(self) -> int:
        """The height in pixels."""
    @property
    def rows(self) -> list[list[int]]:
        """The palette index of every pixel, row by row."""
    @property
    def palette(self) -> list[tuple[int, int, int, int]]:
        """The colors the rows index."""
    def colors(self) -> NDArray[Any]:
        """Returns the flat rgba pixels, transparent wherever an index misses the palette."""
    @staticmethod
    def from_pixels(width: int, height: int, pixels: NDArray[Any]) -> Image:
        """Builds a paletted image from raw rgba pixels, growing the palette as new colors appear."""
    @staticmethod
    def new(width: int, height: int, rows: list[list[int]], palette: list[tuple[int, int, int, int]]) -> Image:
        """Builds an image from its four parts."""
    def png(self, scale: int) -> bytes:
        """Encodes the image as a png at the given scale."""
    def resample(self, width: int, height: int, filter: Literal["Nearest", "Linear", "Box"]) -> Image:
        """Resamples the image to a new size, its palette rebuilt from the blended pixels."""
    @staticmethod
    def from_dict(data: Any) -> Image:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

class Colorizer:
    """A rule that turns counter values into colors."""
    @staticmethod
    def diverge() -> Any:
        """Builds the blue-to-red diverging ramp around a white middle."""
    @staticmethod
    def fire() -> Any:
        """Builds the black-through-ember fire ramp: black, dark red, orange, light yellow."""
    @staticmethod
    def gradient_bins(background: tuple[int, int, int, int], colors: list[tuple[int, int, int, int]], shades: int) -> Any:
        """Builds a binned colorizer from a gradient through the given stops."""
    @staticmethod
    def heat() -> Any:
        """Builds the white-to-black heat ramp."""

class Dtype:
    """The element widths a tensor can hold."""
    @staticmethod
    def max(dtype: Literal["U8", "U16", "U32", "I32"]) -> int:
        """Returns the largest value the width can hold."""

def hex_fit(pixels: NDArray[Any], width: int, height: int, vertical: bool, filter: Literal["Nearest", "Linear", "Box"]) -> tuple[int, int, NDArray[Any]]:
    """Squashes rgba pixels to the hex aspect, returning the new width, height and pixels."""

def hex_size(width: int, height: int, vertical: bool) -> tuple[int, int]:
    """Returns the size a hex rendering wears, the named axis squashed by the triangle ratio."""

def resample(pixels: NDArray[Any], width: int, height: int, out_w: int, out_h: int, filter: Literal["Nearest", "Linear", "Box"]) -> NDArray[Any]:
    """Resamples rgba pixels to a new size."""

def unpng(bytes: bytes) -> tuple[int, int, NDArray[Any]]:
    """Decodes a png to its width, height, and rgba colors."""
