# mobius-designs

- Regenerates every number on [mobius](../../mobius.md): the Mobius meter `M_F(q^L) = sum of mu(n)` over the digit-restricted set `S_F` up to `q^L`, measured against the set's own counting function `A_F(q^L)`.
- The census covers every digit set `F` with `2 <= |F| <= q - 1` at `q = 3, 4, 5`, the full-set control column at `q = 3, 4, 5, 10` (the Mertens function), and the ten base-10 Kempner columns that exclude one digit.
- Restricted families are enumerated in ascending numeric order, length by length, and `mu` is computed by exact factorization: trial division below 1024, deterministic Miller-Rabin on the twelve witnesses `2..37`, Pollard-Brent rho on the survivors; every integer is exact, no floats touch a count.
- Control and Kempner columns come from a linear Euler sieve for `mu` to `3^17 = 129140163`; one family (`q = 3`, `F = {1,2}`, `L = 16`) is computed by both methods and asserted equal at every level.
- The digit-scaling identities are asserted, not assumed: every census family whose digits share a factor `a > 1` is recomputed as the `a`-twist of its primitive family and asserted equal at every level, and the count identity `A_(aF')(q^l) = A_(F')(q^l) - 1` when `{0,1}` is inside `F'` (else equal) is asserted with it.
- Depths per family class: `L = 24` at `(q,k) = (3,2)`, `22` at `(4,2)`, `14` at `(4,3)`, `21` at `(5,2)`, `13` at `(5,3)`, `11` at `(5,4)`, `8` at base 10; controls run to `3^17`, `4^13`, `5^11`, `10^8`.
- Prints one `row` line per family per level with exact `A`, `M`, the cut exponent `theta = log|M|/log A`, the running maximum `Mmax = max |M_F(x)|` over `x <= q^l`, and `thetamax = log(Mmax)/log A`; one `slope` line per family with the final `theta`, `thetamax` and the drift of `thetamax` over the last five levels; one `distribution` line per base sorted by the generator itself; one `band` line with the census-wide extremes of `thetamax`, its deviation from `1/2`, the drifts, the cut exponents, and the controls.

## RUN

- `CARGO_BUILD_JOBS=4 cargo run --release --manifest-path research/lab/Cargo.toml -p mobius-designs`
- About five minutes on six threads; prints only, writes nothing.
- `cargo test -p mobius-designs` pins `mu` against the sieve to 50000, the Miller-Rabin witnesses against six strong pseudoprimes, the two meter methods against each other, the scaled-family identities at depth 8, the counting closed forms, the ascending enumeration order, the running maximum against a prefix recount, and the Mertens anchors `-1, 1, 2, -23, -48` at `10^1..10^5`.

## WITNESSES

- mobius.md the base-3 series table (`M_{0,1}(3^24) = -1886`, running max `3296`), the final-checkpoint table of all 38 families, the Kempner table at `10^8`, and every `thetamax` and drift; the page tables are extracted by script from this generator's printed rows.
- mobius.md the identity checks: the eight scaled families equal their primitive twists at every level, `M` vanishes identically on `F = {0,4}` at `q = 5`, and the base-4 pair reads `34/-34` with shared `Mmax = 1553` at `L = 22`.
- mobius.md the controls: `M(3^17) = -1423`, `M(4^13) = 329`, `M(5^11) = 617`, and `-1, 1, 2, -23, -48, 212, 1037, 1928` at `10^1..10^8`, which is A084237.
- DISCOVERIES.md the digit-restricted Mobius meter and exponent rows.
