import mrlyweb
import numpy as np
import pytest


def boot_snake():
    os = mrlyweb.boot()
    mrlyweb.act(os, {"verb": "nav.open", "args": {"app": "snake"}})
    mrlyweb.act(os, {"verb": "snake.reset", "args": {"seed": 7}})
    return os


def test_face_tuple_is_sane():
    os = mrlyweb.boot()
    w, h, buf = mrlyweb.face(os, "settings")
    assert w == 320
    assert 160 <= h <= 512
    assert isinstance(buf, bytes)
    assert len(buf) == w * h * 4


def test_face_png_has_the_magic():
    os = mrlyweb.boot()
    png = mrlyweb.face_png(os)
    assert png[:8] == b"\x89PNG\r\n\x1a\n"


def test_snake_canvas_is_48x48():
    os = boot_snake()
    w, h, buf = mrlyweb.capture(os)
    assert (w, h) == (48, 48)
    assert len(buf) == w * h * 4
    png = mrlyweb.capture_png(os)
    assert png[:8] == b"\x89PNG\r\n\x1a\n"


def test_frameless_capture_raises_but_faces():
    os = mrlyweb.boot()
    mrlyweb.act(os, {"verb": "nav.open", "args": {"app": "calculator"}})
    with pytest.raises(ValueError, match="nothing to shoot here"):
        mrlyweb.capture(os)
    w, h, buf = mrlyweb.face(os)
    assert len(buf) == w * h * 4


def test_numpy_roundtrip():
    os = boot_snake()
    w, h, buf = mrlyweb.capture(os)
    pixels = np.frombuffer(buf, np.uint8).reshape(h, w, 4)
    assert pixels.shape == (48, 48, 4)
    assert int(pixels[..., 3].min()) == 255


def test_peek_leaves_focus_alone():
    os = mrlyweb.boot()
    view = mrlyweb.peek(os, "colors")
    assert view["app"] == "colors"
    assert view["state"]
    assert mrlyweb.frame(os)["route"]["app"] == "menu"
