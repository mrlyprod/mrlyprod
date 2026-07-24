import mrlypy

world = mrlypy.describe()
assert world["apps"]

os = mrlypy.boot()
obs = mrlypy.act(os, {"verb": "nav.open", "args": {"app": "notes"}})
obs = mrlypy.act(os, {"verb": "notes.add", "args": {"text": "hi"}})
assert obs["view"]["state"]["found"][0]["text"] == "hi"

print("smoke:", len(world["apps"]), "apps, tick", obs["tick"])
