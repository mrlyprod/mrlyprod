import mrlymath as mp
import pytest

def test_2d_json_round_trip():
    cell = mp.two.carpet(3, 2)
    back = mp.two.from_json(cell.to_json())
    assert back.to_lists() == cell.to_lists()
    assert back.to_json() == cell.to_json()

def test_2d_lists_round_trip():
    cell = mp.two.net(3, 2)
    assert mp.two.from_lists(cell.to_lists()).to_lists() == cell.to_lists()

def test_2d_strings_round_trip():
    cell = mp.two.void(4, 1)
    assert mp.two.from_strings(cell.to_strings()).to_lists() == cell.to_lists()

def test_painted_colors_survive():
    palette = {0: [[255, 61, 64], [0, 140, 255]], 1: [[255, 255, 255]]}
    cell = mp.two.carpet(3, 2).paint(palette, "random")
    text = cell.to_json()
    assert '"colors"' in text
    assert mp.two.from_json(text).to_json() == text

def test_3d_json_round_trip():
    cell = mp.three.carpet(3, 1)
    back = mp.three.from_json(cell.to_json())
    assert back.to_lists() == cell.to_lists()

def test_3d_lists_round_trip():
    cell = mp.three.xtree(3, 1)
    assert mp.three.from_lists(cell.to_lists()).to_lists() == cell.to_lists()

def test_ragged_lists_raise():
    with pytest.raises(ValueError):
        mp.two.from_lists([[0, 1], [0]])

def test_empty_lists_raise():
    with pytest.raises(ValueError):
        mp.two.from_lists([])

def test_bad_mode_raises():
    with pytest.raises(ValueError):
        mp.two.carpet(3, 1).paint({0: [[0, 0, 0]]}, "nope")
