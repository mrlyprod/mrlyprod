from pathlib import Path

import mrlypy
import numpy as np

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
print("calculator: 6 x 7 =", obs["view"]["state"]["display"])

mrlypy.act(os, {"verb": "nav.open", "args": {"app": "settings"}})
obs = mrlypy.act(os, {"verb": "settings.set", "args": {"key": "color", "value": "mint"}})
print("settings: color ->", obs["view"]["state"]["color"])

mrlypy.act(os, {"verb": "nav.open", "args": {"app": "snake"}})
obs = mrlypy.act(os, {"verb": "snake.reset", "args": {"seed": 7}})
state = obs["view"]["state"]
verbs = [v["verb"] for v in obs["view"]["actions"]]
print("snake: seed 7, board", state["settings"]["grid"], "verbs", verbs)

dirs = ["up", "right", "down", "left"]
step = 0
while not state["over"] and step < 300:
    if step % 5 == 0:
        mrlypy.act(os, {"verb": "snake.turn", "args": {"dir": dirs[(step // 5) % 4]}})
    obs = mrlypy.act(os, {"verb": "snake.step", "args": {}})
    state = obs["view"]["state"]
    step += 1
fate = "over" if state["over"] else "alive at the cap"
print("snake: score", state["score"], "after", state["steps"], "steps,", fate)

w, h, buf = mrlypy.capture(os)
pixels = np.frombuffer(buf, np.uint8).reshape(h, w, 4)
print("capture: snake canvas as ndarray", pixels.shape, "mean", round(float(pixels.mean()), 1))

png = mrlypy.face_png(os)
out = Path("data/lab")
out.mkdir(parents=True, exist_ok=True)
(out / "snake-face.png").write_bytes(png)
print("face: wrote data/lab/snake-face.png,", len(png), "bytes")

obs = mrlypy.act(os, {"verb": "nav.open", "args": {"app": "menu"}})
print("nav: menu ·", obs["route"]["app"], "· tick", obs["tick"])
