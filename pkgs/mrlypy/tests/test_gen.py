import hashlib
import json

from mrlypy import gen
from mrlypy.core import Rng
from mrlypy.gen import build, name, variation
from mrlypy.math import two

DEFAULT_TILE = {
    "groups": ["General", "Fractal", "Magic", "Special", "Mosaic"],
    "catalog": "Classics",
    "min_size": 3,
    "max_size": 9,
    "parity": "Odds",
    "invert": None,
}

DEFAULT_VARIATION = {
    "tile": DEFAULT_TILE,
    "paint": {"editions": None, "primaries": None, "target": None},
    "files": [(1, 1), (3, 3), (5, 5)],
}


def test_name_recipe(row):
    r = row("gen::name::Tile::recipe")
    recipe = name.Tile.from_json(r["in"]["name"]).recipe()
    assert recipe.to_dict() == r["out"]["recipe"]
    folded = name.Tile.of(recipe)
    assert folded.to_json() == r["out"]["json"]
    assert folded.to_id() == r["out"]["id"]


def test_draw_create(row):
    r = row("gen::draw::create")
    tile = build.create_2d(DEFAULT_TILE, Rng(r["in"]["seed"]))
    assert name.Tile.of(tile).to_json() == r["out"]


def test_build_2d(row):
    r = row("gen::build::build_2d")
    recipe = name.Tile.from_json(r["in"]["tile"]["in"]["name"]).recipe()
    assert json.loads(two.to_json(build.build_2d(recipe))) == r["out"]


def test_variation_create(row):
    r = row("gen::variation::create")
    made = variation.create(DEFAULT_VARIATION, Rng(r["in"]["seed"]))
    assert made.key == r["out"]["key"]
    assert str(made.seed) == r["out"]["seed"]
    assert made.edition == r["out"]["edition"]
    assert name.Tile.of(made.tile).to_json() == r["out"]["tile"]


def test_background(row):
    r = row("gen::background")
    i = r["in"]
    png = gen.background(i["seed"], i["width"], i["height"])
    assert hashlib.sha256(png).hexdigest() == r["out"]
