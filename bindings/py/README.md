# mrly

- MrlyMath, the mathematics of designs, compiled from Rust for Python.
- Every public function of the Rust crate [mrly](https://crates.io/crates/mrly), under its Rust name.
- Install: `pip install mrly` (or `uv add mrly`).

## USE

```python
from mrly.math.two import carpet, census, svg

cell = carpet(3, 2)
cell["types"].shape
census(cell)["euler"]
open("carpet.svg", "w").write(svg(cell, 4, None, 0, "Square"))
```

- Six modules: `mrly.core`, `mrly.num`, `mrly.math`, `mrly.tile`, `mrly.life`, `mrly.font`.
- Names mirror Rust: `mrly::math::two::carpet` is `mrly.math.two.carpet`.
- Type stubs ship with the wheel (`py.typed`).

## DATA

- A tensor is a numpy array of `uint8`, `uint16`, `uint32` or `int32`, C-contiguous.
- A cell is a dict `{"types", "colors", "tags"}`: `types` shaped `(h, w)` or `(d, h, w)`, `colors` a `uint8` array of that shape plus a trailing `4`, or `None`, `tags` an array or `None`.
- A hex cell is a dict `{"cell", "projection", "orientation", "start"}`.
- An image is a `uint8` array `(h, w, 4)`; a pixel list is `(n, 4)`.
- A color is a tuple `(r, g, b, a)`.
- Structs come out as dicts, enums as strings, big integers as Python ints.
- `Rng(seed)` is the seeded stream; a function that takes one advances it in place.
- A Rust error raises `ValueError`; an overflow raises `OverflowError`.

## PLATFORMS

- Python 3.10 or newer.
- Wheels: macOS arm64, Linux x86_64 and Linux aarch64.
- Anywhere else pip builds the sdist, which needs a Rust toolchain.

## LINKS

- What each function does: [docs.rs/mrly](https://docs.rs/mrly); the Python path is the Rust path after `mrly::`.
- JavaScript: [@mrlyprod/mrly](https://www.npmjs.com/package/@mrlyprod/mrly) on npm.
- Live: [mrly.net](https://mrly.net).
- Source: [github.com/mrlyprod/mrlyprod](https://github.com/mrlyprod/mrlyprod).
- 0.x: any minor release may change the API. MIT licensed.
