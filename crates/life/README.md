# mrly-life

- The engine: a rule over a grid, stepped, recorded, measured and rendered.
- Released in lockstep with the other `mrly-*` crates; its bindings sit beside it: `js/` (`@mrlyprod/life`), `py/` (`mrly.life`) and `cli/` (`mrly-life`).
- Docs: [docs.rs/mrly-life](https://docs.rs/mrly-life).

```rust
use mrly_core::tensor::Tensor;
use mrly_life::{Config, Fate, animate, moore};
use mrly_math::two::Cell2d;

let bar = Cell2d::new(Tensor::of(vec![0, 0, 0, 1, 1, 1, 0, 0, 0], vec![3, 3])?)?;
let life = animate(&bar, &Config::new(moore()?, vec![3].into(), vec![2, 3].into()))?;
assert_eq!(life.fate, Fate::Loop);
# Ok::<(), mrly_core::Error>(())
```
