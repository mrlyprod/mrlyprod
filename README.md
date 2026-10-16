# MrlyProd

- MrlyMath: the mathematics of designs, and the instruments that measure them.
- One public crate, `crates/mrly`, in six modules: `core` the substrate, `num` the integers, `math` the designs in space, `tile` the generator, `life` the engine, `font` the alphabet.
- `mrly` is the crate's bin and the shell RPC: `mrly list` prints every name, `mrly <module.fn> '<json>'` prints the JSON result; `cargo install mrly` installs it.
- `bindings/py` on PyPI and `bindings/js` on npm are the bridges: every public function under its Rust name, `mrly.math.two.carpet` and `two.carpet` for `mrly::math::two::carpet`.
- `crates/bridge/` is the generator: it parses `crates/mrly/src` with `syn` into `crates/bridge/manifest.json` and writes both bridges from it. Generated files are committed and never hand-edited; a wrong line is a generator fix.
- `bindings/js` ships one wasm per unit crate (`crates/mrly-<unit>`), plus the root bundle, so a page importing `@mrlyprod/mrly/life` downloads the life wasm and nothing else.
- `scripts/bridge.sh` reruns the generator, `scripts/wasm.sh` builds every wasm and copies the units the site imports into `site/pkg/`, `scripts/publish.sh <rs|py|js>` ships all three at one version.
- `research/` the research, one folder a topic (`README.md`, `notes/`, `claims/`, `papers/`, `refs.md`, `lab/`, `figures/`, drawn live as its gallery): served at `/research/`, read at `/git/research/` and `/raw/research/`; `research/README.md` lists the topics.
- `site/` the website at mrly.net: one static shell, one router, one table of routes, every page drawn in the browser; `apps/<id>/` one folder a route with its page and its figure `icon.js`, `scripts/` the build, `ui/` the shell, `kit/` the tools; `site/README.md` has the unit, the figure contract and how to run it under Bun.
- `aws/` is the Lambda that ships the site: `net.ts` builds on push, the kit's `site/kit/s3.ts` talks to the bucket.
- `cargo test -p mrly` runs the mathematics; `cargo doc -p mrly --no-deps --open` reads it.
- MIT. This is the way. Why is the secret.
