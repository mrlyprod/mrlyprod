import hashlib

import numpy as np

from mrlypy.core import Colorizer, codec, colors, ramp, tensor


def test_tensor_rot90(row):
    r = row("core::Tensor::rot90")
    grid = np.array(r["in"]["data"], dtype=np.uint8).reshape(r["in"]["shape"])
    out = tensor.rot90(grid, r["in"]["k"], tuple(r["in"]["axes"]))
    assert list(out.shape) == r["out"]["shape"]
    assert out.flatten().tolist() == r["out"]["data"]


def test_color_from_hex(row):
    r = row("core::Color::from_hex")
    color = colors.from_hex(r["in"]["hex"])
    assert list(color) == r["out"]["rgba"]
    assert colors.to_hex(color) == r["out"]["hex"]


def test_png(row):
    r = row("core::png")
    i = r["in"]
    png = codec.png(i["colors"], i["width"], i["height"], i["scale"])
    assert hashlib.sha256(png).hexdigest() == r["out"]


def test_colorizer_color(row):
    r = row("core::Colorizer::color")
    i = r["in"]
    stops = [colors.from_hex(h) for h in i["ramp"]]
    colorizer = Colorizer.gradient_bins(colors.from_hex(i["background"]), stops, i["shades"])
    swatches = [colors.to_hex(ramp.color(colorizer, v, i["max"])) for v in i["values"]]
    assert swatches == r["out"]
