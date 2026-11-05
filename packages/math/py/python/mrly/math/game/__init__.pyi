from typing import Any, Literal

from numpy.typing import NDArray
import mrly.math.memory

LAP: int

def cell(point: list[float], base: int, level: int) -> int:
    ...

def cells(base: int, dimension: int, level: int) -> int:
    ...

def counts(matrix: list[list[int]], start: int, levels: int) -> list[int]:
    ...

def de_bruijn(m: int, k: int) -> bytes:
    ...

def mosaic_game(mosaic: dict[str, Any]) -> dict[str, Any]:
    ...

def named_games() -> list[dict[str, Any]]:
    ...

def perron(matrix: list[list[int]]) -> float:
    ...

def reverse(rule: mrly.math.memory.Rule) -> mrly.math.memory.Rule:
    ...

def slice_game(base: int, digits: bytes, p: int, q: int) -> dict[str, Any]:
    ...

def successors(rule: mrly.math.memory.Rule, history: bytes) -> bytes:
    ...

def word_cell(word: bytes, base: int, dimension: int) -> int:
    ...
