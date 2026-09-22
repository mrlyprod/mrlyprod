import numpy as np
import pytest

import mrlypy

DTYPES = (np.uint8, np.uint16, np.uint32, np.int32)


def draw(rng):
    return [rng.range(0, 100) for _ in range(10)]


def test_a_tensor_crosses_both_ways_in_every_dtype():
    for dtype in DTYPES:
        out = mrlypy.rot90(np.array([[1, 2], [3, 4]], dtype=dtype), 1, (0, 1))
        assert out.dtype == dtype
        assert out.shape == (2, 2)
        assert out.tolist() == [[2, 4], [1, 3]]


def test_a_tensor_out_owns_the_buffer_it_was_handed():
    out = mrlypy.rot90(np.zeros((3, 4), dtype=np.uint8), 1, (0, 1))
    assert out.flags.writeable
    assert out.flags.c_contiguous
    assert not isinstance(out.base, np.ndarray)


def test_a_tensor_in_refuses_a_foreign_dtype_or_a_gapped_layout():
    with pytest.raises(ValueError):
        mrlypy.rot90(np.zeros((2, 2), dtype=np.float64), 1, (0, 1))
    with pytest.raises(ValueError):
        mrlypy.rot90(np.zeros((4, 4), dtype=np.uint8)[::2], 1, (0, 1))


def test_a_rust_value_error_lands_as_a_value_error():
    with pytest.raises(ValueError):
        mrlypy.rot90(np.zeros((2, 2), dtype=np.uint8), 1, (0, 2))


def test_a_rust_overflow_lands_as_an_overflow_error():
    assert mrlypy.bernoulli(3) == [(1, 1), (-1, 2), (1, 6)]
    with pytest.raises(OverflowError):
        mrlypy.bernoulli(33)


def test_a_cell_crosses_out_as_a_dict_of_arrays():
    cell = mrlypy.carpet(3, 2)
    assert set(cell) == {"types", "colors", "tags"}
    assert cell["types"].shape == (9, 9)
    assert cell["types"].dtype == np.uint8
    assert int(cell["types"].sum()) == 64
    assert cell["colors"] is None
    assert cell["tags"] is None


def test_a_three_dimensional_cell_keeps_depth_height_width():
    assert mrlypy.carpet_3d(3, 1)["types"].shape == (3, 3, 3)


def test_a_cell_crosses_in_and_a_serde_struct_crosses_out():
    assert mrlypy.census(mrlypy.carpet(3, 1)) == {
        "fills": 8,
        "voids": 1,
        "perimeter": 16,
        "vertices": 16,
        "edges": 24,
        "euler": 0,
    }


def test_cell_colors_cross_out_as_rgba_and_back_in():
    painted = mrlypy.paint(mrlypy.carpet(3, 1))
    assert painted["colors"].shape == (3, 3, 4)
    assert painted["colors"].dtype == np.uint8
    assert painted["colors"][1, 1].tolist() == [255, 255, 255, 255]
    assert mrlypy.census(painted)["fills"] == 8


def test_paint_takes_its_optional_mapping_mode_and_stream():
    cell = mrlypy.carpet(3, 1)
    custom = {0: ["#ff3d40"], 1: ["#008cff"]}
    painted = mrlypy.paint(cell, custom, "Type")
    assert painted["colors"][1, 1].tolist() == [255, 61, 64, 255]
    assert painted["colors"][0, 0].tolist() == [0, 140, 255, 255]
    assert mrlypy.paint(cell, custom, "Random", mrlypy.Rng(2026)) is not None
    with pytest.raises(ValueError):
        mrlypy.paint(cell, custom, "Random")


def test_a_cell_dict_that_is_not_a_2d_cell_is_a_value_error():
    with pytest.raises(ValueError):
        mrlypy.census({"types": np.zeros((2, 2, 2), dtype=np.uint8)})
    with pytest.raises(ValueError):
        mrlypy.census({"tags": np.zeros((2, 2), dtype=np.uint8)})


def test_a_tag_layer_crosses_in_beside_its_types():
    cell = mrlypy.carpet(3, 1)
    cell["tags"] = np.ones((3, 3), dtype=np.uint16)
    assert mrlypy.census(cell)["fills"] == 8


def test_a_hexagonal_cell_crosses_by_its_four_fields():
    cell = mrlypy.cut_design(23, 3, 1, 2)
    assert set(cell) == {"cell", "projection", "orientation", "start"}
    assert cell["projection"] == "Cut"
    assert cell["orientation"] == "Horizontal"
    assert cell["cell"]["types"].ndim == 2
    assert mrlypy.six_census(cell, True)["triangles"] > 0


def test_a_code_crosses_as_a_plain_int():
    assert mrlypy.gcd(1071, 462) == 21
    assert mrlypy.gcd(2**127, 2**100) == 2**100


def test_a_color_crosses_as_four_channel_bytes():
    assert mrlypy.from_hex("#ff3d40") == (255, 61, 64, 255)
    assert mrlypy.from_hex("#008cff80") == (0, 140, 255, 128)
    with pytest.raises(ValueError):
        mrlypy.from_hex("#nope")


def test_a_vec_of_bytes_crosses_as_bytes():
    png = mrlypy.background(1, 2, 2)
    assert isinstance(png, bytes)
    assert png[:8] == b"\x89PNG\r\n\x1a\n"


def test_one_seed_replays_one_stream_and_passes_into_rust():
    assert draw(mrlypy.Rng(2026)) == draw(mrlypy.Rng(2026))
    assert draw(mrlypy.Rng(2026)) != draw(mrlypy.Rng(2027))
    drawn = mrlypy.noise(3, 1, 0.5, mrlypy.Rng(2026))["types"]
    assert drawn.tolist() == mrlypy.noise(3, 1, 0.5, mrlypy.Rng(2026))["types"].tolist()
