# design-recursion

- The design meter `M_F(x) = sum of mu(n) over n in S_F, n <= x` at base 3 `{0,1}` to `3^27`, beside `A(x)`, the same sum over the elements coprime to `3`, and the recursion of [the Franel note](../../../notes/identity-and-recursion.md)'s section The recursion in the base, `M_F(x) = A(x) - A(x/3)`, asserted exactly at every `3^n`.
- Prints, at every `n`, `M_F(3^n)`, `A(3^n)`, the running maximum `max abs M_F(y)` over `y <= 3^n`, the reading `log max / log 3^n` and the same over `log 2^n`, the least-squares slope of `log max` on `log x` over the five rungs `n-4..n` with the bracket `[min, max]` of the reading over them, and `abs A(3^n)/(n + 1)`, the floor the recursion puts under `max_(j <= n) abs M_F(3^j)`.

## METHOD

- The `k`-th element of `S_F` is the base 3 reading of the binary digits of `k`, so the walk over `k = 1..2^n` is ascending and the unit digit is the parity of `k`: `A` sums the odd `k`.
- `mu` by trial division over the primes up to the least `b` with `b^3 > 3^level` (`6562` at level `24`, `19684` at `27`): the cofactor has no prime factor below `b` and is at most `3^level < b^3`, so it is `1`, a prime, a prime square or a product of two distinct primes; below `b^2` it is prime, above it an integer square root tells `p^2` and Miller-Rabin on the twelve witnesses `2..37` tells `p` from `pq`. No Pollard rho is needed.
- Four threads over chunks of `2^16` consecutive `k`; a chunk returns its sum, its sum over odd `k` and the minimum and maximum of its local prefix, and the sequential merge recovers the exact running maximum since `abs(offset + prefix)` over a chunk peaks at an endpoint of its prefix range.
- Self-checks at every run: `mu` against a linear Mobius sieve on all `2^15 - 1` elements below `3^15`, and the recursion `M_F(3^n) = A(3^n) - A(3^(n-1))` with `A(3^n) = sum_(j <= n) M_F(3^j)` at every `n`, both asserted.

## RUN

- `cargo run --release --manifest-path research/franel/lab/rs/design-recursion/Cargo.toml` from the repository root walks to `3^24` in `5.5 s` on four threads; `-- 27` walks to `3^27` in `73 s`. Prints only, writes nothing.
- `cargo test --release --manifest-path research/franel/lab/rs/design-recursion/Cargo.toml`: nine tests, `mu` against the sieve to `200000`, the two-large-prime cofactors, four strong pseudoprimes rejected, the integer square root, the trial bound, the ascending enumeration, the parallel walk against a sequential one at `3^18`, the recursion to `3^16`, and `M_F(3^n)` with its running maximum against the `mobius` census ([mobius-designs](../../../../mobius/lab/rs/mobius-designs/)) at its rungs `4` to `16`.

## WITNESSES

- The Franel note, The recursion in the base: every number of its Verified paragraph, `M_F(3^n)`, `A(3^n)` and the running maximum at `n = 12..27`, the readings `0.311823, 0.307223, 0.301266` at `n = 12, 24, 27`, the bracket `[0.2995, 0.3148]` at `n = 27`.
- The `mobius` census agrees at all thirteen of its rungs `3^4..3^24`, `M_(0,1)(3^24) = -1886` with running maximum `3296` among them; PARI (`moebius` on the literal elements, and `A` as the literal shifted sum over `3w + 1`, `w` a padded string) agrees at every `n <= 14`.
- The three claims rows on the recursion in the base in [franel-on-a-digit-design](../../../claims/franel-on-a-digit-design.md).
