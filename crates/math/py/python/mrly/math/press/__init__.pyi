from typing import Any, Literal

from numpy.typing import NDArray

CORNERS: int

class Press:
    """The tally press: one pass over the integers weighs every design of a universe at once."""
    def __init__(self, dimension: int, base: int) -> None: ...
    @property
    def dimension(self) -> int:
        """The design dimension of the universe."""
    @dimension.setter
    def dimension(self, value: int) -> None: ...
    @property
    def base(self) -> int:
        """The numeral base of the universe."""
    @base.setter
    def base(self, value: int) -> None: ...
    def add(self, number: int, weight: int) -> None:
        """Adds a weighted number to its usage bucket."""
    @staticmethod
    def new(dimension: int, base: int) -> Press:
        """Builds an empty press over every design of the dimension and base."""
    def total(self, code: int) -> int:
        """Returns the total weight the design at a code has collected."""
    def totals(self) -> list[int]:
        """Returns every design's total in code order by one subset-sum transform."""
    @staticmethod
    def from_dict(data: Any) -> Press:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

def containing(number: int, dimension: int, base: int) -> int:
    """Returns the number of designs of the dimension and base that contain the number."""

def coordinates(number: int, dimension: int, base: int) -> list[int]:
    """Splits a number into its dimension coordinates, one base digit peeled per axis in parallel."""

def count_below(code: int, dimension: int, base: int, limit: int) -> int:
    """Counts the members of a design below the limit."""

def distinct(number: int, dimension: int, base: int) -> int:
    """Returns the count of distinct digit vectors the number uses."""

def interleave(coords: list[int], base: int) -> int:
    """Weaves dimension coordinates back into their single interleaved number."""

def layer_table(layer: dict[str, Any]) -> list[bool]:
    """Returns the allowed digit table of one magic layer, one flag per cell of its tile."""

def member(code: int, number: int, dimension: int, base: int) -> bool:
    """Returns whether every digit vector of the number lies in the design."""

def members(code: int, dimension: int, base: int, count: int) -> list[int]:
    """Returns the first members of a design in ascending order."""

def profile(code: int, dimension: int, base: int, level: int) -> list[int]:
    """Returns the diagonal slice profile of one design pressed to a fractal level."""

def usage(number: int, dimension: int, base: int) -> int:
    """Returns the corner-usage mask of a number, one bit per digit vector its expansion uses."""

def word_count(layers: list[dict[str, Any]]) -> int:
    """Counts the members of a magic word from its layer fills, without enumeration."""

def word_member(layers: list[dict[str, Any]], number: int) -> bool:
    """Returns whether the number lies in the magic word's composed design."""

def word_members(layers: list[dict[str, Any]]) -> list[int]:
    """Enumerates every member of the magic word in ascending order."""

def word_profile(layers: list[dict[str, Any]]) -> list[int]:
    """Returns the diagonal slice profile of a magic word by the substitution product."""
