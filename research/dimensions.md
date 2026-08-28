# Complex dimensions

[The core page](core.md) gives every design one real number, the dimension `log(fill)/log(n)`. Fractal-string theory promotes that number to the real part of an infinite family - the *complex dimensions*, the poles of a zeta function attached to the set's gaps. This page computes them for the 1D designs, watches the imaginary parts surface as an oscillation in the box count, and follows the theory to its structural consequence, which needs careful qualifying.

Every claim carries a tag. **Proved** means a proof is given or restated here; **Verified** means recomputed from scratch by a lab study; **Conjecture** means neither. `lab/complex-dimensions` is the one pass that regenerates every number below; it prints only and keeps no log.

## The string of a design

A 1D design at base `n` is a subset `F` of the `n` digits; it draws the set of `x` in `[0,1]` whose base-`n` digits all lie in `F`. That set is the attractor of `k = |F|` maps `x -> (x + f)/n`, every one with the same contraction ratio `1/n`, and its dimension is `d = log(k)/log(n)`. The core page's parity rule at base 3 fills digits `{0,2}` - the middle-thirds Cantor set, `d = log(2)/log(3) = 0.630930`.

The complement of the set is a multiset of gaps, and the gap multiset of a one-base design satisfies `G = G_1 + k` copies of `G/n`. So its geometric zeta function - the sum of `g^s` over all gaps - has the closed form

```
zeta(s) = D(s) / (1 - k*n^(-s)), D(s) = sum of g^s over level-1 gaps,
```

and its poles sit where the complex Moran equation `k*n^(-s) = 1` holds:

```
s = d + 2*pi*i*m/ln(n), m in Z.
```

One vertical line of poles, equally spaced at `omega = 2*pi/ln(n)`. Sets whose complex dimensions line up on such an arithmetic progression are called *lattice*; sets with incommensurable ratios, whose poles spread out, are *nonlattice*. Every one-base design is lattice for the trivial reason that all its ratios are equal.

**Verified** (`lab/complex-dimensions`). The progression is derived in closed form above and then evaluated: for the Cantor design and three others (table below), all 81 predicted poles at `m = -40..40` kill the denominator to `5e-14`, and the numerator `D(s)` stays bounded away from zero at every one of them - minimum `|D(s)| = 0.500000` for the Cantor design - so no zero is cancelled and every one is a genuine pole. The identity `D(1) = 1 - k/n`, the statement that the gaps of a measure-zero set fill the whole interval, holds for all four.

One caveat: the pole set is the zero set of the denominator only where the numerator does not vanish, and a design with no gaps (`k = n`), with one filled digit (`k = 1`, a point), or with none has an empty gap multiset, an identically zero zeta function, and no complex dimensions at all.

| object | `d` | `omega = 2*pi/ln(n)` |
|---|---|---|
| base 3, digits `{0,2}` | 0.630930 | 5.719202 |
| base 5, digits `{0,2,4}` | 0.682606 | 3.903963 |
| base 15, digits `{0,4,10,14}` | 0.511916 | 2.320188 |
| base 15, digits `{0,2,4,10,12,14}` | 0.661642 | 2.320188 |

## The oscillation in the box count

The imaginary parts are not bookkeeping; they are visible. Let `N(eps)` be the number of `eps`-cells the set meets and detrend it: `g(u) = ln N(exp(-u)) - d*u`. A lattice set's `g` oscillates at angular frequency `omega = 2*pi/ln(n)` forever.

**Verified** (`lab/complex-dimensions`). For all four objects the periodogram peak of `g(u)` lands within 1% of the predicted `2*pi/ln(n)`; for the Cantor design the Blackman periodogram on the direct DFT grid reads 5.7024 against the predicted 5.719202. Four-decimal peak values sit inside one frequency bin of the estimator's resolution and are properties of the estimator, not of the object; what is stable, and what ships, is the sub-1% agreement.

A second reading folds `u` modulo each candidate period and asks how much of the variance of `g` the folded profile explains. At 40 bins on a fixed window:

| object | `ln(3)` | `ln(5)` | `ln(15)` |
|---|---|---|---|
| base 3, `{0,2}` | **0.192** | 0.030 | 0.016 |
| base 5, `{0,2,4}` | 0.018 | **0.509** | 0.034 |
| base 15, `{0,4,10,14}` | 0.007 | 0.022 | **0.667** |
| base 15, `{0,2,4,10,12,14}` | 0.009 | 0.030 | **0.534** |
| two-ratio control | 0.082 | 0.047 | 0.038 |
| aperiodic control | 0.007 | 0.012 | 0.115 |

**Verified** (`lab/complex-dimensions`). Each lattice object folds best at its own `ln(base)`, and the two controls fold well at none. But the two lattice signatures are not parallel in strength: the base-15 design's folding explains 66.7% of the variance at its period, the Cantor design's only 19.2% at its own. The Cantor figure is genuinely modest - the box count is a step function and grid alignment injects a large aperiodic component - and it moves between roughly 0.16 and 0.23 as the bin count and window vary (Conjecture; the study prints the 40-bin value only), so it is also partly a property of the estimator. The ordering is stable; the percentages are not constants of the objects.

## Composition multiplies the base

Compose two rules by alternating bases across levels: base 3 with digits `{0,2}` at odd levels, base 5 with digits `{0,4}` at even ones.

**Verified** (`lab/complex-dimensions`). The alternation produces exactly the one-base design at base 15 with digits `{5*d1 + d2} = {0,4,10,14}` - checked as integer arithmetic and then as geometry, eight alternating levels and four base-15 levels producing the identical 256 intervals as exact fractions. So the composite is a different lattice period, `omega = 2*pi/ln(15) = 2.320188`, not a departure from the lattice class - and the table above shows its signature is the *sharpest* of the family, 66.7% of variance at its own period.

Two qualifications, both load-bearing. First, base 5 with digits `{0,4}` is not a mrly design: the core page's move one fills by parity, and the even digits of base 5 are `{0,2,4}`, three of them - nor is the base-15 composite a parity design. The parity-faithful versions - base 5 `{0,2,4}` and their composite base 15 `{0,2,4,10,12,14}` - run alongside and behave identically, which is the real point: the argument turns on the base, not on which digits the rule picks.

Second, the structural claim that one contraction ratio per level implies lattice is false without a periodicity hypothesis. (Refuted.) An aperiodic control that alternates bases 3 and 5 on a Thue-Morse schedule uses exactly one ratio per level and is not self-similar at all, so it is neither lattice nor nonlattice: its best folding is 0.115 against the composite's 0.667, and its periodogram peak matches no `2*pi/ln(base)`. What survives, and needs no computation, is the statement for mrly designs proper: move two is a Kronecker power of one tile, so the schedule is constant, every level subdivides by the same `n`, and periodic cross-base alternation multiplies into one product base. Neither move can express two ratios inside one level - move one only chooses which cells of a fixed `n^D` grid survive, and every cell of that grid is the same size. (Verified for the alternation; the one-tile argument is read off the definition.)

"Read off the definition" has a proof behind it, and it covers more than one tile. **Block reduction, Proved:** for any designs `c_1, ..., c_p` and any `L >= 1`, the periodic word `(c_1, ..., c_p)^L` equals `(A_(c_1) (x) ... (x) A_(c_p))^((x) L)`, by associativity of the Kronecker product and nothing else. (The case `p = 2` is the base-15 alternation Verified above; a check on six test cases at periods 2 and 3, lengths to 6, matching cell for cell, has no lab generator.) So every periodic schedule *is* the ordinary self-similar theory of its one-period composite tile, of base `prod_i n_i` and fill `prod_i k_i`; the base-15 composite above is the case `p = 2`, and its agreement was never in doubt. The corollary is the sharper half. The first genuinely non-stationary behaviour requires an aperiodic word, which is exactly why the Thue-Morse control above is neither lattice nor nonlattice rather than being a third kind of composition. For an aperiodic word at common base `q` with `k_i = k(c_i)`, the scale dimension is `lim_L (sum_(i <= L) log k_i) / (L log q)` when the limit exists; characterizing which words make it exist, and which leave the dimension fluctuating, is open. (Conjecture, untouched.)

The genuine way out is a two-ratio system: maps of ratio `1/3` and `1/5` mixed *within* one level, outside the mrly family, with `d = 0.518370` solving `3^(-d) + 5^(-d) = 1`. **Verified** (`lab/complex-dimensions`). Its 21 complex dimensions in the box `Re` in `[-3,3]`, `Im` in `[-40,40]` - a complete list, by the argument principle: the winding number over the box is 21 and 21 roots are found - have real parts spread from `-0.699926` to `0.518370` and fit no arithmetic progression, the worst offset being 0.43, 0.17 and 0.38 of a step for the three candidate spacings. Nonlattice is a real, different behaviour, and no mrly design or composition exhibits it.

## Measurability, with its hypotheses

A set is *Minkowski measurable* when `M(eps) = eps^(d-1) * V(eps)` - `V` the inner tube, the length of the set's `eps`-neighbourhood inside the gaps - has a limit as `eps -> 0`. The lattice/nonlattice split decides this, and what is known is narrower than it looks.

**Proved.** The Cantor design `{0,2}` at base 3 is not Minkowski measurable. Splitting the tube sum at the scale of `eps` gives the exact limit profile

```
M -> 2^(1-d) * (t^(d-1) + t^d), t in [1/3, 1),
```

one fixed profile traversed each time `eps` is divided by 3, with minimum `2.494975716` at `t = (1-d)/d = 0.584963` and maximum `2.583040469` at the ends - a swing of 3.53%, so the profile is not constant and the limit does not exist. The measured tube matches the closed form to `4.9e-9` at the minimum (`lab/complex-dimensions`).

**Verified** (`lab/complex-dimensions`). The other three lattice objects behave the same way: the swing of `M(eps)` over successive windows is flat from `u = 15` out to `u = 60` (`eps = 8.8e-27`), and each object satisfies `M(eps) = M(eps/n)` at its own base to `1e-9` or better and at neither other candidate. The two-ratio control does the opposite: its swing decays monotonically `3.79%` to `0.42%` and is still falling - converging, as the nonlattice side predicts.

The literature, read rather than recalled, is not symmetric. Nonlattice self-similar sets under the open set condition are Minkowski measurable in every dimension (Gatzouras 2000). Lattice sets are not - but as a theorem only on the line, for a nontrivial set of non-integer dimension (Falconer 1995, completed by Kombrink and Winter 2020; for self-similar strings, Lapidus and van Frankenhuijsen 2006 - that attribution rests on secondary citations, the book itself being unopened for this page). In dimension 2 and above the lattice direction is an open conjecture of Lapidus, proved under a pluriphase hypothesis and for particular families, open in general.

So the claim that *every* mrly design is not Minkowski measurable ships only with two qualifications, and both bite on real designs.

1. **Nontrivial fill and non-integer dimension.** The theorem's own hypotheses exclude integer dimension, and the exclusion is not exotic: `mrly_03`, the core page's `pin(y)`, fills 2 of 4 at base 2, has `d = log(2)/log(2) = 1` exactly, draws a segment - and a segment *is* Minkowski measurable. The solid, single-point and empty designs have no gaps and no oscillation to have. At base 3 in 1D the only design of non-integer dimension is `{0,2}` itself. (Proved: the counterexample and the census of which designs the statement covers are read off the definition.)
2. **Dimension one only.** For 1D designs with `2 <= k < n` and non-integer `d`, non-measurability is a theorem, and for the Cantor design it is proved outright above. For the carpet and the sponge, lattice membership holds - but non-measurability is the *conjectured* consequence, a Conjecture here as in the literature, not a citable theorem.

## The door this shuts, and what would open it

Every design has an exact geometric zeta function: for `k` pieces at base `q`, `zeta_L(s) = 1/(1 - k*q^(-s))`, whose poles are the complex dimensions `s = log_q(k) + 2*pi*i*m/ln(q)` this page already tabulates. What that bookkeeping meets is a thirty-year-old theorem. `(ISP)_D` asks: if a fractal string of dimension `D` has spectral counting function `N(x) = W(x) - C*x^D + o(x^D)` with `C` nonzero, must the string be Minkowski measurable? Lapidus and Maier 1995: `(ISP)_D` holds for all strings of dimension `D` if and only if `zeta` has no zeros on the line `Re(s) = D`. So `(ISP)_D` for every `D` in `(0,1)` except `D = 1/2` is equivalent to RH, and `(ISP)_{1/2}` is false outright, the midfractal case being the obstruction. That is RH stated entirely in the language of fractal geometry, and it is the highest-adjacency RH equivalence this tree touches.

**And it is vacuous here.** Every one-base design is lattice - proved in the sections above, not restated - so the complex dimensions sit periodically on one vertical line and the Lapidus-Maier machinery has nothing to say about the degenerate case. Measurability is not out of reach here: it is trivially settled and therefore empty.

Scope guard, the same one this page already applies: the lattice/nonlattice dichotomy is exact for self-similar STRINGS and settles dimension one. A one-base carpet or sponge is certainly lattice, but lattice membership alone does not prove higher-dimensional non-measurability. State the dimension and the object class every time.

**What would give it content: several incommensurable scaling ratios.** Drop the single base and allow pieces scaled by `r_1, ..., r_N` with `ln(r_i)/ln(r_j)` irrational for some pair - a Moran construction, or a graph-directed self-similar set. Complex dimensions become quasiperiodic instead of periodic, measurability becomes a real question, and `(ISP)_D` acquires content. The two-ratio system verified above is the smallest instance of exactly this. "Several bases" is not automatically non-lattice: the contraction system and its separation hypotheses have to be specified before any of the above applies. This is independently the single most valuable generalization available to the tree, arrived at from two directions - the measurability question and the RH map both end on the same instruction.

## Staircase schedules, the cheapest non-stationary object

- Instead of a constant word, stack `carpet_3`, then `magic(3,5)`, then `magic(3,5,7)`, and so on; Kronecker associativity flattens that to the staircase word `3 | 3,5 | 3,5,7 | 3,5,7,9 | ...`.
- Letter `q_j` occurs `n - j + 1` times in the first `n` blocks, so the controls are immediate and non-negotiable: `side = prod_j q_j^(n-j+1)`, `fill = prod_j f_j^(n-j+1)`, and `dim_n = Sum_j (n-j+1)*ln(f_j) / Sum_j (n-j+1)*ln(q_j)`.
- The staircase word is aperiodic and not eventually periodic, so block reduction does not apply to it. It is the cheapest concrete non-stationary schedule available.
- The weights `(n-j+1)` are a Cesaro profile - the earliest letter carries weight `n`, the newest carries 1 - so if `d_j -> d` the dimension converges to `d`; the interesting regime is `d_j` oscillating.
- The generalisation is what makes this a programme rather than an example: a staircase is one weight profile, any letter-multiplicity schedule is another, and the question "which dimension functions are realisable by a schedule and which are not" quantifies over all schedules and is native to the construction.
- **Caveat that travels with every mixed number.** If each factor is rendered at its native base, `n_i = q_i`, the filled points have the mixed-radix form `x = a_1*(q_2...q_L) + ... + a_L` with `a_i` in `F_i`, and that is the correct arithmetic object. A factor rendered at a side unrelated to its residue base is still a valid tile product, but it is not a mixed-radix digit construction and inherits no digit theorem for free.
- `lab/slice-ladder-controls` prints the five staircase dimensions - `1.892789261`, `1.892315261`, `1.893034267`, `1.894190425`, `1.895495742` at `n = 1..5` - assuming `carpet_q` has fill `q^2 - ((q-1)/2)^2`; the run states that assumption before any number. (Verified under that assumption; confirm the definition against [the core page](core.md) before quoting any number from it.)

## Where the numbers live

`lab/complex-dimensions` is the one pass behind every number on this page - the poles, the box-count periodogram and folding, the composition, the two-ratio control and the tube - and it prints only. `lab/slice-ladder-controls` prints the staircase dimensions. The dimension formula this page extends, and the designs it names, are [the core page](core.md); the spectral side of the same fractals is [the complexity page](complexity.md).
