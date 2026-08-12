# mrlyweb

The Mrly world in Python. One wheel, four doors: `boot`, `list`, `call`, `read`.

```python
import mrlyweb

os = mrlyweb.boot("full")
world = mrlyweb.list(os)
env = mrlyweb.call(os, {"verb": "nav.open", "args": {"app": "snake"}})
tick = mrlyweb.read(os, "tick")
```

- `boot(loadout)` wakes a world: `"full"` or `"arcade"`.
- `list(os, shape=None)` names every app and verb aboard.
- `call(os, req)` performs one verb, dict or JSON text, and returns the envelope.
- `read(os, path="", shape=None)` reads any state by path; absent reads are `None`.

The Rust kernel rides inside the wheel. Data crosses the edge, objects never do.
