# mrly-num

- The integers: primes, divisors, series, sieves, lattices and spectra.
- Part of [mrly](https://crates.io/crates/mrly), released in lockstep with it; most users depend on `mrly` and reach this crate as `mrly::num`.
- Docs: [docs.rs/mrly-num](https://docs.rs/mrly-num).

```rust
assert!(mrly_num::prime::is_prime(97));
assert_eq!(mrly_num::factor::divisors(12), vec![1, 2, 3, 4, 6, 12]);
```
