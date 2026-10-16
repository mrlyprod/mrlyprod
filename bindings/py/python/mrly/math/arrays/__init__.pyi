from typing import Any, Literal

from numpy.typing import NDArray

MOST: int

def essential(sensors: list[int]) -> list[bool]:
    """Returns whether each sensor is essential, in the order given: whether removing it deletes a lag from the coarray, which happens exactly when some lag `t != 0` has all its pairs through it, the one pair of a lag of weight `1` or the two pairs `(s, s - t)` and `(s + t, s)` of a lag of weight `2`."""

def fractal(generator: list[int], base: int, level: int) -> list[int]:
    """Returns the fractal array of the generator at the base and level, ascending: every `sum_(i < level) g_i base^i` with each digit `g_i` in the generator."""

def holes(sensors: list[int]) -> list[int]:
    """Returns the holes of the coarray, ascending: the lags between `-span` and `span` that no pair of sensors makes."""

def law(generator: list[int], level: int) -> tuple[int, int] | None:
    """Returns the essential count and the sensor count that Theorem A of the arrays note proves for the fractal array of the generator at base `2a + 1`, `(u^level, L^level)`, `u` the count of [`paired`] digits; `None` off the theorem: a level below 2, a generator without `0`, or a generator whose coarray has a hole."""

def lost(sensors: list[int], sensor: int) -> list[int]:
    """Returns the lags that removing the sensor deletes from the coarray, ascending: those every pair of which holds it; empty when the sensor is inessential or absent."""

def paired(sensors: list[int]) -> list[int]:
    """Returns the paired sensors, ascending: each `g` with a partner `h` whose lag `g - h` has weight `1`, the set `U` of condition C1 of Cohen and Eldar; a paired sensor is essential."""

def weights(sensors: list[int]) -> list[int]:
    """Returns the coarray weights of the sensors, `w(t)` for every lag `t` from `-span` to `span` at index `t + span`, `span` the largest sensor less the least: the count of ordered pairs `(p, q)` of sensors with `p - q = t`."""
