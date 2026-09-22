from typing import Any, Literal

from numpy.typing import NDArray
from . import pairs, sample

class Field:
    """A square grid of f32 samples."""
    def __init__(self, size: int) -> None: ...
    @property
    def data(self) -> list[float]:
        """The samples in row-major order."""
    @data.setter
    def data(self, value: list[float]) -> None: ...
    @property
    def size(self) -> int:
        """The side length in samples."""
    @size.setter
    def size(self, value: int) -> None: ...
    def as_f64(self) -> list[float]:
        """Returns the samples widened to f64."""
    @staticmethod
    def from_data(data: list[float], size: int) -> Field:
        """Wraps row-major samples of the given side."""
    def max(self) -> float:
        """Returns the largest sample."""
    def mean(self) -> float:
        """Returns the mean sample, or zero for an empty field."""
    def min(self) -> float:
        """Returns the smallest sample."""
    @staticmethod
    def new(size: int) -> Field:
        """Builds a zeroed field of the given side."""
    def normalized(self, symmetric: bool) -> list[float]:
        """Returns the samples scaled into 0..1, symmetric about zero on request."""
    @staticmethod
    def from_dict(data: Any) -> Field:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

class Preset:
    """One named moire recipe: the design, the scales it stacks and the lattice it samples."""
    @property
    def name(self) -> str:
        """The name the recipe answers to."""
    @property
    def spec(self) -> dict[str, Any]:
        """The design sampled at every scale."""
    @spec.setter
    def spec(self, value: dict[str, Any]) -> None: ...
    @property
    def numbers(self) -> list[int]:
        """The side numbers stacked."""
    @numbers.setter
    def numbers(self, value: list[int]) -> None: ...
    @property
    def combine(self) -> Literal["Sum", "And", "Xor"]:
        """The way the layers merge."""
    @combine.setter
    def combine(self, value: Literal["Sum", "And", "Xor"]) -> None: ...
    @property
    def level(self) -> int:
        """The fractal depth of each layer."""
    @level.setter
    def level(self, value: int) -> None: ...
    @property
    def lattice(self) -> Literal["Square", "Hex"]:
        """The lattice the layers are sampled on."""
    @lattice.setter
    def lattice(self, value: Literal["Square", "Hex"]) -> None: ...
    @staticmethod
    def carpet(limit: int) -> Preset:
        """The carpet stack: every base-three corner but the centre, summed over odd scales."""
    def field(self, size: int) -> Field:
        """Samples the preset into a square field of the given side."""
    @staticmethod
    def heatmap(limit: int) -> Preset:
        """The parity heatmap: odd scales of the low corner summed on the square lattice."""
    @staticmethod
    def hive(limit: int) -> Preset:
        """The hive: the parity heatmap sampled on the hexagonal lattice."""
    @staticmethod
    def weave(limit: int) -> Preset:
        """The parity weave: the same odd scales folded to their parity instead of summed."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

class Volume:
    """A cubic grid of f32 samples, x-major."""
    def __init__(self, size: int) -> None: ...
    @property
    def data(self) -> list[float]:
        """The samples, x-major, then y, then z."""
    @data.setter
    def data(self, value: list[float]) -> None: ...
    @property
    def size(self) -> int:
        """The side in samples."""
    @size.setter
    def size(self, value: int) -> None: ...
    def at(self, x: int, y: int, z: int) -> float:
        """Reads the sample at a voxel."""
    def count(self, level: float) -> int:
        """Counts the samples at or above the level."""
    @staticmethod
    def from_data(data: list[float], size: int) -> Volume:
        """Wraps x-major samples of the side."""
    def max(self) -> float:
        """Returns the largest sample."""
    def min(self) -> float:
        """Returns the smallest sample."""
    @staticmethod
    def new(size: int) -> Volume:
        """Builds a zeroed volume of the side."""
    def plane(self, frame: dict[str, Any], out: int) -> tuple[list[float], bytes]:
        """Samples the plane of the frame on an out by out window: the values row by row, and one byte per pixel saying whether it lies inside the cube."""
    def sample(self, p: list[float]) -> float | None:
        """Reads the voxel a point of the unit cube falls in, or zero outside it."""
    def solid(self, level: float) -> NDArray[Any]:
        """Thresholds into a byte tensor: one where a sample reaches the level, zero below."""
    @staticmethod
    def from_dict(data: Any) -> Volume:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

class Layer:
    """The recipe for one moire layer."""
    @staticmethod
    def new(spec: dict[str, Any], number: int) -> dict[str, Any]:
        """Builds a layer at level 1 on a 512-pixel square lattice."""

class Spec:
    """The identity of a design: its code, base and dimension."""
    @staticmethod
    def new(code: int, base: int, dimension: int) -> dict[str, Any]:
        """Builds a spec from a code, base and dimension."""

def all(limit: int) -> list[Preset]:
    """Returns every preset stacked up to the given scale."""

def frame(normal: list[float], offset: float) -> dict[str, Any]:
    """Frames the plane normal to the direction, at the offset from zero to one across the box along it; the window is the smallest square holding every section on that normal."""

def layer(params: dict[str, Any]) -> list[bool]:
    """Samples a design over the pixel grid into a boolean mask."""

def named(name: str, limit: int) -> Preset:
    """Returns the preset the name picks."""

def render(field: Field, colorizer: Any, levels: int, symmetric: bool, invert: bool, scale: int) -> bytes:
    """Quantizes a field into colored levels and encodes PNG bytes."""

def stack(spec: dict[str, Any], numbers: list[int], combine: Literal["Sum", "And", "Xor"], level: int, lattice: Literal["Square", "Hex"], size: int, slices: list[float]) -> Field:
    """Layers one design at several side numbers into a field under the chosen combine."""

def stack_codes(specs: list[dict[str, Any]], number: int, level: int, lattice: Literal["Square", "Hex"], size: int, slices: list[float]) -> Field:
    """Sums layers of several designs at one side number into a field."""

def volume(spec: dict[str, Any], numbers: list[int], combine: Literal["Sum", "And", "Xor"], level: int, size: int) -> Volume:
    """Layers one cube design at several side numbers into a volume under the chosen combine."""
