import numpy as np
import pytest
from mrly.core import colors, tensor

DTYPES = (np.uint8, np.uint16, np.uint32, np.int32)


def test_a_tensor_crosses_both_ways_in_every_dtype():
    for dtype in DTYPES:
        out = tensor.rot90(np.array([[1, 2], [3, 4]], dtype=dtype), 1, (0, 1))
        assert out.dtype == dtype
        assert out.shape == (2, 2)
        assert out.tolist() == [[2, 4], [1, 3]]


def test_a_tensor_out_owns_the_buffer_it_was_handed():
    out = tensor.rot90(np.zeros((3, 4), dtype=np.uint8), 1, (0, 1))
    assert out.flags.writeable
    assert out.flags.c_contiguous
    assert not isinstance(out.base, np.ndarray)


def test_a_tensor_in_refuses_a_foreign_dtype_or_a_gapped_layout():
    with pytest.raises(ValueError):
        tensor.rot90(np.zeros((2, 2), dtype=np.float64), 1, (0, 1))
    with pytest.raises(ValueError):
        tensor.rot90(np.zeros((4, 4), dtype=np.uint8)[::2], 1, (0, 1))


def test_a_rust_value_error_lands_as_a_value_error():
    with pytest.raises(ValueError):
        tensor.rot90(np.zeros((2, 2), dtype=np.uint8), 1, (0, 2))


def test_a_mutated_tensor_writes_back_into_its_array():
    grid = np.zeros((2, 2), dtype=np.uint8)
    tensor.put(grid, 3, 9)
    assert grid[1, 1] == 9


def test_a_color_crosses_as_four_channel_bytes():
    assert colors.from_hex("#ff3d40") == (255, 61, 64, 255)
    assert colors.from_hex("#008cff80") == (0, 140, 255, 128)
    with pytest.raises(ValueError):
        colors.from_hex("#nope")


def test_a_vec_of_bytes_crosses_as_bytes():
    out = tensor.bytes(np.array([[1, 2], [3, 4]], dtype=np.uint8))
    assert isinstance(out, bytes)
    assert out == b"\x01\x02\x03\x04"
