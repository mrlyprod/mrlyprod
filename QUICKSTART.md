# QUICKSTART

- From a fresh clone to mrly.net in the browser and a MrlyNet world on disk.
- Needs `git` and Bun, the version `package.json` pins; no Rust and no wasm.

## TEN COMMANDS

```
git clone https://github.com/mrlyprod/mrlyprod && cd mrlyprod
bun install
bun run dev
bun run mrlynet apps
bun run mrlynet boot
bun run mrlynet list
bun run mrlynet call people.rename '{"name":"Ada"}'
bun run mrlynet read people
bun run mrlynet sim net/mrly.net/world.json --world demo --rounds 10
bun run build
```

## WHAT EACH DOES

- `dev` serves mrly.net at http://127.0.0.1:3000 with hot reload and holds the terminal; run the rest in a second one.
- `apps`: the apps this site can boot, an id, a title and a line each.
- `boot`: a world named `local` holding every app, in `data/net/hosts/local/worlds/local/` (`config.json`, `journal.jsonl`); `--fresh` starts it over.
- `list`: every legal verb now, with its args schema.
- `call`: one act, as `local` unless `--as <id>[:kind]` says otherwise; it prints the envelope and appends one line to the journal.
- `read`: `''` is the world, `people` one app, `<app>/<key>` one leaf.
- `sim`: the test users `test-1` to `test-8` play 10 rounds in a world named `demo`, a second of world time a round; the report replays the journal to check the world to the bit.
- `build`: the whole site into `net/mrly.net/dist/`.

## NEXT

- Output is JSON on stdout; a fault is `{ "error": { code, note } }` on stderr, exit 1.
- `--world <name|dir>` picks the world; `bun run mrlynet` alone prints every verb.
- `bun run mrlynet mcp` serves the world over MCP on stdio; `.mcp.json` registers it at the repo root.
- `bun run mrlynet serve` serves the four calls over HTTP on 127.0.0.1:4100; `bun run mrlynet gym` is the trainers' door, `net/doors/gym/mrlynet.py` its Python side.
- `bun run init <dir>` writes a site config of your own; `MRLY_ROOT=<dir>` points every verb at it (`net/README.md`, CONFIG).
