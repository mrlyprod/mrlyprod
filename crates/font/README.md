# mrly-font

- The alphabet: a stroked pixel font of 108 characters, painted whole or written stroke by stroke.
- Part of [mrly](https://crates.io/crates/mrly), released in lockstep with it; most users depend on `mrly` and reach this crate as `mrly::font`.
- Docs: [docs.rs/mrly-font](https://docs.rs/mrly-font).

```rust
assert_eq!(mrly_font::supported().len(), 108);
assert_eq!(mrly_font::glyph('a').unwrap().rows[0], "01110");
assert_eq!(mrly_font::raster("42").len(), 5);
```
