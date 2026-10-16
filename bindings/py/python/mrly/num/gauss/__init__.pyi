from typing import Any, Literal

from numpy.typing import NDArray

class Window:
    """The symmetric window of one ring: every point within a reach, with the norms sieved once."""
    def __init__(self, ring: Literal["Gaussian", "Eisenstein"], radius: int) -> None: ...
    def census(self) -> dict[str, Any]:
        """Counts every class inside."""
    def class_(self, a: int, b: int) -> Literal["Zero", "Unit", "Ramified", "Split", "Inert", "Composite"]:
        """Classifies a point: prime when its norm is a rational prime, or when it is a unit times a rational prime that stays prime."""
    def holds(self, a: int, b: int) -> bool:
        """Returns whether a point lies inside."""
    @staticmethod
    def new(ring: Literal["Gaussian", "Eisenstein"], radius: int) -> Window:
        """Opens the window of a ring out to a reach, sieving every norm inside it."""
    def points(self) -> list[tuple[int, int]]:
        """Lists every point inside, row by row from the bottom left of the bounding square."""
    def radius(self) -> int:
        """Returns the reach."""
    def ring(self) -> Literal["Gaussian", "Eisenstein"]:
        """Returns the ring."""
    @staticmethod
    def from_dict(data: Any) -> Window:
        """Reads plain data into the class."""
    def to_dict(self) -> Any:
        """Returns the value as plain data."""

class Class:
    """What a point of the ring is."""
    @staticmethod
    def prime(class_: Literal["Zero", "Unit", "Ramified", "Split", "Inert", "Composite"]) -> bool:
        """Returns whether the class is prime."""
    @staticmethod
    def word(class_: Literal["Zero", "Unit", "Ramified", "Split", "Inert", "Composite"]) -> str:
        """Returns the class as a word."""

class Ring:
    """The two rings of whole numbers in the plane, each a pair (a, b) on its own lattice."""
    @staticmethod
    def associates(ring: Literal["Gaussian", "Eisenstein"], a: int, b: int) -> list[tuple[int, int]]:
        """Returns the unit multiples of a point, the point first, turning anticlockwise."""
    @staticmethod
    def canon(ring: Literal["Gaussian", "Eisenstein"], a: int, b: int) -> tuple[int, int]:
        """Returns the canonical associate of a point: the one with `a > 0` and `b >= 0` on the square lattice, the one with `a > 0` and `0 <= b < a` on the hexagonal, the origin for the origin."""
    @staticmethod
    def conjugate(ring: Literal["Gaussian", "Eisenstein"], a: int, b: int) -> tuple[int, int]:
        """Returns the conjugate: the mirror image in the real axis."""
    @staticmethod
    def count(ring: Literal["Gaussian", "Eisenstein"], radius: int) -> int:
        """Returns the count of points within the reach: the square or the hexagon."""
    @staticmethod
    def div_rem(ring: Literal["Gaussian", "Eisenstein"], z: tuple[int, int], w: tuple[int, int]) -> tuple[tuple[int, int], tuple[int, int]]:
        """Returns the quotient and the remainder of a point by a nonzero point: `z = q w + r` with the norm of `r` below the norm of `w`."""
    @staticmethod
    def fate(ring: Literal["Gaussian", "Eisenstein"], n: int) -> Literal["Zero", "Unit", "Ramified", "Split", "Inert", "Composite"]:
        """Returns the fate of a whole number as a prime of the ring: split, inert or ramified, unit for one, zero for zero, composite otherwise."""
    @staticmethod
    def gaussian_gcd(ring: Literal["Gaussian", "Eisenstein"], z: tuple[int, int], w: tuple[int, int]) -> tuple[int, int]:
        """Returns the greatest common divisor of two points as its canonical associate, by the nearest-point Euclidean algorithm, the origin for two origins."""
    @staticmethod
    def inert(ring: Literal["Gaussian", "Eisenstein"], p: int) -> bool:
        """Returns whether a rational prime stays prime in the ring: 3 mod 4, or 2 mod 3."""
    @staticmethod
    def mul(ring: Literal["Gaussian", "Eisenstein"], arg1: tuple[int, int], arg2: tuple[int, int]) -> tuple[int, int]:
        """Returns the product of two points."""
    @staticmethod
    def named(name: str) -> Literal["Gaussian", "Eisenstein"] | None:
        """Reads a ring from its name."""
    @staticmethod
    def nearest(ring: Literal["Gaussian", "Eisenstein"], x: float, y: float) -> tuple[int, int]:
        """Returns the point nearest a place in the plane."""
    @staticmethod
    def norm(ring: Literal["Gaussian", "Eisenstein"], a: int, b: int) -> int:
        """Returns the norm of a point: its squared length."""
    @staticmethod
    def place(ring: Literal["Gaussian", "Eisenstein"], a: int, b: int) -> tuple[float, float]:
        """Returns the place of a point in the plane, x right and y up, one unit between neighbours."""
    @staticmethod
    def ramified(ring: Literal["Gaussian", "Eisenstein"]) -> int:
        """Returns the one rational prime that ramifies: 2 or 3."""
    @staticmethod
    def reach(ring: Literal["Gaussian", "Eisenstein"], a: int, b: int) -> int:
        """Returns the reach of a point: the ring of the window it sits on, the Chebyshev distance or the hex distance."""
    @staticmethod
    def symmetry(ring: Literal["Gaussian", "Eisenstein"]) -> int:
        """Returns the order of the symmetry of the picture, the units and the mirror: 8 or 12."""
    @staticmethod
    def top(ring: Literal["Gaussian", "Eisenstein"], radius: int) -> int:
        """Returns the largest norm within the reach: 2 r^2 at the square's corner, r^2 at the hexagon's."""
    @staticmethod
    def turn(ring: Literal["Gaussian", "Eisenstein"], a: int, b: int) -> tuple[int, int]:
        """Returns the point turned anticlockwise by one unit: a quarter turn or a sixth."""
    @staticmethod
    def units(ring: Literal["Gaussian", "Eisenstein"]) -> int:
        """Returns the count of units: 4 or 6."""
    @staticmethod
    def whole(ring: Literal["Gaussian", "Eisenstein"], a: int, b: int) -> int | None:
        """Returns the whole number an associate of the point lies on, when one lies on the positive real axis."""

def classes(ring: Literal["Gaussian", "Eisenstein"], bound: int) -> list[tuple[int, int]]:
    """Lists one point per associate class of the nonzero points of norm at most the bound: canonical associates, in order of norm and then of coordinates."""

def peak(ring: Literal["Gaussian", "Eisenstein"], limit: int) -> tuple[int, int]:
    """Returns the norm from one through the limit with the most points and that count, the earliest on a tie."""

def shells(ring: Literal["Gaussian", "Eisenstein"], limit: int) -> list[int]:
    """Counts the points of every norm from zero through the limit, by enumeration: the ring weights of the lattice."""
