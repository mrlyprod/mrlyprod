# mrly-num

- The integers: primes, divisors, series, sieves, lattices and spectra.
- Released in lockstep with the other `mrly-*` crates; its bindings sit beside it: `js/` (`@mrlyprod/num`), `py/` (`mrly.num`) and `cli/` (`mrly-num`).
- Docs: [docs.rs/mrly-num](https://docs.rs/mrly-num).

```rust
assert!(mrly_num::prime::is_prime(97));
assert_eq!(mrly_num::factor::divisors(12), vec![1, 2, 3, 4, 6, 12]);
```
