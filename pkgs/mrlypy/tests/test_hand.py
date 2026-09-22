import numpy as np
import pytest

from mrlypy import core, gen, life
from mrlypy.core import Rng, colors, paint, tensor
from mrlypy.math import cell, six, three, two
from mrlypy.num import factor, series

DTYPES = (np.uint8, np.uint16, np.uint32, np.int32)


def draw(rng):
    return [rng.range(0, 100) for _ in range(10)]


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


def test_a_rust_overflow_lands_as_an_overflow_error():
    assert series.bernoulli(3) == [(1, 1), (-1, 2), (1, 6)]
    with pytest.raises(OverflowError):
        series.bernoulli(33)


def test_a_mutated_tensor_writes_back_into_its_array():
    grid = np.zeros((2, 2), dtype=np.uint8)
    tensor.put(grid, 3, 9)
    assert grid[1, 1] == 9


def test_a_cell_crosses_out_as_a_dict_of_arrays():
    c = two.carpet(3, 2)
    assert set(c) == {"types", "colors", "tags"}
    assert c["types"].shape == (9, 9)
    assert c["types"].dtype == np.uint8
    assert int(c["types"].sum()) == 64
    assert c["colors"] is None
    assert c["tags"] is None


def test_a_three_dimensional_cell_keeps_depth_height_width():
    assert three.carpet(3, 1)["types"].shape == (3, 3, 3)


def test_a_cell_crosses_in_and_a_serde_struct_crosses_out():
    assert two.census(two.carpet(3, 1)) == {
        "fills": 8,
        "voids": 1,
        "perimeter": 16,
        "vertices": 16,
        "edges": 24,
        "euler": 0,
    }


def test_cell_colors_cross_out_as_rgba_and_back_in():
    painted = cell.paint(two.carpet(3, 1))
    assert painted["colors"].shape == (3, 3, 4)
    assert painted["colors"].dtype == np.uint8
    assert painted["colors"][1, 1].tolist() == [255, 255, 255, 255]
    assert two.census(painted)["fills"] == 8


def test_paint_takes_its_optional_mapping_mode_and_stream():
    c = two.carpet(3, 1)
    custom = {0: [(255, 61, 64, 255)], 1: [(0, 140, 255, 255)]}
    painted = cell.paint(c, custom, "Type")
    assert painted["colors"][1, 1].tolist() == [255, 61, 64, 255]
    assert painted["colors"][0, 0].tolist() == [0, 140, 255, 255]
    assert cell.paint(c, custom, "Random", Rng(2026)) is not None
    with pytest.raises(ValueError):
        cell.paint(c, custom, "Random")


def test_a_mutated_cell_writes_back_into_its_dict():
    c = two.carpet(3, 1)
    paint.tag(c, "Layers", "Fill", None)
    assert c["tags"] is not None


def test_a_cell_dict_that_is_not_a_2d_cell_is_a_value_error():
    with pytest.raises(ValueError):
        two.census({"types": np.zeros((2, 2, 2), dtype=np.uint8)})
    with pytest.raises(ValueError):
        two.census({"tags": np.zeros((2, 2), dtype=np.uint8)})


def test_a_tag_layer_crosses_in_beside_its_types():
    c = two.carpet(3, 1)
    c["tags"] = np.ones((3, 3), dtype=np.uint16)
    assert two.census(c)["fills"] == 8


def test_a_const_n_function_dispatches_on_the_rank():
    assert cell.models.width(two.carpet(3, 1)) == 3
    assert cell.models.width(three.carpet(3, 1)) == 3
    assert cell.models.rotate(three.carpet(3, 1), 1, (0, 1))["types"].shape == (3, 3, 3)
    with pytest.raises(ValueError):
        cell.models.rotate(three.carpet(3, 1), 1)
    with pytest.raises(ValueError):
        cell.models.depth(two.carpet(3, 1))


def test_a_hexagonal_cell_crosses_by_its_four_fields():
    c = six.cut_design(23, 3, 1, 2)
    assert set(c) == {"cell", "projection", "orientation", "start"}
    assert c["projection"] == "Cut"
    assert c["orientation"] == "Horizontal"
    assert c["cell"]["types"].ndim == 2
    assert six.census(c, True)["triangles"] > 0


def test_a_code_crosses_as_a_plain_int():
    assert factor.gcd(1071, 462) == 21
    assert factor.gcd(2**127, 2**100) == 2**100


def test_a_color_crosses_as_four_channel_bytes():
    assert colors.from_hex("#ff3d40") == (255, 61, 64, 255)
    assert colors.from_hex("#008cff80") == (0, 140, 255, 128)
    with pytest.raises(ValueError):
        colors.from_hex("#nope")


def test_pixels_cross_out_as_an_n_by_4_array():
    width, height, pixels = core.unpng(gen.background(1, 2, 2))
    assert pixels.shape == (width * height, 4)
    assert pixels.dtype == np.uint8
    assert core.codec.png(pixels, width, height, 1) == core.codec.png(pixels.tolist(), width, height, 1)


def test_a_vec_of_bytes_crosses_as_bytes():
    png = gen.background(1, 2, 2)
    assert isinstance(png, bytes)
    assert png[:8] == b"\x89PNG\r\n\x1a\n"


def test_a_class_holds_the_rust_value_and_round_trips_plain_data():
    tile = gen.Tile("Fractal")
    assert tile.group == "Fractal"
    assert gen.Tile.from_dict(tile.to_dict()).to_dict() == tile.to_dict()
    assert gen.Tile.new("Fractal").to_dict() == tile.to_dict()


def test_a_python_keyword_grows_a_trailing_underscore():
    assert life.lambda_(30) == 0.5


def test_one_seed_replays_one_stream_and_passes_into_rust():
    assert draw(Rng(2026)) == draw(Rng(2026))
    assert draw(Rng(2026)) != draw(Rng(2027))
    drawn = two.noise(3, 1, 0.5, Rng(2026))["types"]
    assert drawn.tolist() == two.noise(3, 1, 0.5, Rng(2026))["types"].tolist()
