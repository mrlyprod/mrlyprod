# mrly-math

- The designs: codes, cells, cubes and hexagons, their counts, graphs and names, digit designs on the line, and wheels.
- Released in lockstep with the other `mrly-*` crates; its bindings sit beside it: `js/` (`@mrlyprod/math`), `py/` (`mrly.math`) and `cli/` (`mrly-math`).
- Docs: [docs.rs/mrly-math](https://docs.rs/mrly-math).

```rust
use mrly_math::two;

let carpet = two::carpet(3, 2)?;
assert_eq!((carpet.width(), two::fills(&carpet)), (9, 64));
assert_eq!(mrly_math::bang::bang(2)?.distinct(), 6);
# Ok::<(), mrly_core::Error>(())
```
