from typing import Any, Literal

from numpy.typing import NDArray

def hex(bytes: bytes) -> str:
    """Returns the sha256 digest of `bytes` as 64 lowercase hex digits."""

def sha256(bytes: bytes) -> bytes:
    """Returns the 32-byte sha256 digest of `bytes`."""

def short(bytes: bytes) -> str:
    """Returns the first eight hex digits of the sha256 digest of `bytes`, the id a name prints."""
