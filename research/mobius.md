# The Mobius meter across digit designs

Fix a base `q >= 3` and a digit set `F` inside `{0..q-1}` with `k = |F| >= 2`. The digit-restricted set `S_F` holds the positive integers whose base-`q` digits all lie in `F`, with no leading zero: a one-dimensional digit design, the same restriction rule that carves every fractal in this tree, read on the integer line instead of the square. This page measures how much the Mobius function cancels along each design, against the design's own size - and proves that the columns are not independent: digit sets that are scalar multiples of each other carry exactly transferred meters, including one family whose meter vanishes identically and one base-4 pair locked in exact anti-symmetry. Every number is printed by [lab/mobius-designs](lab/mobius-designs/).

Tags as everywhere in this tree: **Proved** means derived here from definitions, **Verified** means recomputed exactly and checked against an independent path, **Conjecture** is labelled belief.

## The meter and its yardstick

- `A_F(x)` counts `S_F` up to `x`. The count is exact at every checkpoint: `A_F(q^L) = k^L - 1` when `0 in F` (plus 1 when `1 in F` too, for the boundary element `q^L` itself), and `A_F(q^L) = (k^(L+1) - k)/(k - 1)` when `0` is not in `F`, by counting digit strings of each length. **Proved**; the lane's tests pin it against direct enumeration. Between checkpoints `A_F(x)/x^(log_q k)` carries the log-periodic ripple every design in this tree carries - the classical fluctuation of digital sums ([Flajolet, Grabner, Kirschenhofer, Prodinger and Tichy 1994](https://doi.org/10.1016/0304-3975(92)00065-Y)) - so a checkpoint value is a grid value, never a constant.
- The meter is `M_F(x) = sum of mu(n)` over `n in S_F`, `n <= x`, and the exponent is `theta(F) = limsup of log|M_F(x)| / log A_F(x)`. A single cut of `|M_F|` is a bad estimator - the meter crosses zero freely - so the census prints two readings per level: `M_F(q^L)` itself, and the running maximum `max of |M_F(x)|` over `x <= q^L`, whose exponent `thetamax` is monotone in the numerator and is the estimator the slope tables use.
- The yardstick matters. `A_F(x)` grows like `x^(log_q k)`, so `S_F` is sparse, and a bound of shape `o(x)` is weaker than the trivial `|M_F(x)| <= A_F(x)`. The indicator of `S_F` is a `q`-automatic sequence, so [Mullner 2017](https://doi.org/10.1215/00127094-2017-0024) (automatic sequences fulfill the Sarnak conjecture) gives `M_F(x) = o(x)` for every `F`: orthogonality holds and the question is well-posed, but against the set's own mass that bound says nothing at all. The same shape repeats in base 2 through circuits: the indicator is computable in bounded depth from the binary digits, so [Green 2012](https://arxiv.org/abs/1103.4991) also gives `o(x)`, again below the trivial bound. **Verified** against the literature. The honest question is `theta`, and it is open at every `2 <= k <= q - 1`.
- The Dirichlet series over `S_F` is built territory, and this page claims nothing about it: the abscissa is `log_q k` ([Kohler and Spilker 2009](https://doi.org/10.1007/s00591-009-0059-5), with position-varying digit rules in [Nathanson 2021](https://arxiv.org/abs/2010.06295)); the series continues meromorphically to `C` with simple poles among `s = log_q k - m + 2 pi i j / log q` (the automatic-series mechanism of [Allouche, Mendes France and Peyriere 2000](https://doi.org/10.1006/jnth.1999.2487), carried out for missing digits in [Burnol 2026](https://arxiv.org/abs/2602.19727) and unified in [Allouche, Shallit and Stipulanti 2025](https://arxiv.org/abs/2401.13524)); a pole lattice of period `2 pi i / log q` reads as log-periodic oscillation through the Mellin dictionary of [Flajolet, Gourdon and Dumas 1994](https://inria.hal.science/inria-00074307), and the oscillation is visible in the series' own numerical moments ([Burnol 2026 oscillations](https://arxiv.org/abs/2604.24754)); the Mobius function itself is not `k`-automatic for any `k`, so the Mobius-weighted series inherits none of that continuation ([Coons 2010](https://doi.org/10.5802/jtnb.718)); and no Mobius or Mertens sum appears anywhere in that literature. **Verified** against the sources in REFS.md. The series does not carry the meter the way zeta carries Mertens: `S_F` is not multiplicatively closed - at `q = 3`, `F = {0,1}`, both `4 = 11` and `13 = 111` lie in `S_F` while `4 x 13 = 52 = 1221` does not - so there is no Euler product and `M_F` is not the coefficient sum of an inverse series. **Proved** by that witness.
- The full digit set is the classical boundary. `S_F` is then every integer, `M_F` is the Mertens function of [Mertens 1897](https://www.zobodat.at/pdf/SBAWW_106_2a_0761-0830.pdf), and `M(x) = O(x^(1/2 + eps))` for every `eps > 0` is equivalent to the Riemann hypothesis ([Titchmarsh 1986](https://sites.math.rutgers.edu/~zeilberg/EM18/TitchmarshZeta.pdf), Theorem 14.25 (C)), while `limsup |M(x)|/sqrt(x) >= 1.06` unconditionally by [Odlyzko and te Riele 1985](https://doi.org/10.1515/crll.1985.357.138), so the exponent over all `x` equals `1/2` exactly when RH holds. **Verified** against the literature. This page claims nothing about RH: the full-set column below is a control rendered for scale, and the tree's own claims live in the restricted columns.

## The exact transfer between designs

The census columns are tied together by one carry-free mechanism. **Proved:**

- **Scaling.** If every digit of `F` is `a` times a digit of `F'`, so `F = aF'` inside `{0..q-1}`, then `m -> am` maps `S_F'` bijectively onto `S_F` preserving digit length: `am = sum (a d_j) q^j` and each `a d_j <= q - 1`, so no carry occurs and the digit string scales digitwise. Hence `A_F(q^L)` equals the string count of `F'` at the same depth, and `M_F(q^L) = sum of mu(am)` over `m in S_F'` with at most `L` digits.
- **Vanishing.** If `a` has a square factor then `mu(am) = 0` for every `m`, so `M_F` is identically zero: at `q = 5`, `F = {0,4} = 4 x {0,1}`, the meter reads 0 at all 21 levels. A census that reads cancellation without factoring out the digit gcd reads this as infinite cancellation; the digit gcd must be squarefree before `theta` means anything.
- **Prime twist.** If `a = p` is prime then `mu(pm)` is `-mu(m)` on `p`-free `m` and `0` otherwise, so `M_(pF')(q^L) = -sum of mu(m)` over the `m in S_F'` not divisible by `p`. At `q = 3`, `F' = {0,1}`: an element `m = sum of 3^j` is odd exactly when its count of 1-digits is odd, and reading the digit string as a binary index that parity is the Thue-Morse sign, so the `{0,2}` column is the Thue-Morse-twisted `{0,1}` column.
- **Base-4 anti-symmetry.** At `q = 4`, `M_{0,2}(4^L) = -M_{0,1}(4^L)` exactly: since `4 | q`, an element of `S_{0,1}` is `0` or `1 mod 4` by its unit digit, so every even element is divisible by 4 and carries `mu = 0`, and the odd-part twist above is minus the whole meter. Stronger, `M_{0,2}(x) = -M_{0,1}(x/2)` at every real `x`, and since `S_{0,1}` has no element strictly between `(4^L - 1)/3` and `4^L` the running maxima agree level by level as well. The census confirms both at all 22 levels, e.g. meters `-110/110` at `L = 15`, `-342/342` at `L = 17`, `34/-34` at `L = 22`, and `Mmax = 1553` for both at `L = 22`.

**Verified:** the generator recomputes all eight scaled census families (`{0,2}` at `q = 3`; `{0,2}, {0,3}` at `q = 4`; `{0,2}, {0,3}, {0,4}, {2,4}, {0,2,4}` at `q = 5`) from their primitive families through `mu(am)` and asserts equality at every level. The mechanism needs a common digit factor, so it partitions the census into primitive columns and their twists and says nothing across primitive columns.

## The census

Every `M_F(q^L)` below is an exact integer: restricted families enumerated in ascending order with `mu` from deterministic factorization (trial division, Miller-Rabin on the twelve witnesses `2..37`, Pollard rho), controls by a linear Mobius sieve; one family (`q = 3`, `F = {1,2}`, `L = 16`) is computed by both methods and asserted equal at every level. The base-10 control reproduces [A084237](https://oeis.org/A084237) (`-1, 1, 2, -23, -48, 212, 1037, 1928` at `10^1..10^8`). Every table below is extracted by script from the generator's printed rows, never assembled by hand. **Verified.**

The three base-3 columns, checkpoint meter and running maximum `Mmax = max of |M_F(x)|` over `x <= 3^L` per row:

| `L` | `M_{0,1}` | max | `M_{0,2}` | max | `M_{1,2}` | max |
|---|---|---|---|---|---|---|
| 4 | -2 | 3 | 2 | 3 | -8 | 8 |
| 6 | 2 | 5 | 0 | 3 | -8 | 11 |
| 8 | 2 | 8 | 3 | 7 | -31 | 33 |
| 10 | 5 | 13 | 0 | 11 | -14 | 38 |
| 12 | 56 | 61 | -37 | 40 | -35 | 88 |
| 14 | 11 | 105 | -10 | 67 | -205 | 230 |
| 16 | 149 | 173 | -124 | 152 | 4 | 281 |
| 18 | -30 | 312 | 67 | 249 | -1461 | 1582 |
| 20 | 496 | 539 | -382 | 485 | -3175 | 3255 |
| 21 | 533 | 866 | -194 | 617 | -2005 | 3855 |
| 22 | 1009 | 1089 | -1205 | 1324 | -690 | 3855 |
| 23 | 1824 | 2848 | -2242 | 2942 | -3214 | 3855 |
| 24 | -1886 | 3296 | -133 | 3843 | -3248 | 4113 |

The final checkpoint of every family, with `thetamax = log(Mmax)/log A` and its drift (max minus min) over the last five levels:

| `q` | `F` | `L` | `A_F(q^L)` | `M_F(q^L)` | `Mmax` | `thetamax` | drift |
|---|---|---|---|---|---|---|---|
| 3 | 01 | 24 | 16777216 | -1886 | 3296 | 0.4869 | 0.0452 |
| 3 | 02 | 24 | 16777215 | -133 | 3843 | 0.4962 | 0.0596 |
| 3 | 12 | 24 | 33554430 | -3248 | 4113 | 0.4802 | 0.0754 |
| 4 | 01 | 22 | 4194304 | 34 | 1553 | 0.4819 | 0.0391 |
| 4 | 02 | 22 | 4194303 | -34 | 1553 | 0.4819 | 0.0391 |
| 4 | 03 | 22 | 4194303 | -541 | 1180 | 0.4638 | 0.0488 |
| 4 | 12 | 22 | 8388606 | -855 | 3965 | 0.5197 | 0.1056 |
| 4 | 13 | 22 | 8388606 | -712 | 2631 | 0.4940 | 0.0727 |
| 4 | 23 | 22 | 8388606 | -3255 | 3258 | 0.5074 | 0.0157 |
| 4 | 012 | 14 | 4782969 | -503 | 1057 | 0.4527 | 0.0475 |
| 4 | 013 | 14 | 4782969 | 2313 | 2899 | 0.5183 | 0.0487 |
| 4 | 023 | 14 | 4782968 | -753 | 1166 | 0.4591 | 0.0862 |
| 4 | 123 | 14 | 7174452 | -592 | 1644 | 0.4691 | 0.0729 |
| 5 | 01 | 21 | 2097152 | 153 | 849 | 0.4633 | 0.0485 |
| 5 | 02 | 21 | 2097151 | 250 | 889 | 0.4665 | 0.0268 |
| 5 | 03 | 21 | 2097151 | -116 | 700 | 0.4501 | 0.0311 |
| 5 | 04 | 21 | 2097151 | 0 | 0 | - | - |
| 5 | 12 | 21 | 4194302 | -128 | 1643 | 0.4856 | 0.0732 |
| 5 | 13 | 21 | 4194302 | -2875 | 3533 | 0.5358 | 0.0456 |
| 5 | 14 | 21 | 4194302 | -1511 | 2750 | 0.5193 | 0.0640 |
| 5 | 23 | 21 | 4194302 | 405 | 914 | 0.4471 | 0.0581 |
| 5 | 24 | 21 | 4194302 | 1065 | 2287 | 0.5072 | 0.0540 |
| 5 | 34 | 21 | 4194302 | -2137 | 2538 | 0.5141 | 0.0401 |
| 5 | 012 | 13 | 1594323 | -1016 | 1416 | 0.5080 | 0.0846 |
| 5 | 013 | 13 | 1594323 | -137 | 768 | 0.4652 | 0.0528 |
| 5 | 014 | 13 | 1594323 | 213 | 1005 | 0.4840 | 0.0821 |
| 5 | 023 | 13 | 1594322 | 759 | 858 | 0.4729 | 0.0455 |
| 5 | 024 | 13 | 1594322 | 686 | 959 | 0.4807 | 0.0240 |
| 5 | 034 | 13 | 1594322 | 501 | 1000 | 0.4837 | 0.0847 |
| 5 | 123 | 13 | 2391483 | 88 | 816 | 0.4565 | 0.0489 |
| 5 | 124 | 13 | 2391483 | 1036 | 1613 | 0.5029 | 0.0604 |
| 5 | 134 | 13 | 2391483 | -725 | 981 | 0.4690 | 0.0519 |
| 5 | 234 | 13 | 2391483 | -1926 | 2021 | 0.5182 | 0.1056 |
| 5 | 0123 | 11 | 4194304 | -474 | 1725 | 0.4887 | 0.0732 |
| 5 | 0124 | 11 | 4194304 | 426 | 1494 | 0.4793 | 0.0673 |
| 5 | 0134 | 11 | 4194304 | -644 | 1633 | 0.4852 | 0.0222 |
| 5 | 0234 | 11 | 4194303 | -362 | 2179 | 0.5041 | 0.0794 |
| 5 | 1234 | 11 | 5592404 | 145 | 1101 | 0.4508 | 0.0996 |

Base 10 with one digit excluded, the Kempner designs (`k = 9`, the sets behind the convergent harmonic series of [Kempner 1914](https://doi.org/10.2307/2972074), revisited at `s = 1` in [Allouche, Hu and Morin 2024](https://arxiv.org/abs/2403.05678)), at `x = 10^8`:

| excluded | `A_F(10^8)` | `M_F(10^8)` | `Mmax` | `thetamax` |
|---|---|---|---|---|
| 0 | 48427560 | 6410 | 8177 | 0.5091 |
| 1 | 43046720 | 4108 | 6069 | 0.4956 |
| 2 | 43046721 | -183 | 3357 | 0.4619 |
| 3 | 43046721 | 455 | 3512 | 0.4644 |
| 4 | 43046721 | 56 | 4957 | 0.4841 |
| 5 | 43046721 | -7614 | 10601 | 0.5273 |
| 6 | 43046721 | -693 | 2564 | 0.4465 |
| 7 | 43046721 | -1411 | 6494 | 0.4994 |
| 8 | 43046721 | 2131 | 4495 | 0.4785 |
| 9 | 43046721 | 2181 | 5234 | 0.4871 |

The full-set controls at comparable depth: `M(3^17) = -1423` with `Mmax = 4610` (`thetamax` 0.4517), `M(4^13) = 329` with `2845` (0.4413), `M(5^11) = 617` with `2573` (0.4436), `M(10^8) = 1928` with `3448` (0.4422). The Mertens function itself - limiting exponent exactly `1/2` if and only if RH, and at least `1/2` unconditionally - reads `0.4413..0.4517` at these depths, which calibrates every reading above: at census mass even the classical meter sits a few hundredths under `1/2`.

The distribution of the apparent exponent across designs at fixed base, sorted by the generator: at `q = 3` the three columns read `0.4802, 0.4869, 0.4962`; at `q = 4` the ten run `0.4527` to `0.5197`; at `q = 5` the twenty-four with nonzero meter run `0.4471` to `0.5358`; at base 10 the ten Kempner columns run `0.4465` to `0.5273`. All 47 readings sit within `0.054` of `1/2`, against cut readings (`theta` at the checkpoint alone) that scatter over `0.22..0.53` for the same data - the single-cut estimator is noise, the running maximum is the meter.

## The exponent, tagged honestly

- **Conjecture.** For every digit set `F` with `2 <= k <= q - 1` whose digit gcd is squarefree, `theta(F) = 1/2`: square-root cancellation against the set's own mass, the RH shape transplanted to the sparse column. The census is consistent with this and proves none of it: the 47 running-maximum exponents sit in `[0.4465, 0.5358]` with per-family drifts of `0.0157..0.1056` over the last five levels, and the full-set controls - whose limiting exponent is `1/2` under RH - read `0.4413..0.4517` at the same depths. A slope is a fit; the exact integers above are the claim, the exponent is not.
- The believable refutation targets are one family with a proved exponent below `1/2` (excess cancellation) or a proved omega-result (a family whose meter provably tracks its mass). The scaling mechanism produces neither: the vanishing family `{0,4}` at `q = 5` is total cancellation for the trivial reason `4 | n`, and its reduced column `{0,1}` carries the open question unchanged.
- The technology gap is real: distribution of digit-restricted sets in residue classes is [Erdos, Mauduit and Sarkozy 1998](https://www.semanticscholar.org/paper/On-Arithmetic-Properties-of-Integers-with-Missing-Erdos-Mauduit/819d346a221f620ec9107933f0acc22cd345928d), the ellipsephic almost-primes rest on it ([Dartyge and Mauduit 2000](https://doi.org/10.1006/jnth.1999.2458)), and primes in one-excluded-digit sets took the full circle method at large base in [Maynard 2019](https://link.springer.com/article/10.1007/s00222-019-00865-6); the nearest multiplicative function computed over a missing-digit set is the divisor function ([Kim 2024](https://arxiv.org/abs/2411.09076)), a proved `theta` for any restricted column sits at or beyond that frontier, and the missing-digit Dirichlet series literature above carries no Mobius sum at all. The census stands as the falsifiable record the eventual theorem must match.

## Generators

- [lab/mobius-designs](lab/mobius-designs/) prints every row, identity check, slope, distribution and band above: `CARGO_BUILD_JOBS=4 cargo run --release --manifest-path research/lab/Cargo.toml -p mobius-designs`.
- The Mertens control on the [farey](farey.md) page is rendered by `lab/mertens-meter`; the checkpoint controls here are the same function read at powers of the base.
