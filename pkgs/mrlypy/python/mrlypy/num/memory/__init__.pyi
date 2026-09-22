from typing import Any, Literal

from numpy.typing import NDArray

SPAN: int
SWEEPS: int
TOLERANCE: float

class Rule:
    """A rule on `k` consecutive digits of a design word."""
    def __init__(self, dimension: int, width: int, code: int) -> None: ...
    @property
    def dimension(self) -> int:
        """The dimension `D`, one to three."""
    @property
    def width(self) -> int:
        """The window width `k`, at least one."""
    @property
    def code(self) -> int:
        """The window code, bit `w` set when window `w` is allowed."""
    def accepts(self, word: list[int]) -> bool:
        """Returns whether a word, coarsest digit first, is accepted."""
    def allowed(self, window: int) -> bool:
        """Returns whether the window is allowed, and false for any window out of range."""
    def alphabet(self) -> list[int]:
        """Returns the letters that stand in at least one allowed window."""
    def codes(self) -> int:
        """Returns the count of rules of this shape, `2^(2^(k D))`."""
    @staticmethod
    def full(dimension: int, width: int) -> Rule:
        """Returns the rule that allows every window."""
    def letters(self) -> int:
        """Returns the letter count `2^D`, the digit vectors of the cube's corners."""
    @staticmethod
    def new(dimension: int, width: int, code: int) -> Rule:
        """Builds a rule from its dimension, its width and its code."""
    def states(self) -> int:
        """Returns the state count `2^((k - 1) D)`, the windows of one digit less that the transfer matrix runs on."""
    def windows(self) -> int:
        """Returns the window count `2^(k D)`."""
    @staticmethod
    def from_dict(data: Any) -> Rule:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

def allowed_windows(rule: Rule) -> int:
    """Returns the count of allowed windows `card W`, the bits the code sets inside its window range."""

def cells(rule: Rule, level: int) -> list[int]:
    """Returns the accepted words of the level as cell indices of the `2^L` grid, `x` from bit `0` of every digit, `y` from bit `1`, `z` from bit `2`, coarsest digit first."""

def counts(rule: Rule, levels: int) -> list[int]:
    """Returns `N_W(L)`, the count of accepted words, for `L = 1 ..= levels`, and stops early on the level whose count overruns a `u64`."""

def exponent(rule: Rule) -> float:
    """Returns the growth exponent `log_2 rho`, the growth per digit of the accepted word count."""

def kappa(rule: Rule) -> float:
    """Returns the memory number `kappa(W) = log_2(card W) / k - log_2 rho`, the bits a digit spends on memory."""

def perron(rule: Rule) -> float:
    """Returns the Perron root of the transfer matrix, the count's growth per level."""

def transfer(rule: Rule) -> list[list[int]]:
    """Returns the transfer matrix on the `(k - 1)`-windows: entry `(s, t)` is one when the window that overlaps state `s` onto state `t` is allowed."""
