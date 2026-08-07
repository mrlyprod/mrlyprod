# Commands

## TEST (managed by utils/test.py - the front door: gates what changed, rebuilds, regenerates)

```sh
uv run python utils/test.py
uv run python utils/test.py loud     # the same, streaming each command's output
uv run python utils/test.py record   # re-pin apps/web/fixtures/ + apps/cli/tests/frames/ first, then gate
```

`record` is the only way the goldens move: the plain gate compares against them and fails on a
mismatch, so a vocabulary change is a deliberate re-pin, never a silent one.

## UTILS (each runs standalone)

```sh
uv run python utils/tree.py        # regenerate TREE.md (test runs it too)
uv run python utils/stats.py       # regenerate STATS.md (test runs it too)
uv run --group font python utils/brand.py         # regenerate MrlyFont + the brand images, write data/cdn and every font sheet
uv run --group font python utils/brand.py fetch   # the same, refetching files/vendor from Google Fonts first
uv run python utils/spaghetti.py  # remove target, .venv, node_modules, __pycache__, .DS_Store
```

## GIT (managed by git.py at the repo root - GitHub only, stamped from a master outside the repo)

```sh
python3 git.py status
python3 git.py push          # stage all, commit, push (run test.py first)
python3 git.py publish       # wipe history, force-push a fresh main
```

The source tarball on git.mrly.net ships from the private ops console, not with git.

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
uv run python utils/layers.py           # apps never import apps; run alongside the tests
uv run python utils/doors.py            # the wire has four doors; both bindings, no side modules
cargo fmt                               # ship runs it too
cargo clippy -- -D warnings
cargo run -p mrlyweb --example <name>   # examples live in pkgs/mrlyrs/mrlyweb/examples/
cargo run -p mrlyweb --example fixtures # regenerate apps/web/fixtures/*.json from the envelope
cargo run -p mrlyweb --example bake     # regenerate apps/web/src/gen/*.json (palette, shaders, mark)
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
cargo run -p mrlycli -- goose snake --seed 7 --steps 50 --trace   # random legal calls, one JSON line each
cargo run -p mrlycli -- repl                      # interactive; :verbs :shot :render :help
cargo run -p mrlycli -- tui                       # the raw-mode face; / commands, arrows play
cargo run -p mrlycli -- frame apps/cli/tests/screenplays/snake.jsonl 80x24   # a TUI screen as text
cargo run -p mrlycli -- frame --record            # re-pin apps/cli/tests/frames/ (test.py record runs it)
```

Calls are JSON lines or a JSON array, e.g. `{"verb":"nav.open","args":{"app":"snake"}}`.

## MRLYPY (Rust -> Python via maturin; imports as mrlyweb)

```sh
uv run maturin develop --manifest-path pkgs/mrlypy/web/Cargo.toml --release
uv run python -m pytest pkgs/mrlypy/web/tests
uv run python pkgs/mrlypy/web/tests/smoke.py
rm -rf .venv && uv sync && uv run maturin develop --manifest-path pkgs/mrlypy/web/Cargo.toml --release   # clean rebuild
```

## MRLYJS (Rust -> wasm via wasm-pack)

```sh
rustup target add wasm32-unknown-unknown
brew install wasm-pack                      # or: cargo install wasm-pack
wasm-pack build pkgs/mrlyjs/web --target web   # the os kernel -> pkgs/mrlyjs/web/pkg/
cargo test -p mrlyweb --test golden         # fixtures vs the envelope, after vocabulary changes
```

## WEB (dev server + golden screenshots)

```sh
cd apps/web && bun run index.ts             # dev server on :3000 (rebuild wasm first if the core changed)
bun utils/shot.ts                           # every site, every route, into data/<site>/*.png (boots servers itself)
bun utils/shot.ts web snake                 # one site, chosen routes; sites: web net git
bun utils/shot.ts --size 390x844 net        # any viewport
bun utils/shot.ts http://localhost:3000/snake out.png   # any url
bun run apps/web/verify.ts                  # smoke the web face in code: wasm kernel + verbs + every view (no server, no browser)
```

## BUN (runtime + package manager; brew-managed)

```sh
cd apps/web
bun install                      # install from bun.lock into node_modules/
bun add <pkg>                    # add a runtime dependency
bun add --dev <pkg>              # add a dev-only dependency
bun run <script>                 # run a package.json script
bun run index.ts                 # run a file directly (no node)
bun update                       # refresh the lockfile
bunx <cmd>                       # run a bin without installing (= npx)
bunx tsc --noEmit --project apps/web    # typecheck the web face (pre-push gate; works from the repo root)
bunx tsc --noEmit --project apps/net        # typecheck the net site (out of the gate: /build rode the dead math island)
bunx tsc --noEmit --project apps/git        # typecheck the git site
bun run --cwd apps/net dev       # the landing site on :5173
bun run --cwd apps/net links     # resolve every link and image on the site, nonzero on a miss
bun run --cwd apps/git dev       # the projection on :5173, serving /raw/** and /manifest.json from the working tree
bun run --cwd apps/git links     # resolve every link and image in every tracked .md, nonzero on a miss
bun run --cwd pkgs/mrlyui dev    # the design system kitchen sink on :5173
bunx tsc --noEmit --project pkgs/mrlyui     # typecheck the design system (pre-push gate)
```
