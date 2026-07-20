import mrlypy

world = mrlypy.describe()
routes = [a["route"] for a in world["apps"]]
print("mrlynet", world["version"], "·", len(routes), "apps:", " ".join(routes))

os = mrlypy.boot()

mrlypy.act(os, {"verb": "nav.open", "args": {"app": "calculator"}})
for verb, args in [
    ("calculator.digit", {"d": 6}),
    ("calculator.op", {"op": "mul"}),
    ("calculator.digit", {"d": 7}),
    ("calculator.equals", {}),
]:
    obs = mrlypy.act(os, {"verb": verb, "args": args})
print("calculator: 6 x 7 =", obs["state"]["display"])

mrlypy.act(os, {"verb": "nav.open", "args": {"app": "settings"}})
obs = mrlypy.act(os, {"verb": "settings.set", "args": {"key": "color", "value": "mint"}})
print("settings: color ->", obs["state"]["color"])

mrlypy.act(os, {"verb": "nav.open", "args": {"app": "snake"}})
obs = mrlypy.act(os, {"verb": "snake.reset", "args": {"seed": 7}})
n = obs["state"]["space"]["n"]
verbs = [v["verb"] for v in obs["actions"]]
print("snake: seed 7, action space", obs["state"]["space"], "verbs", verbs)

step = 0
while not obs["state"]["terminated"] and step < 300:
    args = {"a": (step // 5) % n} if step % 5 == 0 else {}
    obs = mrlypy.act(os, {"verb": "snake.act", "args": args})
    step += 1
s = obs["state"]
fate = "terminated" if s["terminated"] else "alive at the cap"
print("snake: score", s["score"], "after", s["steps"], "steps,", fate)

obs = mrlypy.act(os, {"verb": "nav.open", "args": {"app": "menu"}})
print("nav: menu ·", obs["route"]["app"], "· tick", obs["tick"])
