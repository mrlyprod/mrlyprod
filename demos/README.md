# demos

- The eyes of MrlyMath: browser pages that draw what the crates compute through `mrlyweb` and wasm.
- Rust is the only math; the pages only draw. Three.js is the one JavaScript dependency.
- `bun install` fetches it; `bun run wasm` builds `pkg/` from `crates/mrlyweb` with wasm-pack.
- One page is one folder: `<name>/index.html` and `<name>/index.js`; the gallery is `index.html` at the root.
- `lib/` holds the shared code: `mrly.js`, `stage.js`, `chart.js`, `query.js`, `ramp.js`, `select.js`, `mrly.css`.
- `select.js` is the one picker: design list, code, base and Randomize; `?seed=7` replays the seventh tap, and a typed code drops the seed.
- `bun run dev` serves every page at `localhost:3000` as `/<name>`; `bun run build` writes the static site to `dist/`.
- `bun run check` prints the fixture numbers the crate's host test asserts; both must agree.
- `pkg/`, `dist/` and `node_modules/` are build output and stay out of git.
