import numpy as np
import pytest
from mrly import core, tile
from mrly.math import sumset


def test_an_image_crosses_out_as_one_h_by_w_by_4_array():
    image = tile.background(7, 2, 3)
    assert image.shape == (27, 18, 4)
    assert image.dtype == np.uint8
    assert image.flags.c_contiguous
    assert image.flags.writeable


def test_an_image_crosses_in_and_back_with_its_shape():
    image = np.arange(24, dtype=np.uint8).reshape(2, 3, 4)
    grown = core.image.scale(image, 2)
    assert grown.shape == (4, 6, 4)
    assert np.array_equal(grown, image.repeat(2, axis=0).repeat(2, axis=1))
    file = tile.variation.File(1, 1)
    assert file.image is None
    file.image = image
    assert np.array_equal(file.image, image)


def test_an_image_that_is_not_h_by_w_by_4_bytes_is_refused():
    for bad in (
        np.zeros((2, 3, 3), dtype=np.uint8),
        np.zeros((6, 4), dtype=np.uint8),
        np.zeros((2, 3, 4), dtype=np.float32),
        np.zeros((2, 6, 4), dtype=np.uint8)[:, ::2],
        [[[0, 0, 0, 255]]],
    ):
        with pytest.raises(ValueError):
            core.image.scale(bad, 2)
        with pytest.raises(ValueError):
            tile.variation.File(1, 1).image = bad
    with pytest.raises(ValueError):
        core.image.new(2, 2, np.zeros((1, 4), dtype=np.uint8))


def test_pixels_cross_out_as_an_n_by_4_array():
    image = tile.background(1, 2, 2)
    height, width = image.shape[:2]
    pixels = core.resample(image, width, height, 2 * width, 2 * height, "Nearest")
    assert pixels.shape == (4 * width * height, 4)
    assert pixels.dtype == np.uint8
    listed = image.reshape(-1, 4).tolist()
    assert np.array_equal(
        pixels, core.resample(listed, width, height, 2 * width, 2 * height, "Nearest")
    )


def test_a_class_holds_the_rust_value_and_round_trips_plain_data():
    made = tile.Tile("Fractal")
    assert made.group == "Fractal"
    assert tile.Tile.from_dict(made.to_dict()).to_dict() == made.to_dict()
    assert tile.Tile.new("Fractal").to_dict() == made.to_dict()


def test_bad_plain_data_is_a_value_error():
    with pytest.raises(ValueError, match="missing field"):
        tile.Tile.from_dict({"group": "Fractal"})
    with pytest.raises(ValueError):
        sumset.Pair.from_dict({"three": -1, "four": 1})
