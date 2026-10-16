from typing import Any, Literal

from numpy.typing import NDArray

WALK_LIMIT: int

def axis_maps(base: int) -> list[list[int]]:
    """Returns the distinct rotation and reflection maps of a base-q axis."""

def bracelets(max_base: int) -> list[int]:
    """Returns the distinct one-dimensional design counts for bases 1 through max_base."""

def canonical(group: list[list[int]], code: int) -> int:
    """Returns the least code of the design's orbit."""

def carry(element: list[int], code: int) -> int:
    """Carries a code through one group element."""

def class_sequence(max_dimension: int) -> list[int]:
    """Returns the fill-class counts for dimensions 1 through max_dimension."""

def classes(dimension: int) -> int:
    """Counts the fill classes of a dimension, the popcount profiles a base-2 design can have: one more than the corners of each weight, multiplied over the weights, A129824 at the dimension."""

def distinct_designs(base: int, dimension: int) -> int:
    """Counts base-q designs distinct under symmetry."""

def even_fill_is_balanced(number: int, dimension: int, popcount: int) -> int:
    """Returns the collapsed fill count at an even side number."""

def fill_from_corners(filled: list[bytes], number: int, dimension: int) -> int:
    """Returns the filled-cell count of a binary design at a side number, folded from its filled corners."""

def group(base: int, dimension: int) -> list[list[int]]:
    """Returns the symmetry group as cell maps, each sending the cell at index `i` to `element[i]`."""

def group_order(base: int, dimension: int) -> int:
    """Returns the symmetry group order counted from the enumerated axis maps."""

def orbit(group: list[list[int]], code: int) -> list[int]:
    """Returns every code a design reaches under the group."""

def predicted_group_order(base: int, dimension: int) -> int:
    """Returns the closed-form group order the axis-map count must match."""

def representatives(base: int, dimension: int) -> list[tuple[int, int]]:
    """Walks every code of a base and dimension and returns each orbit's least code with the orbit's size."""

def sequence(base: int, max_dimension: int) -> list[int]:
    """Returns the distinct-design counts for dimensions 1 through max_dimension."""

def total_designs(base: int, dimension: int) -> int:
    """Returns the raw design count before symmetry, two to the number of cells."""
