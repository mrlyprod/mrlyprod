# mrly-sequences

- The sequence rows: a design sequence's address, its measure and axis, its terms and its status.
- Rust only for now; no bindings sit beside it yet.

```rust
use mrly_sequences::{Axis, Cost, Key, Measure};

let carpet = Key::new(7, 2, 2, Measure::Fills, Axis::Side);
assert_eq!(carpet.number(), 3);
assert_eq!(carpet.measure.cost(carpet.axis), Cost::Closed);
```
