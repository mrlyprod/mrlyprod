import base64

import mrlyweb

def focused(env):
    return env["view"]["state"]

def snake():
    os = mrlyweb.boot("full")
    mrlyweb.call(os, {"verb": "nav.open", "args": {"app": "snake"}})
    mrlyweb.call(os, {"verb": "snake.reset", "args": {"seed": 7}})
    return os

def test_boot_reads_the_envelope():
    os = mrlyweb.boot("full")
    env = mrlyweb.read(os)
    assert env["tick"] == 0
    assert env["route"]["app"] == "menu"
    assert env["view"]["app"] == "menu"

def test_call_takes_dict_or_str():
    os = mrlyweb.boot("full")
    a = mrlyweb.call(os, {"verb": "nav.open", "args": {"app": "calculator"}})
    assert a["route"]["app"] == "calculator"
    b = mrlyweb.call(os, '{"verb": "calculator.digit", "args": {"d": 4}}')
    assert focused(b)["display"] == "4"

def test_list_covers_the_surface():
    os = mrlyweb.boot("full")
    world = mrlyweb.list(os)
    routes = [a["route"] for a in world["apps"]]
    assert "snake" in routes
    assert world["version"]
    assert world["nav"][0]["verb"] == "nav.open"
    assert len(world["nav"]) == 1

def test_list_prunes_with_a_shape():
    os = mrlyweb.boot("full")
    assert sorted(mrlyweb.list(os, {"version": 1}).keys()) == ["version"]

def test_snake_round():
    os = mrlyweb.boot("full")
    mrlyweb.call(os, {"verb": "nav.open", "args": {"app": "snake"}})
    env = mrlyweb.call(os, {"verb": "snake.reset", "args": {"seed": 7}})
    assert focused(env)["seed"] == 7
    env = mrlyweb.call(os, {"verb": "snake.turn", "args": {"dir": "left"}})
    assert focused(env)["dir"] == "left"
    env = mrlyweb.call(os, {"verb": "snake.step", "args": {"n": 3}})
    assert focused(env)["steps"] == 3
    assert not focused(env)["over"]

def test_read_leaves_focus_alone():
    os = mrlyweb.boot("full")
    view = mrlyweb.read(os, "colors")
    assert view["app"] == "colors"
    assert view["state"]
    assert mrlyweb.read(os, "")["route"]["app"] == "menu"

def test_read_drills_and_prunes():
    os = snake()
    assert mrlyweb.read(os, "snake/seed") == 7
    assert mrlyweb.read(os, "snake", {"seed": 1})["state"] == {"seed": 7}
    assert mrlyweb.read(os, "snake/cells", {"skin": 1}) == {"skin": "tiles"}

def test_missing_paths_read_none():
    os = snake()
    assert mrlyweb.read(os, "nowhere") is None
    assert mrlyweb.read(os, "snake/nowhere") is None

def test_snake_cells_are_a_16_grid():
    os = snake()
    cells = mrlyweb.read(os, "snake/cells")
    ids = cells["ids"]
    assert len(ids) == 16
    assert all(len(row) == 16 for row in ids)
    assert cells["skin"] == "tiles"
    assert len(cells["pens"]) == 3
    assert all(pen.startswith("#") for pen in cells["pens"])
    assert max(max(row) for row in ids) < 4

def test_frameless_app_has_no_frame():
    os = mrlyweb.boot("full")
    mrlyweb.call(os, {"verb": "nav.open", "args": {"app": "calculator"}})
    assert mrlyweb.read(os, "calculator/frame") is None
    assert mrlyweb.read(snake(), "snake/frame") is None

def test_replay_rebuilds_the_same_cells():
    first = mrlyweb.read(snake(), "snake/cells")
    second = mrlyweb.read(snake(), "snake/cells")
    assert first == second

def test_geometry_is_base64_f32():
    os = mrlyweb.boot("full")
    buffer = mrlyweb.read(os, "solids/geometry")
    assert buffer["dtype"] == "f32"
    assert len(base64.b64decode(buffer["data"])) % 4 == 0
    assert mrlyweb.read(os, "calculator/geometry") is None
