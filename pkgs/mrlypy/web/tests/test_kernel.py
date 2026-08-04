import base64

import mrlyweb


def focused(env):
    return env["view"]["state"]


def snake():
    os = mrlyweb.boot()
    mrlyweb.call(os, {"verb": "nav.open", "args": {"app": "snake"}})
    mrlyweb.call(os, {"verb": "snake.reset", "args": {"seed": 7}})
    return os


def test_boot_reads_the_envelope():
    os = mrlyweb.boot()
    env = mrlyweb.read(os)
    assert env["tick"] == 0
    assert env["route"]["app"] == "menu"
    assert env["view"]["app"] == "menu"


def test_call_takes_dict_or_str():
    os = mrlyweb.boot()
    a = mrlyweb.call(os, {"verb": "nav.open", "args": {"app": "calculator"}})
    assert a["route"]["app"] == "calculator"
    b = mrlyweb.call(os, '{"verb": "calculator.digit", "args": {"d": 4}}')
    assert focused(b)["display"] == "4"


def test_list_covers_the_surface():
    os = mrlyweb.boot()
    world = mrlyweb.list(os)
    routes = [a["route"] for a in world["apps"]]
    assert "snake" in routes
    assert world["version"]
    assert world["nav"][0]["verb"] == "nav.open"
    assert len(world["nav"]) == 1


def test_list_prunes_with_a_shape():
    os = mrlyweb.boot()
    assert sorted(mrlyweb.list(os, {"version": 1}).keys()) == ["version"]


def test_snake_round():
    os = mrlyweb.boot()
    mrlyweb.call(os, {"verb": "nav.open", "args": {"app": "snake"}})
    env = mrlyweb.call(os, {"verb": "snake.reset", "args": {"seed": 7}})
    assert focused(env)["seed"] == 7
    env = mrlyweb.call(os, {"verb": "snake.turn", "args": {"dir": "left"}})
    assert focused(env)["dir"] == "left"
    env = mrlyweb.call(os, {"verb": "snake.step", "args": {"n": 3}})
    assert focused(env)["steps"] == 3
    assert not focused(env)["over"]


def test_read_leaves_focus_alone():
    os = mrlyweb.boot()
    view = mrlyweb.read(os, "colors")
    assert view["app"] == "colors"
    assert view["state"]
    assert mrlyweb.read(os, "")["route"]["app"] == "menu"


def test_read_drills_and_prunes():
    os = snake()
    assert mrlyweb.read(os, "snake/seed") == 7
    assert mrlyweb.read(os, "snake", {"seed": 1})["state"] == {"seed": 7}
    assert mrlyweb.read(os, "snake/frame", {"width": 1}) == {"width": 48}


def test_missing_paths_read_none():
    os = snake()
    assert mrlyweb.read(os, "nowhere") is None
    assert mrlyweb.read(os, "snake/nowhere") is None


def test_snake_frame_is_a_48_grid():
    os = snake()
    frame = mrlyweb.read(os, "snake/frame")
    assert (frame["width"], frame["height"]) == (48, 48)
    assert len(frame["rows"]) == 48
    assert all(len(row) == 48 for row in frame["rows"])
    assert frame["palette"]
    assert max(max(row) for row in frame["rows"]) < len(frame["palette"])


def test_frameless_app_has_no_frame():
    os = mrlyweb.boot()
    mrlyweb.call(os, {"verb": "nav.open", "args": {"app": "calculator"}})
    assert mrlyweb.read(os, "calculator/frame") is None


def test_replay_rebuilds_the_same_frame():
    first = mrlyweb.read(snake(), "snake/frame")
    second = mrlyweb.read(snake(), "snake/frame")
    assert first == second


def test_geometry_is_base64_f32():
    os = mrlyweb.boot()
    buffer = mrlyweb.read(os, "solids/geometry")
    assert buffer["dtype"] == "f32"
    assert len(base64.b64decode(buffer["data"])) % 4 == 0
    assert mrlyweb.read(os, "calculator/geometry") is None
