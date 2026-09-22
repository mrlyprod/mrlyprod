import hashlib
import json

from mrlypy import font


def test_glyph(row):
    r = row("font::glyph")
    assert font.glyph(r["in"]["c"]).rows == r["out"]


def test_raster(row):
    r = row("font::raster::raster")
    assert [list(line) for line in font.raster(r["in"]["text"])] == r["out"]


def test_path(row):
    r = row("font::path")
    assert [list(step) for step in font.path(r["in"]["c"])] == r["out"]


def test_animate(row):
    r = row("font::animate::animate")
    anim = font.animate(r["in"]["text"], r["in"]["pad"])
    assert anim["rows"] == r["out"]["rows"]
    assert anim["cols"] == r["out"]["cols"]
    assert anim["fps"] == r["out"]["fps"]
    assert len(anim["frames"]) == r["out"]["frames"]
    assert anim["frames"][-1] == r["out"]["last"]


def test_map(row):
    r = row("font::map")
    book = font.map()
    text = json.dumps(book, separators=(",", ":"), sort_keys=True, ensure_ascii=False)
    assert hashlib.sha256(text.encode()).hexdigest() == r["out"]["sha256"]
    assert len(book) == r["out"]["len"]
