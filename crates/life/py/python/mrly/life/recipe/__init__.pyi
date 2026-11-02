from typing import Any, Literal

from numpy.typing import NDArray

class Run:
    """A recipe's run, stepped one generation at a time until its stop."""
    def __init__(self, recipe: dict[str, Any]) -> None: ...
    def background(self) -> int:
        """Returns the background the frame sits on, 1 once a flipping rule has turned it."""
    def birth(self) -> list[int]:
        """Returns the birth counts drawn against the budget."""
    def budget(self) -> int:
        """Returns the mask's budget, the most neighbours a cell can count."""
    def cap(self) -> int:
        """Returns the canvas side: an island's cap, at least its seed board, or a wallpaper's torus."""
    def flips(self) -> bool:
        """Returns whether the rule flips its background, birth holding zero."""
    def frame(self) -> dict[str, Any]:
        """Returns the live frame: the cropped island pattern, or the whole torus."""
    def generation(self) -> int:
        """Returns the generation reached."""
    def mask(self) -> NDArray[Any]:
        """Returns the mask as a square tensor of bytes, its centre popped."""
    @staticmethod
    def new(recipe: dict[str, Any]) -> Run:
        """Sets a recipe up at generation zero: the tiled seed, the mask's kernel, the rule drawn against its budget."""
    def origin(self) -> list[int]:
        """Returns the row and column of the frame's top-left corner, the seed's top-left corner being the origin."""
    def population(self) -> int:
        """Returns the live cells of the frame."""
    def reach(self) -> int:
        """Returns the mask's reach, half its side."""
    def side(self) -> int:
        """Returns the side of the seed board, the seed's side to its level times its tiling."""
    def step(self) -> dict[str, Any] | None:
        """Advances one generation and returns the stop the moment the run ends; after a stop it returns the same stop and changes nothing."""
    def stop(self) -> dict[str, Any] | None:
        """Returns the stop once the run has ended."""
    def survive(self) -> list[int]:
        """Returns the survive counts drawn against the budget."""

class Mode:
    """The mode of a run."""
    @staticmethod
    def all() -> list[Literal["Island", "Wallpaper"]]:
        """Returns every Mode in canonical order."""

def grammar() -> dict[str, Any]:
    """Lists what every knob of a recipe may offer, in grammar order."""

def name(recipe: dict[str, Any]) -> str:
    """Writes a recipe as its name, the one string that regenerates it."""

def parse(name: str) -> dict[str, Any]:
    """Reads a recipe out of its name."""
