# Commands

## TEST (managed by utils/test.py - the front door: gates what changed, rebuilds, regenerates)

```sh
uv run python utils/test.py
uv run python utils/test.py loud     # the same, streaming each command's output
uv run python utils/test.py fast     # fmt + tests/clippy/tsc for what changed; composes with loud
uv run python utils/test.py record   # re-pin every golden home first, then gate
```

`fast` is the iteration lane: it maps changed paths to crates, tests changed crates plus their
dependents, and typechecks touched ts projects; it skips wasm, maturin, pytest, vite, links,
tree and stats, so the plain gate stays the ship gate.

`record` is the only way the goldens move: the plain gate compares against them and fails on a
mismatch, so a vocabulary change is a deliberate re-pin, never a silent one. The five homes are
`sites/web/fixtures/`, `sites/web/src/gen/`, `pkgs/js/mrlyui/src/gen/`,
`pkgs/rs/mrlycli/tests/frames/` and `pkgs/rs/mrlycli/tests/shots/`.

## UTILS (each runs standalone)

```sh
uv run python utils/tree.py        # regenerate TREE.md (test runs it too)
uv run python utils/stats.py       # regenerate STATS.md (test runs it too)
uv run --group font python utils/brand.py         # regenerate MrlyFont + the brand images, write data/cdn and every font sheet
uv run --group font python utils/brand.py fetch   # the same, refetching files/vendor from Google Fonts first
uv run python utils/logos.py       # regenerate files/logos: the mark, the banner, the wordmark, the loop, the palette
uv run python utils/logos.py stills   # only the svg and png stills
uv run python utils/logos.py motion   # only the gif and the mp4
uv run python utils/paints.py      # regenerate data/paints: one painted tile per edition
uv run python utils/spaghetti.py  # remove target, dist, .venv, node_modules, __pycache__, .DS_Store
```

## ENV (uv - manages Python + packages; env lives at .venv/)

```sh
uv python install 3.13.12   # satisfies .python-version
uv python pin 3.13.12       # writes .python-version
uv sync                     # create .venv/ + install from uv.lock
uv add <pkg>                # add a runtime dependency
uv add --dev <pkg>          # add a dev-only dependency
uv run <cmd>                # run inside the env (no activate)
uv lock                     # refresh the lockfile
```

## RUST

```sh
cargo build
cargo test                              # fast; skips slow #[ignore] tests
cargo test -- --ignored                 # the slow statistical tests (hash metrics)
uv run python utils/layers.py           # the crate DAG: app isolation, mrlyweb's roll-call
uv run python utils/doors.py            # the wire has four doors; both bindings, no side modules
cargo fmt                               # ship runs it too
cargo clippy --workspace --all-targets -- -D warnings
cargo run -p mrlyweb --example <name>   # examples live in pkgs/rs/mrlyweb/examples/
cargo run -p mrlyweb --example fixtures # regenerate sites/web/fixtures/*.json from the envelope
cargo run -p mrlyweb --example bake     # regenerate the baked json: web gets palette, shaders, skins, rigs; mrlyui gets mark
cargo doc --open
cargo clean
```

## CLI (a terminal face for the kernel; drive apps without a browser)

```sh
curl -fsSL mrly.net/install.sh | sh              # install the released binary as ~/.local/bin/mrly
cargo run -p mrlycli -- list                     # kernel surface as JSON
cargo run -p mrlycli -- verbs snake              # one app's verbs and args
cargo run -p mrlycli -- verbs                     # every app and its verb count
echo '<calls>' | cargo run -p mrlycli -- run --facts   # replay, print state, grids collapsed
echo '<calls>' | cargo run -p mrlycli -- shot --out f.png   # replay, write the frame as a PNG
cargo run -p mrlycli -- shot --route mandelbrot --size 512 --out m.png   # one route from a fresh boot
cargo run -p mrlycli -- shot --list               # every route the native eye answers for
cargo run -p mrlycli -- shot --record             # re-pin pkgs/rs/mrlycli/tests/shots/ (test.py record runs it)
cargo run -p mrlycli -- goose snake --seed 7 --steps 50 --trace   # random legal calls, one JSON line each
cargo run -p mrlycli -- repl                      # interactive; :verbs :shot :render :help
cargo run -p mrlycli -- tui                       # the raw-mode face; / commands, arrows play
cargo run -p mrlycli -- frame pkgs/rs/mrlycli/tests/screenplays/snake.jsonl 80x24   # a TUI screen as text
cargo run -p mrlycli -- frame --record            # re-pin pkgs/rs/mrlycli/tests/frames/ (test.py record runs it)
```

Calls are JSON lines or a JSON array, e.g. `{"verb":"nav.open","args":{"app":"snake"}}`.

## MRLYWEB-PY (Rust -> Python via maturin; imports as mrlyweb)

```sh
uv run maturin develop --manifest-path pkgs/py/web/Cargo.toml --profile gate
uv run python -m pytest pkgs/py/web/tests
uv run python pkgs/py/web/tests/smoke.py
rm -rf .venv && uv sync && uv run maturin develop --manifest-path pkgs/py/web/Cargo.toml --profile gate   # clean rebuild
```

## MRLYWEB-JS (Rust -> wasm via wasm-pack)

```sh
rustup target add wasm32-unknown-unknown
brew install wasm-pack                      # or: cargo install wasm-pack
wasm-pack build pkgs/js/web --target web --out-name mrlyweb   # the os kernel -> pkgs/js/web/pkg/
cargo test -p mrlyweb --test golden         # fixtures vs the envelope, after vocabulary changes
```

## WEB (dev server + golden screenshots)

```sh
bun run --cwd sites/web dev                  # vite dev server on :5176 under shot.ts (rebuild wasm first if the core changed)
bun utils/shot.ts                           # every site, every route, into data/<site>/*.png (boots servers itself)
bun utils/shot.ts web snake                 # one site, chosen routes; sites: web net git ui
bun utils/shot.ts --size 390x844 net        # any viewport
bun utils/shot.ts http://localhost:5176/snake out.png   # any url
bun run sites/web/verify.ts                  # smoke the web face in code: wasm kernel + verbs + every view (no server, no browser)
bun run --cwd sites/web site                 # build the face into data/web/dist
```

shot.ts pins the dev ports: net :5173, git :5174, ui :5175, web :5176.
A plain `bun run --cwd sites/web dev` is stock vite on :5173.

## BUN (runtime + package manager; brew-managed)

```sh
bun install                      # install from bun.lock into node_modules/
bun add <pkg>                    # add a runtime dependency
bun add --dev <pkg>              # add a dev-only dependency
bun run <script>                 # run a package.json script
bun run <file>.ts                # run a file directly (no node)
bun update                       # refresh the lockfile
bunx <cmd>                       # run a bin without installing (= npx)
bunx tsc --noEmit --project sites/web    # typecheck the web face (pre-push gate; works from the repo root)
bunx tsc --noEmit --project sites/net        # typecheck the net site
bunx tsc --noEmit --project sites/git        # typecheck the git site
bun run --cwd sites/net dev       # the landing site on :5173
bun run --cwd sites/net links     # resolve every link and image on the site, nonzero on a miss
bun run --cwd sites/git dev       # the projection on :5174 under shot.ts, serving /raw/** and /manifest.json from the working tree
bun run --cwd sites/git links     # resolve every link and image in every tracked .md, nonzero on a miss
bun run --cwd pkgs/js/mrlyui dev    # the design system kitchen sink on :5175 under shot.ts
bunx tsc --noEmit --project pkgs/js/mrlyui     # typecheck the design system (pre-push gate)
uv run --group font python utils/brand.py icons     # rebake icon subsets after widening SymbolName
uv run --group font python utils/brand.py bundle    # bake fonts into pkgs/js/mrlyui for the offline local.css
```

## AWS (the sites, the lambda, the infra; ids live in the desk's `../.env`)

```sh
uv run python aws/deploy.py net push     # build the landing site, sync S3, invalidate CloudFront; aws/COMMANDS.md has the rest
```
