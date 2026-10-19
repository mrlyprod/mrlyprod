# mrly

- MrlyMath: the mathematics of designs.
- A small integer code picks the filled corners of a cube; that seed grows level by level into a fractal design, which this crate builds, counts, measures and draws.
- Install: `cargo add mrly`.

```rust
use mrly::math::two;

let carpet = two::carpet(3, 2)?;
assert_eq!((carpet.width(), two::fills(&carpet)), (9, 64));
assert_eq!(two::census(&carpet)?.euler, -8);
let svg = two::svg(&carpet, 4, None, 0, two::Shape::Square);
assert!(svg.starts_with("<svg"));
assert!(mrly::num::prime::is_prime(97));
assert_eq!(mrly::font::raster("MRLY").len(), 5);
# Ok::<(), mrly::Error>(())
```

## MODULES

| Module | Holds |
| --- | --- |
| `mrly::core` | The substrate: tensors, cells, colors, images, resampling, seeded chance and the canonical names. |
| `mrly::num` | The integers: primes, divisors, series, sieves, lattices and spectra. |
| `mrly::math` | The designs: codes, cells, cubes and hexagons, their counts, graphs and names, digit designs on the line, and wheels. |
| `mrly::tile` | The tiles: the whole pipeline from a recipe to a file. |
| `mrly::life` | The engine: a rule over a grid, stepped, recorded, measured and rendered. |
| `mrly::font` | The alphabet: a stroked pixel font of 108 characters, painted whole or written stroke by stroke. |

## CLI

- `cargo install mrly-cli` installs `mrly`: every public function, JSON in, JSON out.

```sh
mrly list | grep tile
mrly math.two.carpet '[3, 1]'
mrly --help
```

## ELSEWHERE

- JavaScript and WebAssembly: [@mrlyprod/mrly](https://www.npmjs.com/package/@mrlyprod/mrly) on npm.
- Python: [mrly](https://pypi.org/project/mrly/) on PyPI.
- API docs: [docs.rs/mrly](https://docs.rs/mrly).
- The designs drawn live: [mrly.net](https://mrly.net).
- Source: [github.com/mrlyprod/mrlyprod](https://github.com/mrlyprod/mrlyprod).

## VERSIONS

- 0.x: any minor release may change the API.
- `mrly` re-exports six unit crates: `mrly-core`, `mrly-num`, `mrly-math`, `mrly-tile`, `mrly-life`, `mrly-font`.
- They release together with `mrly` and `mrly-cli` at one version; depend on `mrly`, not on the units.
- MIT licensed.
