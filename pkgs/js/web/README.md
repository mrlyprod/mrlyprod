# mrlyweb

The Mrly world in JavaScript. One wasm, four doors: `boot`, `list`, `call`, `read`.

```js
import init, { boot, list, call, read } from "mrlyweb"

await init()
const os = boot("full")
const world = JSON.parse(list(os))
const env = JSON.parse(call(os, JSON.stringify({ verb: "nav.open", args: { app: "snake" } })))
const tick = JSON.parse(read(os, "tick"))
```

- `boot(loadout)` wakes a world: `"full"` or `"arcade"`.
- `list(os, shape)` names every app and verb aboard.
- `call(os, req)` performs one verb and returns the envelope.
- `read(os, path, shape)` reads any state by path; absent reads are `"null"`.

Everything crosses the edge as JSON text. The Rust kernel rides inside the wasm; data crosses, objects never do.
