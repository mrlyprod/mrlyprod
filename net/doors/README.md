# DOORS

- A door is a transport that speaks the spec: the four calls in, JSON envelopes out, nothing else.
- The kernel never learns which door an act came through; a door adds no verbs; a door holds no per-app code.

## MRLYNET

- `cli/mrlynet.ts`, run as `bun run mrlynet` or the `mrlynet` bin; the name comes from the site config's `cli` (default `mrlynet`); `goose` and `sim` are the two verbs that reach `../sim/`, the players, through `../hosts/local/sim.ts`.
- `boot [config.json | '{json}'] [--fresh]`, `list`, `call <verb> ['{args}']`, `read [path] ['{shape}']`, `goose [n]`, `sim [config] [--bots N] [--rounds N]` (the config's actors play, else N bots), `serve [--port 4100]`, `mcp`, `gym`, `apps`.
- `--world <name|dir>` picks the world folder (default `local`); `--as <id>[:kind]` says who acts.
- Output is JSON on stdout; a fault is `{ "error": { code, note } }` on stderr, exit 1.
- The door stamps `at` with the wall clock, never earlier than the world's last act.

## MCP

- `mcp/mcp.ts`, `mrlynet mcp`: JSON-RPC 2.0 over stdio, no SDK.
- Each legal verb is a tool, its args schema the tool's input schema; dots become underscores (`mrlyprod.snake.set` is `mrlyprod_snake_set`).
- `read` is a tool too; every app is a resource, `mrlynet://world/<path>`.
- A call that shifts the legal verbs sends `notifications/tools/list_changed`.
- A refusal or a fault comes back as a tool result with `isError`, its note the teaching sentence.
- `../../.mcp.json` registers it for Claude Code at the repo root; the first run boots the `local` world with every app.

## HTTP

- `http/http.ts`, `mrlynet serve`: the four calls as routes on 127.0.0.1, a world by its name, the folder of the local host.
- `POST /api/1/worlds` with a config boots one; `GET /api/1/worlds/<id>/list`; `POST /api/1/worlds/<id>/call` with `{ verb, args }`; `GET /api/1/worlds/<id>/read/<path>?shape=<json>`.
- A refused act is a 200 with `last.ok` false; a fault is `{ error: { code, note } }` with its status: `bad-act` and `bad-config` 400, `no-iden` 401, `refused` 403, `unknown-verb` and `unknown-world` 404, `spec-mismatch` 409.
- No iden yet: every act carries the local agent's `who`; the cloud host (Lambda) waits.

## GYM

- `gym/gym.ts`, `mrlynet gym`: many in-memory worlds in one process, for trainers; one JSON line per batch each way over stdio, no disk.
- A request is `{ op: boot | call | list | read | lines | close, ... }`; the reply holds one envelope or one fault per world, in order.
- `who` is the bot `gym` unless given; `at` advances a fixed beat (`BEAT`, 1000 ms) unless given, never the wall clock.
- Each act runs the bus (`../sdk/system.ts` `bus`) as every host does, so a world stepped through the gym holds the journal `mrlynet` would write; `lines` hands each world's journal back, for a replay from the lines alone.
- `gym/mrlynet.py` is the Python side, stdlib only; `gym/README.md` has the protocol.

## NOT YET

- HTTP in the cloud.
