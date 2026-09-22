from typing import Any, Literal

from numpy.typing import NDArray
from . import elementary, render, source

class Config:
    """The rulebook of a life run."""
    def __init__(self, mask: dict[str, Any], birth: Counts, survive: Counts) -> None: ...
    @property
    def mask(self) -> dict[str, Any]:
        """The neighborhood mask."""
    @mask.setter
    def mask(self, value: dict[str, Any]) -> None: ...
    @property
    def birth(self) -> Counts:
        """The neighbor counts that create a cell."""
    @birth.setter
    def birth(self, value: Counts) -> None: ...
    @property
    def survive(self) -> Counts:
        """The neighbor counts that keep a cell."""
    @survive.setter
    def survive(self, value: Counts) -> None: ...
    @property
    def boundary(self) -> Literal["Constant", "Wrap"]:
        """The edge policy."""
    @boundary.setter
    def boundary(self, value: Literal["Constant", "Wrap"]) -> None: ...
    @property
    def max_generations(self) -> int:
        """The generation cap."""
    @max_generations.setter
    def max_generations(self, value: int) -> None: ...
    @property
    def grid_size(self) -> int:
        """The tiling factor applied to the seed."""
    @grid_size.setter
    def grid_size(self, value: int) -> None: ...
    @property
    def padding(self) -> int:
        """The dead border added around the seed."""
    @padding.setter
    def padding(self, value: int) -> None: ...
    def budget(self) -> int:
        """Returns the largest neighbor count the mask can reach."""
    def counts(self) -> tuple[list[int], list[int]]:
        """Resolves the birth and survive counts against the mask's budget."""
    @staticmethod
    def new(mask: dict[str, Any], birth: Counts, survive: Counts) -> Config:
        """Builds a config with a constant boundary, a 64-generation cap, no tiling and no padding."""
    @staticmethod
    def from_dict(data: Any) -> Config:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

class Counts:
    """The neighbor counts one side of a rule fires on."""
    @staticmethod
    def drawn(seq: Source, zeros: bool, ones: bool) -> Counts:
        """Builds the counts a sequence lays down, keeping zeros and ones on request."""
    @staticmethod
    def list(counts: list[int]) -> Counts:
        """Spells the counts outright."""
    def values(self, budget: int) -> list[int]:
        """Returns the counts, a drawn side resolved against the mask's neighbor budget."""
    @staticmethod
    def from_dict(data: Any) -> Counts:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

class Life:
    """The recorded run of one seed."""
    @property
    def grids(self) -> list[dict[str, Any]]:
        """Every generation in order."""
    @grids.setter
    def grids(self, value: list[dict[str, Any]]) -> None: ...
    @property
    def fate(self) -> Literal["Dead", "Alive", "Loop", "Timeout"]:
        """The run's ending."""
    @fate.setter
    def fate(self, value: Literal["Dead", "Alive", "Loop", "Timeout"]) -> None: ...
    @property
    def count(self) -> int:
        """The number of recorded generations."""
    @count.setter
    def count(self, value: int) -> None: ...
    @property
    def loop_length(self) -> int:
        """The cycle length when the fate is a loop, else zero."""
    @loop_length.setter
    def loop_length(self, value: int) -> None: ...
    def last(self) -> dict[str, Any] | None:
        """Returns the final grid, or None when the run is empty."""
    @staticmethod
    def from_dict(data: Any) -> Life:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

class Rule:
    """A life rule: the birth and survival counts and whether the edge wraps."""
    def __init__(self, birth: Counts, survive: Counts, wrap: bool) -> None: ...
    @property
    def birth(self) -> Counts:
        """The neighbor counts that create a cell, listed or drawn from a sequence."""
    @birth.setter
    def birth(self, value: Counts) -> None: ...
    @property
    def survive(self) -> Counts:
        """The neighbor counts that keep a cell, listed or drawn from a sequence."""
    @survive.setter
    def survive(self, value: Counts) -> None: ...
    @property
    def wrap(self) -> bool:
        """Whether the edge wraps, false unless said."""
    @wrap.setter
    def wrap(self, value: bool) -> None: ...
    def boundary(self) -> Literal["Constant", "Wrap"]:
        """Returns the edge policy the rule runs under."""
    def checked(self) -> Rule:
        """Folds a decoded value to its canonical form, or an error for one outside the kind."""
    def config(self, mask: dict[str, Any]) -> Config:
        """Builds a life config running this rule over a neighborhood mask."""
    @staticmethod
    def from_file(text: str) -> Rule:
        """Reads a filename back into the value, or an error."""
    @staticmethod
    def from_json(text: str) -> Rule:
        """Reads a JSON object into its canonical value, or an error naming the broken key."""
    @staticmethod
    def from_url(text: str) -> Rule:
        """Reads a path and query string back into the value, or an error."""
    @staticmethod
    def new(birth: Counts, survive: Counts, wrap: bool) -> Rule:
        """Builds a rule from its counts and edge policy, listed counts folded to a sorted set."""
    @staticmethod
    def of(config: Config) -> Rule:
        """Reads the rule out of a life config."""
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
    def from_dict(data: Any) -> Rule:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

class Source:
    """A named source of neighbor-count values."""
    @staticmethod
    def all() -> list[Source]:
        """Returns every fixed sequence, the seeded and coded families excluded."""
    @staticmethod
    def designs() -> list[Source]:
        """Returns the seventeen mrly design families: the grid, the four classics and their antis."""
    def is_random(self) -> bool:
        """Returns whether the sequence is a seeded random draw."""
    def name(self) -> str:
        """Returns the sequence's parseable name, the one string that regenerates it."""
    @staticmethod
    def numbers() -> list[Source]:
        """Returns the six number sequences, the random one listed under seed zero."""
    def oeis(self) -> str | None:
        """Returns the sequence's OEIS id, or None off the encyclopedia."""
    @staticmethod
    def parse(name: str) -> Source:
        """Parses a sequence name back to its source."""
    @staticmethod
    def read(text: str) -> tuple[Source, str] | None:
        """Reads a canonical name off the front of the text, returning the tail left over."""
    @staticmethod
    def from_dict(data: Any) -> Source:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

class Boundary:
    """The edge policy of a life grid."""
    @staticmethod
    def all() -> list[Literal["Constant", "Wrap"]]:
        """Returns every Boundary in canonical order."""
    @staticmethod
    def wrap(boundary: Literal["Constant", "Wrap"]) -> bool:
        """Returns whether the edges wrap."""

class Fate:
    """The ending of a life run."""
    @staticmethod
    def all() -> list[Literal["Dead", "Alive", "Loop", "Timeout"]]:
        """Returns every Fate in canonical order."""

def affine(rule: int) -> bool:
    """Returns whether a rule is affine, its algebraic degree at most one."""

def animate(seed: dict[str, Any], config: Config) -> Life:
    """Runs a seed under a config until it fixes, loops or times out, recording every generation."""

def churn(grids: list[dict[str, Any]]) -> float:
    """Returns the mean fraction of sites changed between consecutive grids."""

def corner_bits(rule: int) -> bytes:
    """Returns the eight output bits of a rule, corner `i` at index `i = 4 x0 + 2 x1 + x2`."""

def counts(seq: Source, max_neighbors: int, include_zeros: bool, include_ones: bool) -> list[int]:
    """Returns the sequence up to max_neighbors, keeping zeros and ones only on request."""

def crop(grids: list[dict[str, Any]]) -> list[dict[str, Any]]:
    """Crops a frame sequence to the centred square bounding every cell ever alive."""

def cube_orbit(rule: int) -> bytes:
    """Returns the rules a rule reaches under the signed axis permutations of the cube, in ascending order."""

def design_mask(dimension: int, code: int, number: int, level: int) -> NDArray[Any]:
    """Builds the base-2 design mask a code names at an odd side grown to the given Kronecker
    level, its centre popped."""

def entropy(grid: dict[str, Any]) -> int:
    """Returns the grid's binary Shannon entropy in millibits."""

def frames(grids: list[dict[str, Any]], scale: int) -> list[bytes]:
    """Renders grids to white-on-black PNG bytes at a pixel scale."""

def gasket(rule: int) -> str | None:
    """Returns the base-2 plane design a rule's single seed draws, or None when it draws none."""

def genus(rule: int) -> str:
    """Returns the genus of a rule's cube class: `iso` when it meets a level set, `axis` when it meets an axis-pinned block, else `comp`."""

def heatmap(grids: list[dict[str, Any]], scale: int) -> list[bytes]:
    """Renders a whole run's cumulative-visit heatmap frames with the heat ramp."""

def history(row: bytes, rule: int, steps: int, wrap: bool) -> NDArray[Any]:
    """Returns the space-time diagram of a seed row, row 0 the seed and then one row per generation."""

def lambda_(rule: int) -> float:
    """Returns Langton's lambda, the popcount over eight."""

def lattice_index(mask: NDArray[Any]) -> int:
    """Returns the index of the lattice the mask offsets generate together with the centre, zero when they do not span the dimension."""

def mask_offsets(mask: NDArray[Any]) -> list[list[int]]:
    """Returns the offsets a mask's filled sites take from its centre, the centre itself dropped."""

def moore() -> dict[str, Any]:
    """Builds the 3 by 3 Moore mask, every site on but the center."""

def movie(grids: list[dict[str, Any]], scale: int, delay: int) -> bytes:
    """Renders grids into one looping black-on-white gif, the delay in hundredths of a second."""

def next_grid(cell: dict[str, Any], birth: list[int], survive: list[int], mask: NDArray[Any], boundary: Literal["Constant", "Wrap"]) -> dict[str, Any]:
    """Advances a grid one generation under birth and survive counts, a neighbor mask and a boundary."""

def npn_class(rule: int) -> bytes:
    """Returns the rules a rule reaches under the cube group together with the output complement, its NPN class, in ascending order."""

def outer_totalistic(rule: int) -> tuple[list[int], list[int]] | None:
    """Returns the birth and survive counts of a rule read outer-totalistically on its two outer cells, or None when it does not read them by count alone."""

def popcount(rule: int) -> int:
    """Returns the count of neighbourhoods a rule sends to one."""

def reversible(rule: int) -> bool:
    """Returns whether a rule is reversible, by the pair graph on the de Bruijn nodes pruned to its bi-infinite core."""

def rule_degree(rule: int) -> int:
    """Returns the GF(2) algebraic degree of a rule, minus one for the zero rule."""

def rule_name(rule: int) -> str:
    """Returns the design name a rule carries, `bang dim 3, code <rule>`."""

def single_seed(rule: int, steps: int) -> NDArray[Any]:
    """Returns the single-seed diagram: one live cell run the given generations on a line padded by `steps` cells beyond the `2 steps + 1` window on each side, cropped back to that window."""

def step(row: bytes, rule: int, wrap: bool) -> bytes:
    """Advances one row one generation, a constant-0 boundary unless the edges wrap."""

def surjective(rule: int) -> bool:
    """Returns whether a rule is surjective on bi-infinite lines, by the de Bruijn subset walk from the full node set."""

def tessellate(grids: list[dict[str, Any]], min_canvas: int) -> list[dict[str, Any]]:
    """Tiles every frame n by n to reach at least min_canvas a side, unchanged when already there."""

def wolfram_class(rule: int) -> bytes:
    """Returns the rules a rule reaches under left-right reflection and conjugation, Wolfram's equivalence, in ascending order."""
