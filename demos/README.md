# demos

- The eyes of MrlyMath: browser pages that draw what the crates compute through `mrlyweb` and wasm.
- Rust is the only math; the pages only draw. Three.js is the one JavaScript dependency.
- `bun install` fetches it; `bun run wasm` builds `pkg/` from `crates/mrlyweb` with wasm-pack.
- `bun run dev` serves every page at `localhost:3000`, routed without `.html`; `bun run build` writes the static site to `dist/`.
- `bun run check` prints the fixture numbers the crate's host test asserts; both must agree.
- `pkg/`, `dist/` and `node_modules/` are build output and stay out of git.
