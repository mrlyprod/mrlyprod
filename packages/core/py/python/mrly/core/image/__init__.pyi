from typing import Any, Literal

from numpy.typing import NDArray

def blur(pixels: NDArray[Any], width: int, height: int, radius: int) -> NDArray[Any]:
    ...

def new(width: int, height: int, colors: NDArray[Any]) -> NDArray[Any]:
    ...

def resample(image: NDArray[Any], width: int, height: int, filter: Literal["Nearest", "Linear", "Box"]) -> NDArray[Any]:
    ...

def scale(image: NDArray[Any], scale: int) -> NDArray[Any]:
    ...
