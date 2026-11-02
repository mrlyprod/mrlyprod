import json
import pathlib

from mrly.math import atoms, bang, counts, spectrum, three, two
from mrly.math.graph import Network

FIXTURES = pathlib.Path(__file__).resolve().parents[2] / "tests" / "fixtures.json"
ROWS = json.loads(FIXTURES.read_text())


def row(fn):
    return next(r for r in ROWS if r["fn"] == fn)


def test_atoms_carpet_2d():
    r = row("math::atoms::carpet_2d")
    atom = atoms.carpet_2d(r["in"]["n"])
    assert list(atom.shape) == r["out"]["shape"]
    assert atom.flatten().tolist() == r["out"]["data"]


def test_two_carpet():
    r = row("math::two::carpet")
    cell = two.carpet(r["in"]["number"], r["in"]["level"])
    assert json.loads(two.to_json(cell)) == r["out"]


def test_two_census():
    r = row("math::two::census::census")
    inner = r["in"]["cell"]["in"]
    assert two.census(two.carpet(inner["number"], inner["level"])) == r["out"]


def test_counts_fill():
    r = row("math::counts::fill")
    i = r["in"]
    filled = counts.fill(
        int(i["code"]), i["number"], i["dimension"], i["level"], i["base"]
    )
    assert filled == int(r["out"])


def test_bang():
    r = row("math::bang::bang")
    assert bang.bang(r["in"]["dimension"]).distinct() == r["out"]


def test_three_census():
    r = row("math::three::census::census")
    inner = r["in"]["cell"]["in"]
    solid = three.census(three.carpet(inner["number"], inner["level"]))
    assert solid["surface"] == int(r["out"])


def test_laplacian_spectrum():
    r = row("math::spectrum::laplacian_spectrum")
    i = r["in"]["network"]
    network = Network(i["dim"])
    for position in i["nodes"]:
        network.add_node(position)
    for parent, child, radius in i["branches"]:
        network.add_branch(parent, child, radius)
    spread = spectrum.laplacian_spectrum(network, r["in"]["normalised"])
    assert [round(value, 12) for value in spread] == r["out"]
