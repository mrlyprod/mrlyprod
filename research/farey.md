# The stack is an RH-observable

Lay the same fractal grid on the unit square at many scales at once - scale `n` puts its cell boundaries at `x = k/n` - drop the opacity and add the layers up. The result is a moire, and a bright point is one that many scales agree on. The question this page answers is what the bright points are, and the answer is not decorative: the lit nodes are the Farey fractions, the amount of new structure each scale contributes is Euler's totient `phi(n)`, and how evenly those nodes spread is - by a pair of theorems from 1924 - literally equivalent to the Riemann hypothesis.

Every claim below carries a tag. **Proved** means derived here from definitions. **Verified** means recomputed from scratch, or checked against the published literature. Nothing on this page is a conjecture. The [Farey demo](../demos/farey/) builds the stack scale by scale, lights the Farey fractions, and shows `phi(n)` novelty peaking at the primes.

## Where the lines land

Stack the scales `n = 1..N`. A point `a/b` in lowest terms receives a grid line from exactly the scales that are multiples of `b`, so over `1..N` its brightness is `floor(N/b)`. **Proved**, and **Verified** by direct simulation at `N = 30`: building the stack node by node and comparing every node's hit count against `floor(30/b)` gives no mismatch anywhere (`lab/farey-discrepancy`).

Brightness therefore falls as one over the denominator, which is the Stern-Brocot ordering of the rationals. The top of the table at `N = 30`:

| node | brightness | `floor(30/b)` |
|---|---|---|
| `0`, `1` | 30 | 30 |
| `1/2` | 15 | 15 |
| `1/3`, `2/3` | 10 | 10 |
| `1/4`, `3/4` | 7 | 7 |
| `1/5` ... | 6 | 6 |

The lit nodes are also exactly the lattice points visible from the origin, since `a/b` is in lowest terms precisely when `gcd(a,b) = 1`. That is the "lighthouse" reading of the picture. **Proved.** The density of visible points is `6/pi^2` - the same constant, and the same base-blindness, discussed in [what base 3 hides](bases.md), where it is measured as `0.608042` on a `3000 x 3000` grid. **Verified**, by recounting that grid.

## Primes are the maximally novel scales

The nodes scale `n` introduces *for the first time* are the fractions `a/n` with `gcd(a,n) = 1`, since any `a/n` that reduces was already lit by the smaller scale it reduces to. There are exactly `phi(n)` of them. **Proved**, and **Verified** by set difference over the stack for `n = 2..30`:

```
n    2 3 4 5 6 7 8 9 10 11 12 13 14 15 16
new   1 2 2 4 2 6 4 6 4 10 4 12 6 8 8

n   17 18 19 20 21 22 23 24 25 26 27 28 29 30
new  16 6 18 8 12 10 22 8 20 12 18 12 28 8
```

Every count equals `phi(n)`, and the running maxima `1, 2, 4, 6, 10, 12, 16, 18, 22, 28` occur at `n = 2, 3, 5, 7, 11, 13, 17, 19, 23, 29`. The reason is one line: `phi(n) = n - 1` if and only if `n` is prime, because every one of `1..n-1` is coprime to `n` exactly when `n` has no smaller factor. **Proved**, and **Verified** by testing the equivalence against trial division for all `n` up to 200.

So primality is readable off the picture. Stack `1..n-1`, then add scale `n`, and count what appeared: `n - 1` new nodes means `n` is prime, fewer means composite. **Proved** (it is the previous claim restated). A composite scale mostly re-lights nodes its own divisors already drew - scale 30 adds only 8 new lines, the rest of its grid falling on lines from 1, 2, 3, 5, 6, 10 and 15.

## Franel and Landau, 1924

Over scales `1..Q` the stack lights exactly the reduced fractions of denominator at most `Q`: the Farey sequence `F_Q`. Its size in `(0,1]` is `m = sum_{k<=Q} phi(k)`. **Proved**, and **Verified** by generating `F_Q` through the next-term recurrence and comparing its length with the totient sum at `Q = 10, 30, 60` (`lab/farey-discrepancy`, which runs the same comparison at `Q = 10, 30, 60, 125`).

Write `rho_1 < ... < rho_m` for those nodes and `delta_j = rho_j - j/m` for how far each one sits from perfect equidistribution. Then:

- Franel (1924) proved that `sum_j delta_j^2 = O(Q^(-1+eps))` for every `eps > 0` is equivalent to the Riemann hypothesis.
- Landau (1924), in a note published immediately after Franel's, proved the same for `sum_j |delta_j| = O(Q^(1/2+eps))`.

**Verified** against the literature: both statements, with the original 1924 citations to the Göttingen Nachrichten, are the standard Franel-Landau formulation, and are reproduced in Edwards, *Riemann's Zeta Function*, chapter 12.

Put the two halves together. The nodes whose discrepancy Franel and Landau are talking about are the nodes the stack draws - not an analogue of them, the same set. So the question "how evenly are the bright points spread?" is not *related to* the Riemann hypothesis; at this level of precision it **is** the Riemann hypothesis. **Proved**, given the identification above, which is what the first two sections establish.

## The meter reads what RH predicts

Both sums are computable. Generating `F_Q` exactly and measuring, with `S2 = sum delta_j^2` and `S1 = sum |delta_j|`:

| `Q` | nodes | `S2*Q` | `S1/sqrt(Q)` | local exponent of `S2` |
|---|---|---|---|---|
| 125 | 4796 | 0.5395 | 0.2040 | - |
| 250 | 19024 | 0.5848 | 0.1942 | -0.884 |
| 500 | 76116 | 0.6241 | 0.1852 | -0.906 |
| 1000 | 304192 | 0.6387 | 0.1634 | -0.967 |
| 2000 | 1216588 | 0.6560 | 0.1512 | -0.961 |
| 4000 | 4863602 | 0.6538 | 0.1314 | -1.005 |
| 8000 | 19455782 | 0.6564 | 0.1123 | -0.994 |

**Verified** by `lab/farey-discrepancy`. `S2*Q` flattens near `0.656` and the local exponent walks to `-1`, which is the Franel condition; `S1` stays under its `Q^(1/2)` envelope and its own local exponent runs between `0.27` and `0.43`, under the Landau threshold of `0.5`. The node count matches `sum phi(k)` exactly at every rung, which is the control that says the object being measured really is the stack's node set.

## Weighting the stack by Mobius

Give scale `n` the weight `mu(n)` instead of weight one and the same stack renders a different arithmetic function: the node `a/b` collects `mu` over the scales that are multiples of `b`, so its brightness is `Sum_{k <= N/b} mu(kb) = mu(b) * Sum_{k <= N/b, gcd(k,b) = 1} mu(k)`, a Mertens-type sum over the integers coprime to `b`. **Proved**, by the same divisor count that gives `floor(N/b)` in the unweighted stack. It is not `M(floor(N/b))`: the two agree at only 64 of 200 denominators at `N = 200` (`lab/mertens-meter`), and coincide at `b = 1`, where the node reads `M(N)` exactly.

That makes the picture a Mertens meter rather than a Farey one, and the oscillations of `M(x)/sqrt(x)` are where the nontrivial zeta zeros live, by the explicit formula. Sampling `M(x)/sqrt(x)` in log-space and taking the power spectrum puts peaks at the first eight zeros:

| known `gamma` | detected | error |
|---|---|---|
| 14.1347 | 13.94 | 0.20 |
| 21.0220 | 20.90 | 0.12 |
| 25.0109 | 24.97 | 0.04 |
| 30.4249 | 30.19 | 0.23 |
| 32.9351 | 32.52 | 0.42 |
| 37.5862 | 37.74 | 0.16 |
| 40.9187 | 40.64 | 0.27 |
| 43.3271 | 42.97 | 0.36 |

**Verified** by `lab/mertens-meter`, and weaker than everything above it on this page: `M(x)/sqrt(x)` for `x = 1..50000` from a linear Mobius sieve, resampled uniformly in `log x` on 8192 points, Hann-windowed, the real FFT power spectrum read as `gamma = 2 pi f`, local maxima above three times the band median over `8 < gamma < 55`. The bin width is `0.5806`, so every error in the table sits inside one bin. The honest cap below covers this section too, and covers it harder: the zeros are known to far greater precision than a moire can reach, so what the picture buys is a rendering, not a measurement.

## The honest cap

An observable is not a handle. What the last two sections establish is that this picture renders a genuinely RH-equivalent object, which is a real upgrade over the vaguer "fractals and zeta both have self-similar structure" gestures. What it does not do is supply any route to a proof. The Riemann hypothesis is already checked numerically far beyond any range this or any other meter can reach, so the table above can only ever illustrate the expected behaviour - it is consistent with RH, it is not evidence for it, and no amount of extra `Q` changes that. Scored here, the link quality is 6 out of 10 and the meter's tractability 0, and both numbers deserve to be stated together: the connection is exact, and no renderer reaches it - an attack must come through the equidistribution toolkit, never through a picture.

**Two instructions this page hands the rest of the tree.** First: the toolkit flows both ways. The window at dimension one in [coprime](coprime.md) is a discrepancy statement about a discrete arithmetic set and so is Franel-Landau, so the equidistribution methods that attack one are the methods the other needs - that kinship in technique is why an RH equivalence sits on a page of this tree, and it marks the one honest route: theorems, not renders. The verdict above is final for the meter alone; it caps what a picture can claim, never what a proof may attempt. Second: this page renders the Farey set without owning it. The rule whose ORBIT is the Farey set is the mediant, `(a/b, c/d) -> (a+c)/(b+d)`, with the Gauss map `x -> {1/x}` as its continued-fraction twin - simple local rules with emergent complexity, exactly this project's own principle, and they carry the Stern-Brocot and `GL_2(Z)` symmetry that base-`q` digit restriction does not. Mayer's theorem lives there: the Selberg zeta function of the modular surface is the Fredholm determinant of the Gauss-Kuzmin-Wirsing transfer operator, a genuine fractal-dynamics-to-zeta bridge. Two cautions travel with it - that is Selberg zeta and not Riemann zeta, and its RH-analogue is known for unrelated reasons; and the alphabet is infinite, so every finite-state tool on this tree needs rebuilding there.

## Farey order is the stack, not the design

- There is no design-specific Farey sequence, and there never was one to find. **Refuted.**
- The stack's lit set at maximum scale `Q` is exactly `{a/b : 1 <= a <= b <= Q, gcd(a,b) = 1}`, because a boundary coordinate `k/n` reduces to `a/b` and reappears at every scale divisible by `b`.
- Farey order is therefore `Q`, the maximum stacked grid scale. Fill count plays no part, and every design gives the same Farey sequence at fixed `Q`. **Proved** from the construction.
- Brightness `hits(a/b) = floor(Q/b)` is checked by literal stacking at `Q = 30` on all 278 lit fractions and up to `Q = 125` (`lab/farey-discrepancy`). **Verified.**
- Under the transparent convention `Q = 3^L`, the geometric side length, the Landau discrepancy `D_Q = sum_i |f_i - i/m|` reads `0.166667, 0.549206, 1.150760, 2.118500, 3.187070` at `Q = 3, 9, 27, 81, 243`, with `m = 4, 28, 230, 2020, 18056`. Both generation routes agree exactly, a Farey next-term recurrence being the independent cross-check; the rows have no generator in `lab/`. **Conjecture.**
- `D_Q/sqrt(Q)` stays inside `[0.0962, 0.2354]` and reads `0.2045` at `Q = 243`. The adjacent log slope falls `1.085, 0.673, 0.556, 0.372`; the all-five log-log fit is `0.660` and the last-three fit `0.464`. Consistent with `O(Q^{1/2+eps})`, discriminating nothing: five nested deterministic points cannot test a statement quantified over every positive epsilon.
- The `Q = 3^L` map is a comparison convention chosen here, not a mapping the tree defines. Mapping `Q` to fill count would be arbitrary and was explicitly rejected.

## The stack is an address, not a construction

Can a stack be created immediately, without stacking? The answer is yes, exactly, and the boundaries of the yes are theorems of their own; every number in this section is regenerated by `lab/carpet-stack-address`.

Everything layer `n` does at a rational point `x = (a_1/q, a_2/q)` depends only on `r = n mod 2q`: `n` is odd iff `r` is odd, and `floor(n*a/q)` is odd iff `(r*a) mod 2q >= q`, since `n*a mod 2q = q*(floor(n*a/q) mod 2) + (n*a mod q)`. So the odd-carpet stack's brightness is a residue count with the `N`-dependence in closed form,

```
B_N(x) = ceil(N/2) - Sum_{r in S(x), r <= N} (floor((N-r)/2q) + 1)
```

with `S(x)` the bad residues, and the per-point cost depends on `q` alone, never on `N`. **Proved.** The line-stack's own form is the `floor(N/b)` at the top of this page, `O(1)` per node. **Verified** by two generators sharing no code in `lab/carpet-stack-address`, one stacking literally and one forbidden to loop over layers: identical Farey digests at `N = 55` (940 nodes, brightness sum `1540 = N(N+1)/2` landed by count), sha256-identical `512 x 512` renders by three routes, all 48 probes equal at `N = 55` and `5555`, the closed form against literal stacking at `N in {1, 2, 55, 5555, 19945, 19946, 19947, 40001}` with zero mismatches, and a stack of `5*10^17` layers - `N = 10^18` - evaluated in a tenth of a second by both implementations, exactly, values agreeing digit for digit.

The scope is part of the result, each boundary proved. Per-point only: an `R x R` raster costs `R^2` writes no matter what. Exact representations only: on a point supplied as a real oracle the value is undecidable at the discontinuity set `{n*x integer}`, while an irrational with a known continued fraction stays computable by the Ostrowski recursion - the obstruction is representation, not irrationality. Finite `N` only: membership in the infinite-depth limsup set is not decidable. And unweighted only: the Mobius-weighted node of this page carries the Mertens-type sum `Sum_{k <= N/b} mu(kb)`, `M(N)` at `b = 1`, and no polynomial-time algorithm for the Mertens function at binary input is known, the best standing near `x^(2/3)` (Deleglise-Rivat 1996) - the one value on this page without an immediate form, an open computational status and explicitly not a hardness result.

**What immediacy does not buy is the RH question, and the reason is sharp.** The Franel-Landau functional needs each node's rank, and the rank's own closed form is `A(x, Q) = Sum_{d <= Q} mu(d) Sum_{e <= Q/d} floor(x*e)` - classical, **Verified** here at `Q = 12, 25, 40` against brute-force enumeration - so the moment the picture is asked where its nodes sit, Mobius enters the formula. Brightness has a `mu`-free closed form; rank does not; only rank carries the difficulty. Franel's 1924 theorem *is* the symbolic all-`Q` reduction of the discrepancy to Mertens-type sums, so the route "generate every frame at once and read off structure" is not unexplored - it is the proof of the equivalence, and it terminates at Mertens. The Mertens meter's natural global readout collapses outright: `Sum_{n <= N} M(floor(N/n)) = 1` identically (**Proved**, classical Mobius inversion; **Verified** at every `N` through 20000 with zero breaches, `lab/mertens-meter`), so the weighted picture aggregates to a constant and informs only where it presupposes `M`. One steelman deserves its named kill so it is closed: the stack's divisibility incidence array is the Redheffer matrix up to its first column, whose entries were always trivial and whose determinant is `M(n)`, RH iff `M(n) = O(n^(1/2+eps))`. Immediate entries, untouchable determinant - the same wall this page's honest cap already describes, stated in the highest shape-adjacency object this tree carries.

**The complexity frontier runs beside this page, not through it.** Deciding a pixel's brightness with every input in binary is in P: the constraint set is a rational polytope in fixed dimension three (the both-even parity branch summed alongside the both-odd), and lattice-point counting in fixed dimension is polynomial (Barvinok 1994). **Verified** against the literature, with the caution that the tree's `O(q)` residue pass is polynomial in `q` and so exponential in bit-length - a unary-input algorithm, the honest name for what runs in `lab/carpet-stack-address`. The shared scales are the whole engine: moduli `1..N` give the picture polynomially many faces and closed-form extrema, maximum brightness on the diagonal and `floor(N/b)` at `b = 1`. Destroy the sharing - arbitrary binary moduli, one darkened residue class per layer - and "does any point reach maximum brightness" is Simultaneous Incongruences, NP-complete (Garey and Johnson, SP3); make the ambient dimension part of the input and "is any layer lit at this fixed point" is NP-complete (Lagarias 1985), polynomial at every fixed dimension. **Verified** against the literature, both at source. Evaluation stays easy here exactly because the stack shares its scales; hardness begins where the sharing ends, one structural parameter away. A proved no-shortcut theorem for this stack could therefore never have separated P from NP: the problem it would bound is already in P, and what remains bindable there is fine-grained or expressibility only. Nor does the yes touch RH, for the reasons above - both halves of the question were category errors, and each points at the true theorem beside it.
