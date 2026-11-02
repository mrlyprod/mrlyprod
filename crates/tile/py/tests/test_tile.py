import hashlib
import json
import pathlib

from mrly import tile
from mrly.core import Rng
from mrly.math import two
from mrly.tile import build, name, variation

FIXTURES = pathlib.Path(__file__).resolve().parents[2] / "tests" / "fixtures.json"
ROWS = json.loads(FIXTURES.read_text())


def row(fn):
    return next(r for r in ROWS if r["fn"] == fn)


def pixels(image):
    return {
        "shape": list(image.shape[:2]),
        "sha256": hashlib.sha256(image.tobytes()).hexdigest(),
    }


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


def test_name_recipe():
    r = row("tile::name::Tile::recipe")
    recipe = name.Tile.from_json(r["in"]["name"]).recipe()
    assert recipe.to_dict() == r["out"]["recipe"]
    folded = name.Tile.of(recipe)
    assert folded.to_json() == r["out"]["json"]
    assert folded.to_id() == r["out"]["id"]


def test_draw_create():
    r = row("tile::draw::create")
    made = build.create_2d(DEFAULT_TILE, Rng(r["in"]["seed"]))
    assert name.Tile.of(made).to_json() == r["out"]


def test_build_2d():
    r = row("tile::build::build_2d")
    recipe = name.Tile.from_json(r["in"]["tile"]["in"]["name"]).recipe()
    assert json.loads(two.to_json(build.build_2d(recipe))) == r["out"]


def test_variation_create():
    r = row("tile::variation::create")
    made = variation.create(DEFAULT_VARIATION, Rng(r["in"]["seed"]))
    assert made.key == r["out"]["key"]
    assert str(made.seed) == r["out"]["seed"]
    assert made.edition == r["out"]["edition"]
    assert name.Tile.of(made.tile).to_json() == r["out"]["tile"]


def test_background():
    r = row("tile::background")
    i = r["in"]
    assert pixels(tile.background(i["seed"], i["width"], i["height"])) == r["out"]
