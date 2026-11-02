# GYM

- The fourth door, for trainers: many in-memory worlds in one Bun process, one JSON line per batch each way over stdio; no disk, no port, no clock.
- `gym.ts` is the door, run as `mrlynet gym`; `mrlynet.py` is its Python side, stdlib only.
- `gym.test.ts` and `test_mrlynet.py` are its proof.

```
  Python --spawn--> bun net/doors/cli/mrlynet.ts gym
    stdin:  { "op": ..., ... }            one line a batch
    stdout: [ entry 0, ..., entry n-1 ]   one line a batch
                  |
   +---------+ +---------+       +-----------+
   | world 0 | | world 1 |  ...  | world n-1 |
   +---------+ +---------+       +-----------+
```

## PROTOCOL

- A request is one JSON object a line: `{ "op": "boot" | "call" | "list" | "read" | "lines" | "close", ... }`.
- A reply is one JSON array a line, one entry per world, in order; world `i` is the `i`-th of the last boot.
- An entry is an envelope, or a fault `{ "error": { "code", "note" } }`; a fault in one world never stops the batch.
- A request the gym cannot take (not JSON, an unknown op, the wrong number of acts) is one fault for the whole line, never an array.

## OPS

- `boot`: `{ "op": "boot", "configs": [config, ...] }`, or `{ "op": "boot", "config": config, "seeds": [1, 2, 3] }` for one world a seed.
- A boot replaces every world and answers each one's envelope; a world that fails to boot holds a fault in its place.
- Only the apps the configs name are installed, each once per process.
- `call`: `{ "op": "call", "acts": [act | null, ...] }`, one act a world; `null` skips a world and answers it as it stands.
- An act is `{ verb, args?, who?, at? }`; `who` defaults to the bot `{ "id": "gym", "kind": "bot" }`, `at` to the world's last `at` plus a beat of 1000 ms.
- Each act runs the bus (`../../sdk/system.ts`): its notice wishes, act wishes and each app's `after` follow it as acts by the same user at the same time, so the world's tick may advance by more than one; the reply is the act's own envelope.
- `list`: `{ "op": "list" }` answers each world's legal moves, with their args schemas.
- `read`: `{ "op": "read", "path": "snake/play", "shape": ["score"] }` answers each world's envelope at that path; both are optional.
- `lines`: `{ "op": "lines" }` answers each world's journal since its boot, every line the bus wrote included; `replay` over them reaches the world to the bit.
- `close`: `{ "op": "close" }` answers `[]` and ends the process; so does the end of stdin.

## EXAMPLE

```
> {"op":"boot","config":{"name":"snake","apps":["snake"]},"seeds":[1,2]}
< [{"tick":0,"route":{"app":"snake"},"state":{...},"moves":[...],"last":{"ok":true,"note":null},...},{"tick":0,...}]
> {"op":"call","acts":[{"verb":"mrlyprod.snake.tick"},{"verb":"nope"}]}
< [{"tick":1,...},{"error":{"code":"unknown-verb","note":"no app owns nope; nearest: nav.open"}}]
```

## PYTHON

- `Gym(repo=None)` spawns `bun <repo>/net/doors/cli/mrlynet.ts gym`; `repo` defaults to this checkout.
- `boot(config, seeds=None)`, `step(acts)`, `moves()`, `read(path="", shape=None)`, `lines()`, `close()`: each returns one entry per world.
- A whole-line fault raises `Fault`, with `code` and `note`.
- `with Gym() as gym:` closes the door on the way out; no process is left.
- The Gymnasium mapping as plain methods, no Gymnasium import: `reset(seed)` boots again from the last config, seeds `seed` to `seed + n - 1` or a list; `step` is `call`, its envelope already the read; legal actions are `moves()`.
- Reward and done are facts in state: the caller reads them, as `read("snake/play", ["score", "over"])`; the door invents none.

## PROOF

- `bun test net/doors/gym`: 10,000 seeded snake worlds take three goose acts each in batches, then replay from their lines to the same `digest`; a fault stays in its own world; one act sequence through the gym and through the local store is one journal, `lines` handing it back.
- `uv run python -m unittest discover -s net/doors/gym -t net/doors/gym`: three seeds booted, stepped, read and closed, no process left.
