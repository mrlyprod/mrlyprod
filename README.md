# MrlyProd

- MrlyMath: the mathematics of designs, and the instruments that measure them.
- One Rust library, bridged to JavaScript, Python and the shell, and the site that draws it at [mrly.net](https://mrly.net).

## INSTALL

- Rust: `cargo add mrly` ([crates.io](https://crates.io/crates/mrly), [docs.rs](https://docs.rs/mrly)).
- JavaScript: `npm install @mrlyprod/mrly` ([npm](https://www.npmjs.com/package/@mrlyprod/mrly)).
- Python: `pip install mrly` ([PyPI](https://pypi.org/project/mrly/)).
- Shell: `cargo install mrly-cli` installs `mrly`.

## TREE

- `crates/mrly`: the facade, the crate users depend on; it re-exports the six units as modules.
- `crates/mrly-{core,num,math,tile,life,font}`: the six unit crates, one per module, released with `mrly` at one version.
- `crates/bridge`: the generator, never published; it parses the crates into `crates/bridge/manifest.json` and writes every binding from it.
- `bindings/js`: `@mrlyprod/mrly` on npm, one wasm per unit, one subpath per module.
- `bindings/py`: `mrly` on PyPI, one native module, numpy in and out.
- `bindings/cli`: `mrly-cli` on crates.io, the `mrly` binary, JSON in and out.
- `site/`: mrly.net, every page drawn in the browser from the wasm units; `site/README.md` has the rest.
- `research/`: the research, one folder a topic, served at [mrly.net/research](https://mrly.net/research/); `research/README.md` lists the topics.
- `aws/`: the Lambda that rebuilds the site on push.
- `pyproject.toml`: the research Python env, `mrly-research`, never published.
- `scripts/`: `check.sh` the gate, `bridge.sh` the generator, `wasm.sh` the wasm build, `bump.sh` the version, `publish.sh` the release, `cargo.sh` the cargo lock the others share.

## RULES

- `scripts/check.sh` is the one gate.
- Generated files are committed and never hand-edited; a wrong line is a generator fix, then `scripts/bridge.sh`.
- `.gitattributes` lists every file the bridge writes; the site's `*.gen.*` files come from its own tools.
- One version for every package, in the root `Cargo.toml`; `scripts/bump.sh` moves it.
- `CONTRIBUTING.md` has the recipes.
- MIT.
