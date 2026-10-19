# mrly-core

- The substrate: tensors, cells, colors, images, resampling, seeded chance and the canonical names.
- Part of [mrly](https://crates.io/crates/mrly), released in lockstep with it; most users depend on `mrly` and reach this crate as `mrly::core`.
- Docs: [docs.rs/mrly-core](https://docs.rs/mrly-core).

```rust
use mrly_core::{Color, Tensor};

assert_eq!(Tensor::of(vec![1, 0, 0, 1], vec![2, 2])?.sum(), 2);
assert_eq!(Color::from_hex("#008cff80")?.to_hex(), "#008cff80");
# Ok::<(), mrly_core::Error>(())
```
