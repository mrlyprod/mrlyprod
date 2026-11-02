from typing import Any, Literal

from numpy.typing import NDArray
import mrly.math.memory

LAP: int

def cell(point: list[float], base: int, level: int) -> int:
    """Returns the index of the level-`k` cell holding a point of `[0, 1)^D`: `floor(x b^k) + floor(y b^k) b^k + ...`, row major with coordinate `0` least significant, the dimension read off the point."""

def cells(base: int, dimension: int, level: int) -> int:
    """Returns the count of level-`k` cells of the cube, `b^(D k)`."""

def counts(matrix: list[list[int]], start: int, levels: int) -> list[int]:
    """Returns the path counts from a state, `e_start M^L 1` for `L = 1 ..= levels`: the filled cells of a mosaic slot or of a slice at each level, and stops early on the level whose count overruns a `u64`."""

def de_bruijn(m: int, k: int) -> bytes:
    """Returns a de Bruijn cycle of order `k` over `m` letters: `m^k` letters in which every word of length `k` occurs exactly once as a cyclic window, so played as the draw schedule of an `m`-digit design it paints every level-`k` cell once per lap, and from a cold start once in `m^k + k - 1` draws."""

def mosaic_game(mosaic: dict[str, Any]) -> dict[str, Any]:
    """Reads the game of a mosaic: its substitution matrix, Perron root, dimension and edges."""

def named_games() -> list[dict[str, Any]]:
    """Returns the named games of the square, corner `c = x + 2 y`, cyclic order `0, 1, 3, 2` anticlockwise with `y` up: the seven one-step games and the width-3 game that forbids a neighbour after a repeat."""

def perron(matrix: list[list[int]]) -> float:
    """Returns the Perron root of a square nonnegative integer matrix, the growth per level of the paths its graph counts."""

def reverse(rule: mrly.math.memory.Rule) -> mrly.math.memory.Rule:
    """Returns the rule whose windows are the rule's windows read backwards."""

def slice_game(base: int, digits: bytes, p: int, q: int) -> dict[str, Any]:
    """Reads the game of a plane design, corners `c = x + b y`, cut by the line `x + y = p / q`: its carry states, edges, matrix, Perron root and dimension."""

def successors(rule: mrly.math.memory.Rule, history: bytes) -> bytes:
    """Returns the letters a restricted game may draw next: the corners `c` for which some allowed window, read in time order, ends in the last `k - 1` draws of the history followed by `c`."""

def word_cell(word: bytes, base: int, dimension: int) -> int:
    """Returns the cell of a word of corners read coarsest digit first, the cell the game lands in after playing the word's digits last letter first."""
