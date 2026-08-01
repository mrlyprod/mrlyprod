import mrlyweb

world = mrlyweb.describe()
assert world["apps"]

os = mrlyweb.boot()
obs = mrlyweb.act(os, {"verb": "nav.open", "args": {"app": "notes"}})
obs = mrlyweb.act(os, {"verb": "notes.add", "args": {"text": "hi"}})
assert obs["view"]["state"]["found"][0]["text"] == "hi"

mrlyweb.act(os, {"verb": "nav.open", "args": {"app": "snake"}})
assert any(mrlyweb.goose_step(os, 7) for _ in range(10))

print("smoke:", len(world["apps"]), "apps, tick", obs["tick"])
