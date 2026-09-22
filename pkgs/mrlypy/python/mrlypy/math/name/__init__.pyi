from typing import Any, Literal

from numpy.typing import NDArray

class Bang:
    """A design code pinned to its dimension, lattice and base, with one unit index per filled digit when it twists."""
    def __init__(self, code: int, dim: int, base: int) -> None: ...
    @property
    def dim(self) -> int:
        """The number of axes."""
    @dim.setter
    def dim(self, value: int) -> None: ...
    @property
    def lattice(self) -> Literal["square", "hex"]:
        """The lattice, square unless said."""
    @lattice.setter
    def lattice(self, value: Literal["square", "hex"]) -> None: ...
    @property
    def base(self) -> int:
        """The digits per axis, 2 unless said."""
    @base.setter
    def base(self, value: int) -> None: ...
    @property
    def code(self) -> int:
        """The design as a number."""
    @code.setter
    def code(self, value: int) -> None: ...
    @property
    def twist(self) -> list[int] | None:
        """One unit index per filled digit, absent when nothing turns."""
    @twist.setter
    def twist(self, value: list[int] | None) -> None: ...
    def cells(self) -> int:
        """Returns the number of digits the code addresses."""
    def checked(self) -> Bang:
        """Folds a decoded value to its canonical form, or an error for one outside the kind."""
    @staticmethod
    def from_file(text: str) -> Bang:
        """Reads a filename back into the value, or an error."""
    @staticmethod
    def from_json(text: str) -> Bang:
        """Reads a JSON object into its canonical value, or an error naming the broken key."""
    @staticmethod
    def from_url(text: str) -> Bang:
        """Reads a path and query string back into the value, or an error."""
    @staticmethod
    def new(code: int, dim: int, base: int) -> Bang:
        """Pins a code to its dimension and base on the square lattice."""
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
    def from_dict(data: Any) -> Bang:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

class Sequence:
    """A design sequence's address: the design, the reading taken off it and the index it runs along."""
    def __init__(self, code: int, dim: int, base: int, measure: str, axis: str) -> None: ...
    @property
    def dim(self) -> int:
        """The number of axes."""
    @dim.setter
    def dim(self, value: int) -> None: ...
    @property
    def base(self) -> int:
        """The digits per axis, 2 unless said."""
    @base.setter
    def base(self, value: int) -> None: ...
    @property
    def code(self) -> int:
        """The design as a number."""
    @code.setter
    def code(self, value: int) -> None: ...
    @property
    def measure(self) -> str:
        """The reading taken."""
    @measure.setter
    def measure(self, value: str) -> None: ...
    @property
    def axis(self) -> str:
        """The index the reading runs along."""
    @axis.setter
    def axis(self, value: str) -> None: ...
    def checked(self) -> Sequence:
        """Folds a decoded value to its canonical form, or an error for one outside the kind."""
    def design(self) -> Bang:
        """Returns the design pinned to its dimension and base."""
    @staticmethod
    def from_file(text: str) -> Sequence:
        """Reads a filename back into the value, or an error."""
    @staticmethod
    def from_json(text: str) -> Sequence:
        """Reads a JSON object into its canonical value, or an error naming the broken key."""
    @staticmethod
    def from_url(text: str) -> Sequence:
        """Reads a path and query string back into the value, or an error."""
    @staticmethod
    def new(code: int, dim: int, base: int, measure: str, axis: str) -> Sequence:
        """Pins a design's reading to its measure and axis."""
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
    def from_dict(data: Any) -> Sequence:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

class Word:
    """A magic word: an ordered list of design letters, first letter outermost, each at its own side."""
    def __init__(self, dim: int, letters: list[tuple[int, int]]) -> None: ...
    @property
    def dim(self) -> int:
        """The number of axes every letter shares."""
    @dim.setter
    def dim(self, value: int) -> None: ...
    @property
    def magic(self) -> list[int]:
        """The codes of the letters in order."""
    @magic.setter
    def magic(self, value: list[int]) -> None: ...
    @property
    def side(self) -> list[int]:
        """The side each letter renders at."""
    @side.setter
    def side(self, value: list[int]) -> None: ...
    @property
    def base(self) -> list[int] | None:
        """The base of each letter, absent when every letter is base 2."""
    @base.setter
    def base(self, value: list[int] | None) -> None: ...
    def bases(self) -> list[int]:
        """Returns the base of every letter, 2 where the name says nothing."""
    def checked(self) -> Word:
        """Folds a decoded value to its canonical form, or an error for one outside the kind."""
    @staticmethod
    def from_file(text: str) -> Word:
        """Reads a filename back into the value, or an error."""
    @staticmethod
    def from_json(text: str) -> Word:
        """Reads a JSON object into its canonical value, or an error naming the broken key."""
    @staticmethod
    def from_url(text: str) -> Word:
        """Reads a path and query string back into the value, or an error."""
    def letters(self) -> list[Bang]:
        """Returns every letter as a design pinned to the word's dimension and its own base."""
    @staticmethod
    def new(dim: int, letters: list[tuple[int, int]]) -> Word:
        """Pins an ordered letter list at base 2."""
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
    def from_dict(data: Any) -> Word:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

class Lattice:
    """The lattice the cells sit on."""
    @staticmethod
    def default() -> Literal["square", "hex"]:
        """Returns the default Lattice."""
    @staticmethod
    def is_square(lattice: Literal["square", "hex"]) -> bool:
        """Returns whether this is the square lattice."""
    @staticmethod
    def units(lattice: Literal["square", "hex"]) -> int:
        """Returns the number of unit directions a twist may pick from."""
