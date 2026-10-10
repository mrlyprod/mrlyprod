# MrlyProd

- MrlyMath: the mathematics of designs, and the instruments that measure them.
- One public crate, `pkgs/mrlyrs`, in six modules: `core` the substrate, `num` the integers, `math` the designs in space, `gen` the generator, `life` the engine, `font` the alphabet.
- `mrly` is the crate's bin and the shell RPC: `mrly list` prints every name, `mrly <module.fn> '<json>'` prints the JSON result; `cargo install mrlyrs` installs it.
- `pkgs/mrlypy` on PyPI and `pkgs/mrlyjs` on npm are the bridges: every public function under its Rust name, `mrlypy.math.two.carpet` and `two.carpet` for `mrlyrs::math::two::carpet`.
- `pkgs/bridge/` is the generator: it parses `pkgs/mrlyrs/src` with `syn` into `pkgs/bridge/manifest.json` and writes both bridges from it. Generated files are committed and never hand-edited; a wrong line is a generator fix.
- `pkgs/mrlyjs` ships one wasm per line of `pkgs/bridge/units.txt`, so a page importing `mrlyjs/life` downloads the life wasm and nothing else.
- `scripts/bridge.sh` reruns the generator, `scripts/wasm.sh` builds every wasm and copies the units the site imports into `site/pkg/`, `scripts/publish.sh <rs|py|js>` ships all three at one version.
- `figures/` the site's figures, drawn live in the browser: one `<name>.ts` a figure (`site-*`, `app-*`, `blog-*`) through the pen of `pkgs/mrlyjs/view`; `figures/README.md` has the contract and the time gate.
- `research/` the archive: the research tree, the demos and their figures as of one commit, public and read only; the site, Cargo and Bun ignore it, and `research/README.md` says what it holds.
- `site/` the website at mrly.net: one static shell, one router, one table of routes, every page drawn in the browser; `scripts/` the build, `ui/` the shell and the pages, `kit/` the tools; `site/README.md` says how to run it under Bun.
- `aws/` is the Lambda that ships the site: `net.ts` builds on push, the kit's `site/kit/s3.ts` talks to the bucket.
- `cargo test -p mrlyrs` runs the mathematics; `cargo doc -p mrlyrs --no-deps --open` reads it.
- MIT. This is the way. Why is the secret.
