from typing import Any, Literal

from numpy.typing import NDArray

def at(tensor: NDArray[Any], flat: int) -> int:
    """Returns the element at a flat index, which must be below the size like a slice index."""

def binarize(tensor: NDArray[Any], threshold: int) -> NDArray[Any]:
    """Maps every element to one at or above the threshold, zero below."""

def binarize_otsu(tensor: NDArray[Any]) -> NDArray[Any]:
    """Binarizes at one above the Otsu threshold."""

def blur(tensor: NDArray[Any], mask: NDArray[Any], wrap: bool) -> NDArray[Any]:
    """Averages every position over its masked neighborhood, rounded."""

def bytes(tensor: NDArray[Any]) -> bytes:
    """Returns the elements as bytes."""

def count(tensor: NDArray[Any], value: int) -> int:
    """Counts the cells holding one value."""

def dtype(tensor: NDArray[Any]) -> Literal["U8", "U16", "U32", "I32"]:
    """Returns the element width."""

def exposed(tensor: NDArray[Any]) -> int:
    """Counts the faces where filled cells meet empty cells or the boundary: perimeter in 2d, surface in 3d."""

def filled(shape: list[int], value: int, dtype: Literal["U8", "U16", "U32", "I32"]) -> NDArray[Any]:
    """Builds a tensor of the shape and width filled with one value."""

def flip(tensor: NDArray[Any], axis: int) -> NDArray[Any]:
    """Reverses the tensor along one axis."""

def fractal(tensor: NDArray[Any], level: int) -> NDArray[Any]:
    """Folds the tensor into its level-fold Kronecker power."""

def full(shape: list[int], value: int) -> NDArray[Any]:
    """Builds a u8 tensor filled with one value."""

def get(tensor: NDArray[Any], multi: list[int]) -> int:
    """Returns the byte at a multi-index."""

def i32(data: list[int], shape: list[int]) -> NDArray[Any]:
    """Wraps an i32 vector as a tensor of the shape."""

def i32s(tensor: NDArray[Any]) -> list[int]:
    """Returns the elements as i32s."""

def index(tensor: NDArray[Any], multi: list[int]) -> int:
    """Folds a multi-index into its flat index."""

def invert(tensor: NDArray[Any]) -> NDArray[Any]:
    """Flips every element between zero and one."""

def kron(tensor: NDArray[Any], other: NDArray[Any]) -> NDArray[Any]:
    """Builds the Kronecker product of the two tensors."""

def layers(tensor: NDArray[Any], dtype: Literal["U8", "U16", "U32", "I32"]) -> NDArray[Any]:
    """Numbers every position by its concentric ring out from the center."""

def neighbors(tensor: NDArray[Any], mask: NDArray[Any], target: int, wrap: bool, dtype: Literal["U8", "U16", "U32", "I32"]) -> NDArray[Any]:
    """Counts each position's masked neighbors holding the target bit."""

def new(shape: list[int]) -> NDArray[Any]:
    """Builds a zeroed u8 tensor of the shape."""

def of(data: bytes, shape: list[int]) -> NDArray[Any]:
    """Wraps a byte vector as a tensor of the shape."""

def otsu_threshold(tensor: NDArray[Any]) -> int:
    """Returns the Otsu threshold splitting the histogram at greatest variance."""

def pad(tensor: NDArray[Any], count: int, value: int) -> NDArray[Any]:
    """Wraps the tensor in a count-thick border of one value."""

def perforate(tensor: NDArray[Any], mask: NDArray[Any], value: int) -> NDArray[Any]:
    """Stamps the value wherever the tiled mask is nonzero."""

def put(tensor: NDArray[Any], flat: int, value: int) -> None:
    """Writes the element at a flat index, which must be below the size like a slice index."""

def rot90(tensor: NDArray[Any], k: int, axes: tuple[int, int]) -> NDArray[Any]:
    """Rotates the tensor k quarter turns in the plane of two axes."""

def set(tensor: NDArray[Any], multi: list[int], value: int) -> None:
    """Writes the byte at a multi-index."""

def size(tensor: NDArray[Any]) -> int:
    """Returns the number of elements."""

def slice(tensor: NDArray[Any], axis: int, index: int) -> NDArray[Any]:
    """Drops one axis by fixing it at an index."""

def sum(tensor: NDArray[Any]) -> int:
    """Returns the sum of all elements."""

def tile(tensor: NDArray[Any], reps: list[int]) -> NDArray[Any]:
    """Repeats the tensor the given number of times along each axis."""

def transpose(tensor: NDArray[Any], a: int, b: int) -> NDArray[Any]:
    """Swaps two axes."""

def typed(shape: list[int], dtype: Literal["U8", "U16", "U32", "I32"]) -> NDArray[Any]:
    """Builds a zeroed tensor of the shape and width."""

def u16(data: list[int], shape: list[int]) -> NDArray[Any]:
    """Wraps a u16 vector as a tensor of the shape."""

def u16s(tensor: NDArray[Any]) -> list[int]:
    """Returns the elements as u16s."""

def u32(data: list[int], shape: list[int]) -> NDArray[Any]:
    """Wraps a u32 vector as a tensor of the shape."""

def u32s(tensor: NDArray[Any]) -> list[int]:
    """Returns the elements as u32s."""

def u8(data: bytes, shape: list[int]) -> NDArray[Any]:
    """Wraps a u8 vector as a tensor of the shape, the same door as [`Tensor::of`]."""
