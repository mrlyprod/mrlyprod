# KERNEL

- One pure reducer over one JSON world, in TypeScript; it imports only `../spec/`.
- It owns app state and rules, nothing else: no pixels, no network, no clock, no design, no MrlyProd.
- `index.ts` is the whole surface; `kernel.test.ts` is its proof.

```
        act { verb, args, who, at }
                  |
           +------v------+   line { tick, at, who, verb, args, n }
           |   kernel    |----------------------------> journal
           | call(w, a)  |
           +------+------+
                  | envelope { tick, route, state, moves, last, wishes, paid? }
       +----------+-----------+
       |                      |
  the shell (later)      the doors: mrlynet CLI, MCP
```

## THE FOUR CALLS

- `kernel(apps)` installs apps and returns the four calls; an app id is taken once.
- `boot(config) -> world`: each app boots from a seed drawn from the world's seed and its id (`seedOf`); no IO.
- Rules: `config.rules` holds the world's knobs, each pinned (`"payments": "on"`) or picked (`"payments": { "pick": ["on", "off"] }`), drawn from the seed at boot (`settle`); `world.rules`, `ctx.rules` and each app's boot see the settled values: "Can this be randomized, or is this a constant?" Reality pins a rule; a simulation may vary it.
- `list(world) -> { moves, apps }`: every legal verb now with its args schema, plus each app's manifest; it shifts as state does.
- `call(world, act) -> { world, envelope, line }`: one act in, a fresh world out; the old world is untouched; `line` is what the journal appends.
- `read(world, path, shape?, who?) -> envelope`: `""` is every app, `snake` one app, `snake/seed` one leaf; `shape` keeps only the keys it names (`["a", "b"]` or `{ "a": true, "b": { "c": true } }`); `who` is the reader: an app's `read(state, path, who)` shows a private part only to its owner, and a call's envelope shows the caller's own view, never another user's private state.

## AN APP

- `app({ id, user, title, line, boot, verbs, moves?, read?, beat? })`; `boot({ seed, config, actors })` returns its first state, `actors` the world's (null in an open world).
- A verb is `{ line, args, run(state, args, ctx), pay? }`; `args` is a schema or a function of the state; `run` returns the next state and never mutates its input.
- `pay(state, args, ctx)` returns `{ to, amount, from? }` or `null`: the kernel settles it through the app named `wallet` (`money.move(state, { from, to, amount, at, verb })`, new state or a refusal note) before `run`, `from` the caller unless named, the caller always one side; the envelope then carries `paid: { from, to, amount }`; a refusal from the wallet or from `run` leaves both untouched.
- `ctx` holds `who`, `at`, `tick`, `draw()` (a u32 from the world's generator), `read(path)` (any app's state at call time, a copy: an app may look at another, never write it; a change in another app is a wish), `wish(w)` and `refuse(note)`.
- `moves(state)` names the verbs legal now; without it every verb always is.
- A system app has no `user`: its verbs are `<app>.<verb>` (`people.rename`, `messages.send`); it is a route's `app.ts` (`../system/README.md`), holds each user's data keyed by user id, checks `ctx.who` where ownership matters and refuses with a plain note otherwise.
- `read(state, path)` may answer `<app>/@<id>`, one user's slice (`../sdk/system.ts` `sliced`); the kernel's reads stay open to every watcher, only acts are owned.
- `../sdk/apps.ts` lifts today's apps: `scene()` turns a scene's knobs into `set` and `reroll` (the scene's own `PAGE.reroll` when it has one), its `PAGE.acts` into verbs (`mrlyprod.arcs.up`), and an `engine.ts` reducer's acts into verbs (`mrlyprod.snake.turn`); a world's seed and an app's `config` in the world config set its opening knobs.

## OUTCOMES

- A broken act throws `Fault` with one code and a plain note, and changes nothing: `bad-act` (fails its schema), `unknown-verb` (names the nearest), `refused` (a watcher acting), `bad-config`, `spec-mismatch`.
- A refused act is an outcome: journaled, the state kept, `last` `{ ok: false, note }`, the note naming the nearest legal verb.

## DETERMINISM

- No clock: time arrives on the act as `at`, in milliseconds, never before the world's last `at`.
- No randomness outside the world: one xorshift32 generator (`rand.ts`), its state carried in `world.rand`, seeded by `mix(seed)`; a Rust kernel reproduces it bit for bit.
- Plain JSON only: `canon()` sorts keys and refuses anything JSON cannot hold; big integers cross as decimal strings; `digest()` is FNV-1a 64 over the canonical bytes.
- A float in state is data an act carried, never one an app computed with `Math` functions, which engines may round differently.
- Replay (`journal.ts`): boot from the config, apply the lines in order; seek is a replay that stops early; a thawed world given another act is a fork.
- No player lives here: the goose and the bots are `../sim/`; a player is not a reducer.
