import hashlib
import json
import pathlib

import numpy as np
from mrly.core import Colorizer, colors, image, ramp, tensor

FIXTURES = pathlib.Path(__file__).resolve().parents[2] / "tests" / "fixtures.json"
ROWS = json.loads(FIXTURES.read_text())


def row(fn):
    return next(r for r in ROWS if r["fn"] == fn)


def pixels(image):
    return {
        "shape": list(image.shape[:2]),
        "sha256": hashlib.sha256(image.tobytes()).hexdigest(),
    }


def test_tensor_rot90():
    r = row("core::Tensor::rot90")
    grid = np.array(r["in"]["data"], dtype=np.uint8).reshape(r["in"]["shape"])
    out = tensor.rot90(grid, r["in"]["k"], tuple(r["in"]["axes"]))
    assert list(out.shape) == r["out"]["shape"]
    assert out.flatten().tolist() == r["out"]["data"]


def test_color_from_hex():
    r = row("core::Color::from_hex")
    color = colors.from_hex(r["in"]["hex"])
    assert list(color) == r["out"]["rgba"]
    assert colors.to_hex(color) == r["out"]["hex"]


def test_image_scale():
    r = row("core::Image::scale")
    i = r["in"]
    corners = np.array(i["colors"], dtype=np.uint8).reshape(i["height"], i["width"], 4)
    assert pixels(image.scale(corners, i["scale"])) == r["out"]


def test_colorizer_color():
    r = row("core::Colorizer::color")
    i = r["in"]
    stops = [colors.from_hex(h) for h in i["ramp"]]
    colorizer = Colorizer.gradient_bins(
        colors.from_hex(i["background"]), stops, i["shades"]
    )
    swatches = [colors.to_hex(ramp.color(colorizer, v, i["max"])) for v in i["values"]]
    assert swatches == r["out"]
