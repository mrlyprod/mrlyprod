# mrly-tile

- The tiles: the whole pipeline from a recipe to a file.
- Released in lockstep with the other `mrly-*` crates; its bindings sit beside it: `js/` (`@mrlyprod/tile`), `py/` (`mrly.tile`) and `cli/` (`mrly-tile`).
- Docs: [docs.rs/mrly-tile](https://docs.rs/mrly-tile).

```rust
let image = mrly_tile::background(1, 2, 2)?;
assert_eq!((image.width, image.height), (18, 18));
# Ok::<(), mrly_core::Error>(())
```
