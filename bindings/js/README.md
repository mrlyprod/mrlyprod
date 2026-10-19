# @mrlyprod/mrly

- MrlyMath, the mathematics of designs, compiled from Rust to WebAssembly.
- Every public function of the Rust crate [mrly](https://crates.io/crates/mrly), under its Rust name.
- Install: `npm install @mrlyprod/mrly` (or `bun add`, `pnpm add`).

## USE

```js
import init, { two } from "@mrlyprod/mrly/math";

await init();
const carpet = two.carpet(3, 2);
carpet.shape;
two.census(carpet).euler;
two.svg(carpet, 4, null, 0, "Square");
```

- Six subpaths, one per module: `@mrlyprod/mrly/core`, `/num`, `/math`, `/tile`, `/life`, `/font`.
- Each subpath is its own wasm, so a page that imports `/font` downloads the font and nothing else.
- The root `@mrlyprod/mrly` re-exports all six as namespaces (`core`, `num`, `math`, `tile`, `life`, `font`), and its `init()` loads all six.

## LOADING

- Browser and bundlers: call `await init()` from the subpath once before the first call.
- Node: the `node` export condition reads the wasm from disk at import, so it works out of the box.
- Custom loading: `initSync({ module })` takes the wasm bytes or a compiled `WebAssembly.Module`; the files sit at `@mrlyprod/mrly/pkg/<unit>/mrly_js_<unit>_bg.wasm`.

## INSTANCES

- Each subpath is its own wasm instance with its own memory.
- Plain data crosses between subpaths: a cell from `/math` can go into `/life`.
- Class objects (`Rng`, `Rule`, `Glyph` and the rest) never cross: each subpath exports its own `Rng`, so hand a function the `Rng` of its own subpath.

## NAMES AND DATA

- Names are snake_case and mirror Rust: `mrly::math::spectrum::laplacian_spectrum` is `spectrum.laplacian_spectrum` from `/math`.
- A name that is a JS reserved word takes a trailing `_`: `core.tensor.new_`.
- A tensor is `{ shape, data }`, `data` a `Uint8Array`, `Uint16Array`, `Uint32Array` or `Int32Array`.
- A cell is `{ shape, types, colors?, tags? }`; `colors` is flat rgba bytes.
- An image is `{ shape: [height, width], colors: Uint8Array }`, four bytes a pixel.
- A color is `[r, g, b, a]`.
- A `u64` or `i64` comes out as a `BigInt`; a `u128` crosses as a decimal string.
- Structs and enums cross as plain objects and strings, as serde writes them.
- A Rust error is thrown as an `Error` with the Rust message.

## TYPES

- Every subpath ships a `.d.ts`.
- The generated glue uses `Symbol.dispose`: set `"lib": ["esnext"]` in `tsconfig.json`, or `"skipLibCheck": true`.

## LINKS

- What each function does: [docs.rs/mrly](https://docs.rs/mrly); the JS name is the Rust path after `mrly::`.
- Python: [mrly](https://pypi.org/project/mrly/) on PyPI.
- Live: [mrly.net](https://mrly.net).
- Source: [github.com/mrlyprod/mrlyprod](https://github.com/mrlyprod/mrlyprod).
- 0.x: any minor release may change the API. MIT licensed.
