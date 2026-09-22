from typing import Any, Literal

from numpy.typing import NDArray

class Schedule:
    """The named infinite schedules over an ordered pair of letters."""
    @staticmethod
    def all() -> list[Literal["ThueMorse", "Periodic", "Constant"]]:
        """Returns every Schedule in canonical order."""
    @staticmethod
    def frequencies(schedule: Literal["ThueMorse", "Periodic", "Constant"]) -> tuple[float, float]:
        """Returns the letter frequencies the schedule tends to."""
    @staticmethod
    def place(schedule: Literal["ThueMorse", "Periodic", "Constant"], index: int) -> int:
        """Returns the letter the schedule takes at the place, zero or one."""

def components(layers: list[dict[str, Any]]) -> int:
    """Counts the 4-connected components of a plane word without drawing it."""

def constant_functional(layers: list[dict[str, Any]]) -> float:
    """Returns the constant-word component functional of a plane word's letter frequencies,
    in log two units."""

def dimension(layers: list[dict[str, Any]]) -> float:
    """Returns the scale dimension of a word, the sum of the log fills over the sum of the log sides."""

def fill(layers: list[dict[str, Any]]) -> int:
    """Returns the filled cells of a word, the product of its letter fills."""

def fills(layers: list[dict[str, Any]]) -> list[int]:
    """Lists the filled cells of every letter, the product of which is the word's fill."""

def letter(layer: dict[str, Any]) -> dict[str, Any]:
    """Reads one plane letter: its fill, its runs, the rows and columns that wrap into a
    neighbouring copy, and its own components."""

def native(layers: list[dict[str, Any]]) -> bool:
    """Returns whether every letter renders at its own residue base, the native case where a
    periodic word folds to one residue rule at the product base."""

def period(layers: list[dict[str, Any]]) -> int:
    """Returns the shortest whole period of the letter list, its own length when no shorter block repeats."""

def prefixes(layers: list[dict[str, Any]]) -> list[dict[str, Any]]:
    """Folds a plane word letter by letter and returns the counts at every prefix."""

def rates(layers: list[dict[str, Any]]) -> list[tuple[float, float]]:
    """Returns the prefix rates of a plane word in log two units, the component rate
    `(1/L) log2 comp` and the fill rate `(1/L) log2 fill` at every prefix length."""

def side(layers: list[dict[str, Any]]) -> int:
    """Returns the side of a word, the product of its letter sides."""

def spell(schedule: Literal["ThueMorse", "Periodic", "Constant"], pair: tuple[dict[str, Any], dict[str, Any]], length: int) -> list[dict[str, Any]]:
    """Spells the first letters of a schedule over an ordered pair of letters."""

def staircase(depth: int) -> list[dict[str, Any]]:
    """Builds the carpet staircase word to the depth, the stacked prefixes `magic(3)`,
    then `magic(3,5)`, then `magic(3,5,7)`, and so on."""

def thue_morse(index: int) -> int:
    """Returns the Thue-Morse letter at the place, the parity of its binary digit sum."""
