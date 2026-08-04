import mrlyweb

os = mrlyweb.boot()

world = mrlyweb.list(os)
assert world["apps"]

mrlyweb.call(os, {"verb": "nav.open", "args": {"app": "notes"}})
env = mrlyweb.call(os, {"verb": "notes.add", "args": {"text": "hi"}})
assert env["view"]["state"]["found"][0]["text"] == "hi"

assert mrlyweb.read(os, "notes/found/0/text") == "hi"
assert mrlyweb.read(os, "notes/nowhere") is None

print("smoke:", len(world["apps"]), "apps, tick", env["tick"])
