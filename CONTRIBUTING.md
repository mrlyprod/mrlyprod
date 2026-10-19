# CONTRIBUTING

- The map of this repo for the next maintainer, human or Claude.
- `README.md` has the tree; each folder's `README.md` has its own rules.

## THE GATE

- `scripts/check.sh` is the one gate: the leak grep, fmt, the bridge check, clippy, the tests (the ignored `mrly-math` ones included), the docs, the wasm, the JS tests, the Node smoke test, the Python tests, the site.
- It stops at the first failing step; each step's log lands in `data/scripts/check/`.
- `scripts/publish.sh` runs it before every release.
- A gitignored `.leak` file adds patterns the leak step refuses, one extended regex per line.
- While working, `cargo check -p <crate>` and `cargo test -p <crate>` are enough.
- `scripts/cargo.sh <cmd>` runs a command under the repo's cargo lock; the other scripts take it themselves.

## PREREQUISITES

- rustup: `rust-toolchain.toml` pins the toolchain and its targets.
- `cargo install wasm-bindgen-cli --version 0.2.125 --locked`, the version in `Cargo.lock`; `scripts/wasm.sh` refuses any other.
- bun, node and uv on the path; zig for the Linux wheels.

## GENERATED FILES

- Committed, never hand-edited: a wrong line is a generator fix, then `scripts/bridge.sh`.
- `.gitattributes` lists every file the bridge writes, each `linguist-generated`.
- `crates/bridge/manifest.json`: the parsed API every binding is written from.
- Python: `bindings/py/src/lib.rs`, `bindings/py/src/doors.rs`, `bindings/py/python/mrly/**` and `bindings/py/tests/test_manifest.py`.
- JS: `bindings/js/units/*`, `bindings/js/package.json`, `bindings/js/manifest.test.js`, and each `<unit>` and `index` as `.js`, `.node.js` and `.d.ts`.
- CLI: `bindings/cli/src/doors.gen.rs`.
- The bridge also rewrites the `"=X.Y.Z"` pins of the root `Cargo.toml` in place; that file stays hand-written.
- `site/**/*.gen.*` come from the site's own tools, not the bridge; `site/README.md` says which.
- `scripts/bridge.sh check` writes nothing and exits 1 when any of them is stale.

## STYLE

- No code comments; the one exception is a one-line CAPITALISED section delimiter, plus `///` docs on public items.
- One empty line between sections, never two.
- No em dash anywhere; a hyphen instead.
- Markdown: bullets, one idea per bullet, a paragraph on one line, never wrapped.
- One behavior, one test; a duplicate test is deleted.
- Before 1.0 there is no backwards compatibility to keep.

## ADD A FUNCTION

- Write a `pub fn` in `crates/mrly-<unit>/src`, reachable through a public module, with `///` docs and a doctest.
- Arguments and results are scalars, strings, the hand types (tensors, cells, images, colors, codes, `Rng`), types that derive `Serialize` and `Deserialize`, or classes (a struct with a self method).
- The bridge skips generics, closures, `impl Trait`, iterators, borrowed returns other than `&'static str`, `&mut` plain data and classes of another unit; `crates/bridge/README.md` has the full list.
- Run `scripts/bridge.sh`: it adds the function to JS, Python and the CLI and prints what it skipped and why.
- A deliberate skip gets a line in `crates/bridge/skip.txt`; the run fails on a skip with no line.
- Run `scripts/check.sh`.

## ADD A UNIT CRATE

- Create `crates/mrly-<unit>` with a `Cargo.toml` like `crates/mrly-core/Cargo.toml` (the workspace fields, `readme`, `include`, the shared `[lints]`), a `//!` summary, a `README.md` and a `LICENSE` link to `../../LICENSE`.
- Pin it in the root `Cargo.toml` `[workspace.dependencies]` at `version = "=X.Y.Z"`, like the other units.
- Add `mrly-<unit>.workspace = true` to `crates/mrly/Cargo.toml`.
- Re-export it in `crates/mrly/src/lib.rs` as `pub use mrly_<unit> as <unit>;` with a `///` line equal to the crate's summary.
- Run `scripts/bridge.sh`: the new top-level module becomes a wasm unit under `bindings/js/units/`, a JS subpath, a Python module and a CLI prefix; the workspace globs and `scripts/wasm.sh` pick it up.
- Hand-written tests go in `bindings/js/<unit>.test.js` and `bindings/py/tests/test_<unit>.py`.
- To draw it on the site, add a loader `site/src/lib/units/<unit>.js` like `font.js` and its entry in `site/src/lib/units/index.js`.
- Run `scripts/check.sh`.

## ADD A HAND TYPE

- A hand type crosses each binding by hand-written code, not by serde.
- `crates/bridge/README.md` has the kinds and the steps; the code lives in `bindings/js/hand/src/lib.rs` and `bindings/py/src/hand.rs`.

## ADD A SITE APP

- One folder under `site/src/apps/<id>/`: `app.json`, `icon.js`, `index.jsx`, plus `scene.js` and `widget.jsx` for a scene app.
- `site/src/apps/README.md` has the contract; `site/README.md` has the build.

## CUT A RELEASE

- `scripts/bump.sh <patch|minor|major|X.Y.Z>` moves the one version, its pins, and regenerates; the new version must be above the old one.
- Commit, run `scripts/check.sh`, push `main`.
- `scripts/publish.sh rs py js --dry`, then `scripts/publish.sh rs py js`; the gate runs once for every target named.
- A real publish fetches tags and refuses a HEAD that is not on `origin/main`, a dirty tree, or a `vX.Y.Z` tag at another commit.
- The semver gate compares the manifest against the highest other `v*` tag and refuses a version too small for the API change.
- The first target to ship tags `vX.Y.Z` and pushes the tag; crates publish in dependency order with `cargo publish --workspace`.
- Wheels and the sdist land in `data/scripts/publish/wheels`; the host wheel is imported in a fresh venv before upload.
- Tokens come from the environment or a gitignored `.env`: `CARGO_REGISTRY_TOKEN`, `UV_PUBLISH_USERNAME` and `UV_PUBLISH_PASSWORD`, `NPM_CONFIG_TOKEN`; only those four names are read, and each reaches only its own publish command.
- `mrly-cli` is new on crates.io, so its first publish needs a token allowed to create crates.
