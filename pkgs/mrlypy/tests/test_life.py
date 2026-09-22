import json

import numpy as np

from mrlypy import life
from mrlypy.life import Config, Counts, Source
from mrlypy.math import two

FATES = {"Dead": "dead", "Alive": "alive", "Loop": "loop", "Timeout": "timeout"}


def cell(spec):
    return {"types": np.array(spec["data"], dtype=np.uint8).reshape(spec["shape"])}


def run(spec):
    c = spec["config"]
    config = Config(life.moore(), Counts.list(c["birth"]), Counts.list(c["survive"]))
    assert config.boundary == c["boundary"]
    assert config.max_generations == c["max_generations"]
    assert config.grid_size == c["grid_size"]
    assert config.padding == c["padding"]
    return life.animate(cell(spec["seed"]), config)


def test_history(row):
    r = row("life::history")
    i = r["in"]
    space = life.history(i["row"], i["rule"], i["steps"], i["wrap"])
    assert list(space.shape) == r["out"]["shape"]
    assert space.flatten().tolist() == r["out"]["data"]


def test_animate(row):
    r = row("life::animate::animate")
    life_run = run(r["in"])
    assert FATES[life_run.fate] == r["out"]["fate"]
    assert life_run.count == r["out"]["count"]
    assert life_run.loop_length == r["out"]["loop_length"]
    assert json.loads(two.to_json(life_run.last())) == r["out"]["last"]


def test_entropy(row):
    r = row("life::entropy")
    inner = r["in"]["grid"]["in"]
    assert life.entropy(two.carpet(inner["number"], inner["level"])) == r["out"]


def test_churn(row):
    r = row("life::churn")
    blank, carpet = r["in"]["grids"]
    grids = [
        {"types": np.zeros(blank["zeros"]["shape"], dtype=np.uint8)},
        two.carpet(carpet["in"]["number"], carpet["in"]["level"]),
    ]
    assert round(life.churn(grids), 12) == r["out"]


def test_counts(row):
    r = row("life::counts")
    i = r["in"]
    seq = Source.parse(i["seq"])
    assert life.counts(seq, i["max_neighbors"], i["include_zeros"], i["include_ones"]) == r["out"]


def test_heatmap(row):
    r = row("life::heatmap")
    life_run = run(r["in"]["grids"]["in"])
    frames = life.heatmap(life_run.grids, r["in"]["scale"])
    assert [list(frame) for frame in frames] == r["out"]
