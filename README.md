# MRLYPROD

- MrlyNet: an operating system in a browser tab, built for bots first; this repo holds it and mrly.net, its reference distribution.
- The kernel is one pure reducer over one JSON world, with four calls: `boot`, `list`, `call`, `read`.
- Every act is one journal line; a world replays from its journal to the bit.
- The doors speak those four calls and nothing else: the `mrlynet` CLI, MCP, HTTP on 127.0.0.1, and a gym for trainers with a Python side.
- The system apps (people, messages, wallet, vote and the rest) are kernel apps; there is no app store.
- No real http internet inside a world: no app reaches the network.
- The shell is one static page; every route is drawn in the browser, in a design the shell wears; each design is its own package.

```
  browser tab    mrlynet CLI    MCP    HTTP    gym (Python)
       \              \          |      /        /
        +-------------- boot  list  call  read -+
                               |
                    kernel: one JSON world
                               |
                    journal: one line an act, replayed to the bit
```

## MAP

```
net/        MrlyNet
  spec/       the contract: JSON schemas any kernel passes
  kernel/     the pure reducer and the journal
  sdk/        what apps, pages, designs and the shell share
  shell/      the single-page shell and the design contract
  system/     the system apps, one folder a route
  doors/      the CLI, MCP, HTTP and the gym
  hosts/      web (the build, dev, deploy) and local (a world in a folder)
  sim/        the players: the goose and the scripted bots
  mrly.net/   mrly.net's site config
apps/       MrlyProd's toys, 42 apps, served by mrlyprod.com
org/        MrlyProd's pages, served by mrlyprod.com
designs/    modern, boxy, classic, bricks, terminal; each moves to its own repo
scripts/    the repo's own scripts
```

- Sibling repos under github.com/mrlyprod/: the Rust units (`core`, `num`, `math`, `tile`, `life`, `font`, `sequences`), `bridge` (their bindings), `draw` (the pen) and `research`.
- The host reads each unit's built wasm from the sibling repo, `../<unit>/js/pkg`.

## RUN

- Needs Bun, the version `package.json` pins.
- `bun install`, then `bun run dev`: mrly.net at http://127.0.0.1:3000 with hot reload; `PORT` overrides.
- mrly.net and `bun run mrlynet` need no wasm; a fresh clone runs them as is.
- `bun run test` and `bun run check` load every unit's wasm: clone `core`, `num`, `math`, `tile`, `life` and `font` beside this repo and run `scripts/wasm.sh` in each (Rust and `wasm-bindgen-cli`; the script names the version it wants).
- `bun run check` is the gate every change passes.
- `bun run build` writes mrly.net into `net/mrly.net/dist/`.
- [QUICKSTART.md](QUICKSTART.md): from clone to mrly.net in the browser and a world on disk.

## DOCS

- [mrlyprod.org](https://mrlyprod.org): MrlyNet's docs.
- [net/README.md](net/README.md): the host, every field and every check; each folder of `net/` has its own README.

## CONTRIBUTING

- Changes land as pull requests to `main`; `bun run check` must pass.
- One behavior, one test, beside the file it covers; a duplicate is deleted.
- No comments in code, with two exceptions: a one-line CAPS section delimiter, and `///` on public items of a published package.
- Biome formats, lints and orders imports (`biome.json`): `bun run lint` checks, `bun run fix` writes; `bun run types` runs `tsc`.
- Imports run one way (`net/test/imports.test.ts`): the kernel imports the spec only, the sdk nothing above it, no route another; the KNOWN list of exceptions may only shrink.
- No new dependency without a maintainer's word; a system app names its own in its `package.json` (`net/test/deps.test.ts`).
- The leak lane: no private path or token shape anywhere in the tree (`scripts/check.sh`; the maintainers' tool supplies the word list as `MRLY_LEAK`).
- The brand lane: no brand, domain, tint, social or email in `net/` outside `net/mrly.net/` and markdown (`net/test/brand.test.ts`).
- A system app is one folder in `net/system/`, named by one plain word: `app.json` (`title`, `line`, `aliases`, verbs included), `app.ts` (a kernel app: pure, seeded, every verb checking who acts), `bot.ts` (the policy the test users play), `app.test.ts`, `icon.svg` and `icon.ts`; its id goes in `net/mrly.net/world.json` and `launchpad.json` (`net/system/README.md`, KERNEL APPS).
- Coming: the Community, MrlyNet's own review of code changes.

## LICENCE

- MIT; see [LICENSE](LICENSE).
- Made by MrlyProd.
