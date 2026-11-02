import numpy as np
import pytest
from mrly.core import Rng, paint
from mrly.math import cell, six, three, two


def draw(rng):
    return [rng.range(0, 100) for _ in range(10)]


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


def test_one_seed_replays_one_stream_and_passes_into_rust():
    assert draw(Rng(2026)) == draw(Rng(2026))
    assert draw(Rng(2026)) != draw(Rng(2027))
    drawn = two.noise(3, 1, 0.5, Rng(2026))["types"]
    assert drawn.tolist() == two.noise(3, 1, 0.5, Rng(2026))["types"].tolist()
