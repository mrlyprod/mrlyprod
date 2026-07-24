# Commands

## SHIP (managed by utils/ship.py - the front door: gates what changed, rebuilds, pushes)

```sh
uv run python utils/ship.py
uv run python utils/ship.py fast   # no gates
```

## UTILS (each runs standalone)

```sh
uv run python utils/tree.py        # regenerate TREE.md (ship runs it too)
uv run python utils/spaghetti.py  # remove target, .venv, node_modules, __pycache__, .DS_Store
```

## GIT (managed by utils/git.py - the plumbing under ship.py)

```sh
uv run python utils/git.py status
uv run python utils/git.py push          # gates, stage all, commit, push
uv run python utils/git.py publish       # wipe history, force-push a fresh main
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
uv run python utils/layers.py           # apps never import apps; run alongside the tests
cargo fmt
cargo clippy -- -D warnings
cargo run -p mrlynet --example <name>   # examples live in pkgs/mrlyrs/mrlynet/examples/
cargo run -p mrlynet --example fixtures # regenerate apps/web/fixtures/*.json from frame()
cargo doc --open
cargo clean
```

## CLI (a terminal face for the kernel; drive apps without a browser)

```sh
cargo run -p mrlycli -- describe                 # kernel surface as JSON
cargo run -p mrlycli -- verbs snake              # one app's verbs and args
cargo run -p mrlycli -- verbs                     # every app and its verb count
echo '<calls>' | cargo run -p mrlycli -- run --facts   # replay, print state, grids collapsed
echo '<calls>' | cargo run -p mrlycli -- shot --out f.png   # replay, write the frame as a PNG
cargo run -p mrlycli -- repl                      # interactive; :verbs :shot :render :help
```

Calls are JSON lines or a JSON array, e.g. `{"verb":"nav.open","args":{"app":"snake"}}`.

## MRLYPY (Rust -> Python via maturin)

```sh
uv run maturin develop --manifest-path pkgs/mrlypy/Cargo.toml --release
uv run python pkgs/mrlypy/tests/smoke.py
uv run pytest pkgs/mrlypy
uv run python lab/demo.py            # drive mrlynet from python: calculator, theme, a snake round
rm -rf .venv && uv sync && uv run maturin develop --manifest-path pkgs/mrlypy/Cargo.toml --release   # clean rebuild
```

## MRLYJS (Rust -> wasm via wasm-pack)

```sh
rustup target add wasm32-unknown-unknown
brew install wasm-pack                      # or: cargo install wasm-pack
wasm-pack build pkgs/mrlyjs --target web    # outputs pkgs/mrlyjs/pkg/
cargo test -p mrlynet --test golden         # fixtures vs frame(), after vocabulary changes
```

## WEB (dev server + golden screenshots)

```sh
cd apps/web && bun run index.ts             # dev server on :3000 (rebuild wasm first if the core changed)
bun run apps/web/shot.ts <app>              # from the repo root, server running: screenshot <app> into data/web/<app>.png
bun run apps/web/verify.ts                  # smoke the web face in code: wasm kernel + verbs + every view (no server, no browser)
```

## OPS (infra + web deploys; needs the AWS keys in .env)

```sh
uv run python ops/deploy.py build          # release wasm + bun bundle + public/ + sitemap into data/web/dist/
uv run python ops/deploy.py push           # build, wipe and sync S3, invalidate CloudFront
uv run python ops/cloudfront.py check      # report cert, zone, distribution, dns state
uv run python ops/cloudfront.py create     # create the mrly distribution (site + cdn/*)
uv run python ops/cloudfront.py flip       # point aliases + route53 at the distribution
uv run python ops/cloudfront.py harden     # attach the CSP policy
uv run python ops/cdn.py                   # sync cdn/ -> mrly.net/cdn/, changed files only
uv run python ops/dns.py records           # list the zone (also: zones, set, drop)
uv run python ops/acm.py list              # list certificates (also: request, validation)
uv run python ops/s3.py buckets            # list buckets (also: keys, drop, wipe, rmbucket)
uv run python ops/budget.py check          # spend vs the $20 tripwire (also: set)
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
```
