from typing import Any, Literal

from numpy.typing import NDArray

def byte_cube(value: Any) -> list[list[bytes]]:
    ...

def byte_grid(value: Any) -> list[bytes]:
    ...

def color_grid(value: Any) -> list[NDArray[Any]]:
    ...

def count_cube(value: Any) -> list[int]:
    ...

def count_grid(value: Any) -> list[int]:
    ...

def parse(text: str) -> Any:
    ...

def tag_layer(counts: list[int], shape: list[int]) -> NDArray[Any]:
    ...

def types_field(data: Any) -> Any:
    ...
