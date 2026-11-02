from typing import Any, Literal

from numpy.typing import NDArray

def coprime_pairs(n: int) -> int:
    """Counts the ordered pairs of coprime coordinates between one and n: twice the totient sum less one."""

def farey(order: int) -> list[dict[str, Any]]:
    """Walks the Farey sequence of the order by the Stern-Brocot mediant recurrence from zero over one to one over one: every reduced fraction with denominator at most the order, ascending."""

def grid(n: int) -> list[dict[str, Any]]:
    """Lists the grid crossings of a window's nodes, row-major over the ascending axis nodes."""

def new_nodes(n: int) -> int:
    """Counts the nodes window n lights that window n minus one lacked: two at window one, phi of n after."""

def pi_estimate(n: int) -> float:
    """Estimates pi from visibility: the density of coprime pairs in the n-by-n window tends to six over pi squared."""

def recovered(n: int, dimension: int) -> float:
    """Recovers the constant the dimension hides from the visible count of the window, pi at an even dimension and zeta of the dimension at an odd one."""

def visible_density(dimension: int) -> float:
    """The density the visible count of a window in the dimension walks to, one over zeta of the dimension."""

def zeta_factor(dimension: int) -> float | None:
    """The rational factor r with zeta of the dimension equal to r times pi to the dimension, read off the Bernoulli fraction; none at an odd dimension or past twelve."""

def zeta_whole(s: int) -> float:
    """The value zeta takes at a whole argument above one, the exact Bernoulli form at an even one and the Euler-Maclaurin sum at an odd one."""
