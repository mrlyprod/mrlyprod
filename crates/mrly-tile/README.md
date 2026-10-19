# mrly-tile

- The tiles: the whole pipeline from a recipe to a file.
- Part of [mrly](https://crates.io/crates/mrly), released in lockstep with it; most users depend on `mrly` and reach this crate as `mrly::tile`.
- Docs: [docs.rs/mrly-tile](https://docs.rs/mrly-tile).

```rust
let image = mrly_tile::background(1, 2, 2)?;
assert_eq!((image.width, image.height), (18, 18));
# Ok::<(), mrly_core::Error>(())
```
