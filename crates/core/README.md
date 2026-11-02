# mrly-core

- The substrate: tensors, cells, colors, images, resampling, seeded chance and the canonical names.
- Released in lockstep with the other `mrly-*` crates; its bindings sit beside it: `js/` (`@mrlyprod/core`), `py/` (`mrly.core`) and `cli/` (`mrly-core`).
- Docs: [docs.rs/mrly-core](https://docs.rs/mrly-core).

```rust
use mrly_core::{Color, Tensor};

assert_eq!(Tensor::of(vec![1, 0, 0, 1], vec![2, 2])?.sum(), 2);
assert_eq!(Color::from_hex("#008cff80")?.to_hex(), "#008cff80");
# Ok::<(), mrly_core::Error>(())
```
