import mrlypy


def focused(obs):
    return obs["view"]["state"]


def test_boot_frames():
    os = mrlypy.boot()
    obs = mrlypy.frame(os)
    assert obs["tick"] == 0
    assert obs["route"]["app"] == "menu"
    assert obs["view"]["app"] == "menu"


def test_act_takes_dict_or_str():
    os = mrlypy.boot()
    a = mrlypy.act(os, {"verb": "nav.open", "args": {"app": "calculator"}})
    assert a["route"]["app"] == "calculator"
    b = mrlypy.act(os, '{"verb": "calculator.digit", "args": {"d": 4}}')
    assert focused(b)["display"] == "4"


def test_describe_covers_the_surface():
    d = mrlypy.describe()
    routes = [a["route"] for a in d["apps"]]
    assert "snake" in routes
    assert d["nav"][0]["verb"] == "nav.open"
    assert len(d["nav"]) == 1


def test_snake_round():
    os = mrlypy.boot()
    mrlypy.act(os, {"verb": "nav.open", "args": {"app": "snake"}})
    obs = mrlypy.act(os, {"verb": "snake.reset", "args": {"seed": 7}})
    assert focused(obs)["seed"] == 7
    obs = mrlypy.act(os, {"verb": "snake.turn", "args": {"dir": "left"}})
    assert focused(obs)["dir"] == "left"
    obs = mrlypy.act(os, {"verb": "snake.step", "args": {"n": 3}})
    assert focused(obs)["steps"] == 3
    assert not focused(obs)["over"]
