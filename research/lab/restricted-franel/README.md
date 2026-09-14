# restricted-franel

- Franel's identity on a digit design: an exact formula for the Fourier and the rank `L^2` discrepancy of the denominator-restricted Farey set, written as a gcd-weighted double sum over DILATED Mertens sums.
- The object is [the Farey page](../../farey.md)'s THE METER ON A DIGIT DESIGN, denominator convention, written `F_Q^d(S_F)` here because [the Farey page](../../farey.md) binds `F_Q(S_F)` to the STRICT set: `F_Q^d(S_F) = {a/b reduced, b in S_F, b <= Q, 1 <= a <= b}`, with `S_F` the whole numbers whose every base-`q` digit lies in a digit set `F`.
- Four results: the frequency-`m` exponential sum (Theorem 1), the identity in both its Fourier and its rank form (Theorem 2), the one-line inequality that carries the restricted Mertens sum back out of it (Theorem 3), and the strict convention's divisor-sum identity with the refutation that kills every Mertens-type closed form for it (Theorem 4).
- The template is [gaussian-franel](../gaussian-franel/), which is the same transcription one field up. The enumeration is this study's own: [farey-discrepancy](../farey-discrepancy/) is a Rust crate whose `Design` type and mediant walk are not exported to Python, so the digit test and the design list are rebuilt here in eight lines and the node counts are checked against that study's published table.

## THE OBJECT

- Design: base `q >= 2`, digit set `F`, `S_F` the whole numbers whose every base-`q` digit lies in `F`, mass exponent `alpha = log|F|/log q`. The two designs run here are base 3 with `F = {0,1}` and base 10 with the digit 9 missing, the pair [the Farey page](../../farey.md) meters.
- Denominator set: `F_Q^d(S_F) = {a/b : b in S_F, b <= Q, 1 <= a <= b, gcd(a,b) = 1}`, of size `m_F(Q) = sum_{b in S_F, b <= Q} phi(b)`.
- Strict set: `F_Q^s(S_F) = {a/b : a in S_F, b in S_F, b <= Q, 1 <= a <= b, gcd(a,b) = 1}`.
- **The dilated Mertens sums.** `M_F(x; d) = sum_{c <= x, dc in S_F} mu(c)`, the Mertens function of the dilated design `d^{-1} S_F = {c : dc in S_F}`. At `d = 1` it is the design's own meter `M_F(x) = sum_{n in S_F, n <= x} mu(n)`, [the Mobius page](../../mobius.md)'s. The dilates are the whole content of the restriction: `d^{-1} S_F` is not `S_F`, is not a digit design in general, and carries no digit test, so every `d > 1` brings a new function rather than a rescaling of the old one. On the full set every dilate is `Z` and all of them collapse to `M`.
- Exponential sums: `S_F(k, Q) = sum_{r in F_Q^d(S_F)} e(kr)` with `e(x) = exp(2 pi i x)`, and `S_F^s(k, Q)` the same over the strict set.

## THEOREM 1, THE EXPONENTIAL SUM

- **Ramanujan's sum over a digit design.** For every `m >= 1` and `Q >= 1`, `S_F(m, Q) = sum_{d | m} d M_F(Q/d; d)`. **Proved.**
- Proof. Partition by denominator: `S_F(m, Q) = sum_{b in S_F, b <= Q} c_b(m)` with `c_b(m) = sum_{a mod b, gcd(a,b) = 1} e(ma/b)` Ramanujan's sum, which depends on `b` alone and not on the design. Kluyver's formula is `c_b(m) = sum_{d | gcd(m, b)} d mu(b/d)`. Exchanging the two sums gives `S_F(m, Q) = sum_{d | m} d sum_{b in S_F, b <= Q, d | b} mu(b/d)`, and substituting `b = dc` turns the inner sum into `M_F(Q/d; d)`.
- **At frequency 1 the exponential sum IS the design's Mertens meter.** `S_F(1, Q) = M_F(Q)`. **Proved**, the `m = 1` case, only `d = 1` surviving. It needs no Ramanujan input: `sum_{a mod b, gcd(a,b) = 1} e(a/b) = sum_{d | b} mu(d) sum_{c mod b/d} e(c/(b/d)) = mu(b)`, the inner complete sum vanishing unless `b/d = 1`.
- The classical case is `S_F = Z`, where `M_F(Q/d; d) = M(Q/d)` for every `d` and the formula reads `sum_{d | m} d M(Q/d)`, the divisor-shifted Mertens sums.
- **Verified.** Every denominator `b in S_F` up to `Q = 10^5` at base 3 `{0,1}`, and up to `Q = 10^4` at base 10 without 9 and on the control, has its literal sum of `phi(b)` roots of unity equal to `mu(b)`: worst deviation `1.09e-11` at `b = 86293` on the base 3 design and `1.36e-12` at `b = 7247` on base 10 without 9, `0` roundings to the wrong integer at either. The `Q = 10^5` rung on base 10 without 9 costs `2.6e9` roots of unity, past the machine budget, so it is not walked and nothing is claimed at it. The aggregated literal sum agrees with the exact integer `M_F(Q)` to `2.02e-09` at the top rung, floating-point accumulation over `5.4e7` roots of unity and not a mismatch. The frequency-`m` formula is checked at `m = 1, 2, 3, 4, 5, 6, 12` on both designs and the control (`verb_denominator`).

## THEOREM 2, FRANEL ON A DIGIT DESIGN

- **The kernel.** `G_F(Q) = sum_{d, e >= 1} (gcd(d,e)^2/(d e)) M_F(Q/d; d) M_F(Q/e; e)`, a finite sum of exact rationals, every term with `d > Q` or `e > Q` vanishing.
- **The Fourier form.** `sum_{k != 0} |S_F(k, Q)|^2 / k^2 = (pi^2/3) G_F(Q)`. **Proved.** Substitute Theorem 1, expand the square and exchange: for fixed `d, e` the inner sum is `sum_{k != 0, lcm(d,e) | k} k^-2 = 2 zeta(2)/lcm(d,e)^2`, and `d e/lcm(d,e)^2 = gcd(d,e)^2/(d e)`.
- **The rank form.** If the node set is closed under `r -> 1 - r` away from the node `1`, then with `rho_1 < ... < rho_m` the nodes of `F_Q^d(S_F)` ascending, `m = m_F(Q)` and `delta_j = rho_j - j/m`, `G_F(Q) - 1 = 12 m sum_j delta_j^2`, an identity of exact rationals. **Proved.** Closure is used only to fix `sum_r r = (m+1)/2`, so THAT mean-value condition is the surviving hypothesis and closure is one sufficient condition for it; the piecewise integration below needs nothing of the node set beyond a top node of `1`. Closure holds for every denominator-restricted set, since `a/b` reduced with `b in S_F` gives `(b-a)/b` reduced with the same `b` and `a = b` only at the node `1`; it fails for every proper strict set.
- Proof of the rank form. With `B1bar` the sawtooth and `A(v)` the counting function of the nodes, `sum_r B1bar(u + r) = D(1 - u) + c` with `D(v) = A(v) - m v` and `c = sum_r r - m/2`, so Parseval gives `sum_{k != 0} |S_F(k,Q)|^2/k^2 = 4 pi^2 (int_0^1 D^2 - c^2)`. Integrating `D^2` piecewise between consecutive nodes and telescoping gives `int_0^1 D^2 = m sum_j delta_j^2 - (m+1)/2 + sum_r r + 1/3`, the two boundary cubes vanishing because `rho_m = 1`. Under the reflection hypothesis the nodes other than `1` pair into `m - 1` values summing to `(m-1)/2`, so `sum_r r = (m+1)/2` and `c = 1/2`, and the remainder collapses to `1/12`.
- The kernel is the gcd matrix `gcd(d,e)^2/(d e)`, the same Smith kernel that carries the moire correlation law on [the stack page](../../stack.md) and the Gaussian identity in [gaussian-franel](../gaussian-franel/). Digit restriction moves the entries, never the kernel.
- **Verified.** The rank form is an exact rational identity at base 3 `{0,1}` `Q = 81` and `Q = 243`, base 10 without 9 `Q = 40`, and the full-set control `Q = 40`: `True` at all four. The Fourier form is checked against the literal Fourier side truncated at `|k| <= 200000` with the printed tail bound `2 m^2/K`, the gap inside the bound in every row. The control at `Q = 40` regenerates Edwards section 12.2, `G_F(40) - 1 = 12 m sum delta^2` with `m = 490` and `sum delta^2 = 0.0104270117`, which is the number [gaussian-franel](../gaussian-franel/) prints (`verb_identity`).

## THEOREM 3, WHAT COMES BACK OUT

- **The inequality.** `2 M_F(Q)^2 <= (pi^2/3) G_F(Q)`, and by the rank form `(pi^2/3) G_F(Q) = 4 pi^2 m_F(Q) sum_j delta_j^2 + pi^2/3`. **Proved**, in one line: `k = 1` and `k = -1` contribute `2|S_F(1,Q)|^2 = 2 M_F(Q)^2` to a sum of nonnegative terms. The constant is not decorative: the weaker-looking `2 M_F(Q)^2 <= G_F(Q)` is FALSE, failing already at the full-set control `Q = 5` (`2 M(5)^2 = 8` against `G(5) = 64/15`) and at base 3 `{0,1}` `Q = 37` (`18` against `G_F(37) = 14.230517`).
- So a square-root bound on the denominator-restricted Farey discrepancy forces a square-root bound on the design's Mertens meter. The converse needs the whole `k` sum and is not claimed here.
- **The implication.** `A_F(Q) = #{n in S_F : n <= Q} << Q^alpha`, the block count `|F|^L` at `Q = q^L` with a constant depending on the design alone, so `m_F(Q) = sum_{b in S_F, b <= Q} phi(b) <= Q A_F(Q) << Q^(1+alpha)`. Hence the denominator lane's CONJECTURED shape `sum_j delta_j^2 = O(Q^(-1+eps))` on [the Farey page](../../farey.md) gives `G_F(Q) = O(Q^(alpha+eps))` through the rank form, and the inequality then gives `|M_F(Q)| = O(Q^(alpha/2+eps))`, which is the square-root ceiling for the design's Mertens meter on [the Mobius page](../../mobius.md). **Proved**, no unproved input entering, the constant `pi^2/3` being absorbed. What yields the ceiling is the conjectured exponent `-1` and not the measured one: the lane's measured `e_2` sits at `-0.959` and `-0.899` at the top rungs and `S2 Q` is still climbing there, and a proof of only the measured shape would give no ceiling. Digit restriction of the denominator is invisible to the SHAPE of the meter, and this is what that invisibility costs.
- The converse is open and is where the road ends. It needs the whole `k` sum controlled from Mertens bounds, so it needs `M_F(x; d)` for `d > 1`, and no bound for a dilated design's Mertens function exists on the tree.
- **Verified**, at every integer `Q` rather than at a sample. Both sides step only at `Q in S_F`, since `M_F(Q)` and every `M_F(Q/d; d)` change only when `Q` itself joins `S_F`, so scanning `S_F` covers every integer `Q` below the bound. The inequality is asserted at `0` violations over `Q <= 2187` at base 3 `{0,1}` (128 jumps) and `Q <= 400` at base 10 without 9 (324) and on the control (400). The ratio `2 M_F(Q)^2/((pi^2/3) G_F(Q))` peaks at `0.607927` at the trivial `Q = 1` on all three; over `Q >= 100` its maximum is `0.340071` at `Q = 253` on base 3 `{0,1}`, `0.137645` at `Q = 221` on base 10 without 9 and `0.086385` at `Q = 114` on the control (`scan_backward`).
- That the `k = +-1` terms carry a third of the whole functional at `Q = 253` on a thin design is worth its own look: if the fraction does not fall with `Q`, the inequality is near sharp there and the converse is nearer than OBSTRUCTIONS claims. Not measured here.

## THEOREM 4, THE STRICT SET

- **The divisor-sum identity.** `S_F^s(1, Q) = sum_{b in S_F, b <= Q} sum_{d | b} mu(d) sum_{a <= b/d, da in S_F} e(a/(b/d))`. **Proved**, Mobius inversion of the coprimality condition followed by `a -> da`. It is exact and it is where the road stops: the inner sum is a digit-restricted exponential sum over an arithmetic progression, the Type II object [the Mobius page](../../mobius.md) has no bound for.
- **Verified** at every denominator: literal summation over `{a in S_F : a <= b, gcd(a,b) = 1}` against the divisor route agrees to `6.28e-15` over the 64 denominators of base 3 `{0,1}` below 729 and to `1.95e-14` over the 162 denominators of base 10 without 9 below 200.
- **No Mertens-type sum over `S_F` equals it. Refuted**, two ways, the first exact.
- First, the strict sum is not real. Base 3 `{0,1}` at `Q = 3` gives `1 + e(1/3) = 0.5 + (sqrt 3/2) i`, and base 10 without 9 at `Q = 10` gives `-1.809016994 + 0.587785252 i`; every Mertens-type sum over `S_F` is a real integer or a real rational. The two witnesses carry the whole refutation. Beside them sits an observation and not a mechanism: the strict set also fails the pairing `a -> b - a` that makes the denominator set's sum real, since `a in S_F` does not give `b - a in S_F`. Failure of that pairing is not shown to force a non-real sum, and nothing here rests on it.
- Second, it rides the mass. `|S_F^s(1,Q)|/card` settles at `0.335693, 0.343837, 0.345905, 0.346338` over `Q = 3^5, 3^7, 3^9, 3^11` at base 3 `{0,1}` and at `0.015138, 0.012250, 0.011561` over `Q = 10^2, 10^3, 10^4` at base 10 without 9, so `|S_F^s(1,Q)|` grows like the node count `Q^(2 alpha)` while every Mertens-type sum over `S_F` is bounded by `#{n in S_F : n <= Q} = O(Q^alpha)`. At `Q = 3^11` the strict sum has modulus `374203.231` against `M_F(Q) = -10`.
- The three candidates the refutation names, each killed at the smallest `Q` printed: `M_F(Q)`, the count-weighted `sum_{b in S_F, b <= Q} mu(b) phi_F(b)` and the normalised `sum_{b in S_F, b <= Q} mu(b) phi_F(b)/phi(b)`, with `phi_F(b) = #{a in S_F : a <= b, gcd(a,b) = 1}`. All three are real, so the first refutation kills all three at once, at `Q = 3` and `Q = 10` respectively.
- This is the Fourier face of [the Farey page](../../farey.md)'s reading that the strict lane's `S1` and `S2` ride the mass: a set whose frequency-1 sum is proportional to its own count has no cancellation at frequency 1 and does not equidistribute.

## WHAT IT PRINTS

- `verb_denominator`: the frequency-`m` table at `m = 1, 2, 3, 4, 5, 6, 12` for base 3 `{0,1}` at `Q = 2187`, base 10 without 9 at `Q = 1000` and the control at `Q = 300`, exact integer against literal sum; then the frequency-1 ladder to `Q = 10^5` on base 3, `Q = 10^4` on base 10 and on the control, with the per-denominator worst deviation, the denominator it sits at, and the count of denominators rounding to the wrong integer.
- `verb_identity`: the Fourier form against the truncated Fourier side with its tail bound at four settings; the rank form as an exact rational identity at the same four; the Edwards control at `Q = 40`; the backward inequality's ratio at six settings.
- `verb_strict`: the divisor-sum identity's worst deviation at base 3 to 729 and base 10 to 200; the smallest `Q` with a nonzero imaginary part on each design; and the refutation ladder to `Q = 3^11` and `Q = 10^4` carrying `card`, `Re`, `Im`, `|S_F^s|`, `|S_F^s|/card` and all three candidates.

| set | `Q` | `card` | `Re` | `Im` | `abs` | `abs/card` | `M_F(Q)` |
|---|---|---|---|---|---|---|---|
| base 3 `{0,1}` | 243 | 278 | 75.524 | 54.820 | 93.323 | 0.335693 | 4 |
| base 3 `{0,1}` | 2187 | 4286 | 1251.588 | 777.991 | 1473.683 | 0.343837 | -7 |
| base 3 `{0,1}` | 19683 | 67561 | 19889.522 | 12269.876 | 23369.702 | 0.345905 | -4 |
| base 3 `{0,1}` | 177147 | 1080458 | 320313.430 | 193461.533 | 374203.231 | 0.346338 | -10 |
| base 10 without 9 | 100 | 1830 | 24.736 | 12.471 | 27.702 | 0.015138 | 1 |
| base 10 without 9 | 1000 | 147096 | 1730.230 | 503.091 | 1801.887 | 0.012250 | -1 |
| base 10 without 9 | 10000 | 11890654 | 130648.220 | 42778.695 | 137473.540 | 0.011561 | 17 |

- The `card` column is the independent control: `278, 4286, 67561, 1080458` and `1830, 147096, 11890654` are the strict counts [farey-discrepancy](../farey-discrepancy/) prints from a Mobius sieve that enumerates no fraction, and this study reaches them by literal enumeration.

## RUN

- `uv run python research/lab/restricted-franel/restricted_franel.py`
- From the repository root. One core, 18.8 s for all three verbs: `denominator` 6.9 s, `identity` 10.5 s, `strict` 1.6 s. Append `denominator`, `identity` or `strict` for one.
- Domain: the frequency-1 identity to `Q = 10^5` at base 3 `{0,1}` and `Q = 10^4` at base 10 without 9 and the control; the frequency-`m` identity at seven frequencies; the Franel identity at `Q <= 243`; the strict refutation to `Q = 3^11` and `Q = 10^4`.
- The wall: the literal frequency-1 check at `Q = 10^5` on base 10 without 9 costs `2.6e9` roots of unity, past the machine budget, so that rung is not walked.
- Nothing is written to disk.

## WITNESSES

- Theorem 1: `ladder_check` and `mertens_dilated`, with `literal_denominator` as the multi-frequency route.
- Theorem 2: `kernel_sum` against `fourier_side` with the printed tail bound, and against `farey_delta_square` as an exact rational identity.
- Theorem 3: the ratio column of `verb_identity`.
- Theorem 4: `strict_ramanujan_literal` against `strict_ramanujan_divisor`, then `strict_literal` and `strict_weights` on the ladder.
- The node counts: `strict_literal`'s `card` against the published strict counts of [farey-discrepancy](../farey-discrepancy/).
- [The Farey page](../../farey.md) THE RESTRICTED FRANEL IDENTITY, the objects and the identity: the dilated sums `M_F(x; d)`, the frequency-`m` formula and its frequency-1 case, with the deviations `1.09e-11` at `b = 86293` to `Q = 10^5` at base 3 `{0,1}` and `1.36e-12` at `b = 7247` to `Q = 10^4` at base 10 without 9 and on the control, the `Q = 10^5` rung on base 10 not walked, `0` wrong roundings, and the seven frequencies (Theorem 1).
- [The Farey page](../../farey.md) THE RESTRICTED FRANEL IDENTITY, the two `L^2` forms: `G_F(Q)`, the Fourier form inside its tail bound and the rank form as exact rationals at the four settings, with the Edwards control `G_F(40) = 62.310829` at `m = 490` and `sum delta^2 = 0.0104270117` (Theorem 2).
- [The Farey page](../../farey.md) THE RESTRICTED FRANEL IDENTITY, the inequality and the implication: the false display's witnesses `Q = 5` and `Q = 37`, `0` violations at every integer `Q <= 2187` and `Q <= 400`, the peak `0.607927` at `Q = 1` and the maxima `0.340071` at `Q = 253`, `0.137645` at `Q = 221` and `0.086385` at `Q = 114` (Theorem 3).
- [The Farey page](../../farey.md) THE RESTRICTED FRANEL IDENTITY, the strict set: the divisor identity, the non-real sums at `Q = 3` and `Q = 10`, the three killed candidates, the ratios `0.335693` to `0.346338` and `0.015138` to `0.011561`, and the modulus `374203.231` against `M_F(3^11) = -10` (Theorem 4).
