from typing import Any, Literal

from numpy.typing import NDArray

class Layout:
    """A force-directed layout: every node repels every other, every branch pulls its ends together, and a cooling cap on the move per tick lets the lattice settle."""
    def __init__(self, positions: list[float], branches: list[tuple[int, int]], dim: int, seed: int) -> None: ...
    def energy(self) -> float:
        """Returns the mean net force per node in units of `k` after the last tick."""
    @staticmethod
    def from_network(network: Network, seed: int) -> Layout:
        """Starts a layout from a network's own positions and branches."""
    def ideal(self) -> float:
        """Returns the ideal branch length `k`."""
    def moved(self) -> float:
        """Returns the mean distance a node moved in the last tick."""
    @staticmethod
    def new(positions: list[float], branches: list[tuple[int, int]], dim: int, seed: int) -> Layout:
        """Starts a layout from flat positions, `dim` floats per node, and the branch pairs."""
    def nodes(self) -> int:
        """Returns the node count."""
    def positions(self) -> list[float]:
        """Returns the positions, `dim` floats per node."""
    def step(self, ticks: int) -> float:
        """Runs the ticks and returns the energy left: the mean net force per node in units of `k`."""
    def temperature(self) -> float:
        """Returns the cap on one node's move in the next tick."""
    def ticks(self) -> int:
        """Returns the ticks stepped so far."""

class Network:
    """A spatial graph of nodes and branches."""
    def __init__(self, dim: int) -> None: ...
    @property
    def dim(self) -> int:
        """The dimension every position must match."""
    @dim.setter
    def dim(self, value: int) -> None: ...
    @property
    def nodes(self) -> list[dict[str, Any]]:
        """The nodes in insertion order."""
    @nodes.setter
    def nodes(self, value: list[dict[str, Any]]) -> None: ...
    @property
    def branches(self) -> list[dict[str, Any]]:
        """The branches in insertion order."""
    @branches.setter
    def branches(self, value: list[dict[str, Any]]) -> None: ...
    def add_branch(self, parent: int, child: int, radius: float) -> None:
        """Appends a branch between two node indices."""
    def add_node(self, position: list[float]) -> int:
        """Appends a node at the position and returns its index."""
    def adjacency(self) -> dict[int, list[int]]:
        """Returns the undirected neighbor lists of every node."""
    def degree(self) -> list[int]:
        """Returns each node's branch count, indexed like the node list."""
    @staticmethod
    def new(dim: int) -> Network:
        """Builds an empty network of the given dimension."""
    @staticmethod
    def from_dict(data: Any) -> Network:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

def census(network: Network) -> dict[str, Any]:
    """Takes the full census of a network."""

def components(network: Network) -> int:
    """Counts the connected components of the network."""

def core_graph(grid: NDArray[Any]) -> Network:
    """Extracts the network of filled sites joined to their axis neighbors."""

def edge_graph(grid: NDArray[Any]) -> Network:
    """Extracts the network of corners and edges outlining every filled site."""

def fractal_dimension(network: Network, samples: int) -> float:
    """Estimates the box-counting dimension of the node cloud over a ladder of halving boxes, one rung per sample."""

def junctions(network: Network) -> int:
    """Counts the nodes of degree three or more."""

def largest_component(network: Network) -> Network:
    """Extracts the largest connected piece as a network of its own, branches re-indexed."""

def roles(network: Network) -> list[Literal["Alone", "Tip", "Through", "Junction"]]:
    """Tags every node by its degree, indexed like the node list."""

def tips(network: Network) -> int:
    """Counts the nodes of degree one."""

def total_length(network: Network) -> float:
    """Sums the straight-line lengths of every branch."""

def tunnel_graph(grid: NDArray[Any]) -> Network:
    """Extracts the core graph of the inverted grid, joining empty sites instead."""
