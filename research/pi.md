# Pi out of the stack

Pi is not inside a single carpet's area, which is rational at every level. Pi is in the **stack** of carpets, and it comes out as a counted number rather than an assumed one.

Every claim below is tagged. **Proved** means proved or classical; **Verified** means recomputed here and reported as measured; **Conjecture** means supported and open.

## A single carpet cannot hold pi

Take any self-similar fractal that keeps `k` of the `b^d` subcells at every level. Its filled fraction at level `n` is exactly `(k/b^d)^n` - rational at every level, and its limit is `0` or `1`. The carpet keeps 8 of 9, the sponge 20 of 27, so the level-`n` fractions are `(8/9)^n` and `(20/27)^n`. No irrational constant can be read off a sequence of that shape. (**Proved**, elementary.)

The classical fractal that *does* give pi works differently: the Wallis sieve changes its ratio at every level, so its area is a genuine infinite product,

```
Product_{n >= 1} (1 - 1/(2*n + 1)^2) = pi/4
```

recomputed to `0.785398261` at two million factors against `pi/4 = 0.785398163`; the identity itself is Wallis, **Proved**, and the truncated product has no generator in `lab/`, so that figure is **Conjecture**. A fixed-ratio carpet has one ratio, not a product of changing ones. That is the whole reason a hunt inside one carpet fails, and it is worth logging so nobody chases it.

## What the stack lights up

Stack the grid at scales `k = 1, 2, 3, ...`. Every lattice point `(a, b)` factors uniquely as `g * (a/g, b/g)` with `g = gcd(a, b)`, so it belongs to exactly one layer of the stack, and the points that appear in the first layer are exactly those with `gcd(a, b) = 1` - equivalently, the points visible from the corner, with no nearer lattice point on the same ray. The stack is therefore a partition of the grid into scaled copies of the lit set. (**Proved**, elementary.) Checked numerically as `sum_{k >= 1} lit(floor(N/k)) = N^2`, exact at `N = 10, 100, 1000, 3000`; the check has no generator in `lab/`, so it stands as **Conjecture** while the partition itself is proved.

The density of those lit points is the classical coprimality density,

```
lit / N^2 -> 1/zeta(2) = 6/pi^2 = 0.6079271018...
```

(**Proved**, classical - Dirichlet and Mertens.) Inverting it turns the picture into an estimator:

```
pi = sqrt( 6 / density ) = sqrt( 6 * N^2 / lit )
```

The constant is classical; what the stack adds is a picture, so pi is counted out of the grid rather than imposed on it. The same visibility fact anchors the base-3 story in [what base 3 hides](bases.md).

## The numbers

Lit points counted by a totient sieve as `2 * sum_{k=1..N} phi(k) - 1`, which is [A018805](https://oeis.org/A018805), cross-checked against a brute `gcd` scan at `N = 10, 50, 100, 200` with exact agreement.

| N | lit points | density | pi estimate | abs error |
|---|---|---|---|---|
| 10 | 63 | 0.6300000000 | 3.08606700 | 5.6e-02 |
| 100 | 6087 | 0.6087000000 | 3.13959750 | 2.0e-03 |
| 1000 | 608383 | 0.6083830000 | 3.14041534 | 1.2e-03 |
| 10000 | 60794971 | 0.6079497100 | 3.14153424 | 5.8e-05 |
| 20000 | 243180791 | 0.6079519775 | 3.14152838 | 6.4e-05 |
| 100000 | 6079301507 | 0.6079301507 | 3.14158478 | 7.9e-06 |
| 200000 | 24317197835 | 0.6079299459 | 3.14158531 | 7.3e-06 |

At `N = 100000` the stack gives `3.14158...`, six correct figures against `pi = 3.14159265...`. The rows to `N = 10000` are terms of A018805, **Verified**; the three larger rows have no generator in `lab/` and stand as **Conjecture**.

It is a slow estimator and an honestly noisy one. The error term in the coprime count is `O(N log N)`, so accuracy improves only like `1/N`, and because that term fluctuates arithmetically the approach is not monotone: `N = 20000` is *worse* than `N = 10000` in the table above, and `N = 200000` barely improves on `N = 100000` (**Conjecture** as a table, on the rows above). That puts it in the same family as the Wallis product and the Leibniz series - correct, convergent, not fast. The point was never speed.

## The dimension picks the zeta

The same count in `d` dimensions has density `1/zeta(d)` (**Proved**, classical, by Mobius inversion). So the stack is a geometric generator for the whole zeta family, and the dimension you stack in decides which constant falls out. Counted by `sum_{k=1..N} mu(k) * floor(N/k)^d`, cross-checked against brute `gcd` at `N = 10, 40` for `d = 2, 3`:

| d | density at N = 100000 | limit | constant recovered | true value |
|---|---|---|---|---|
| 2 | 0.6079301507 | `6/pi^2` | `pi = 3.14158478` | `3.14159265` |
| 3 | 0.8319084771 | `1/zeta(3)` | `zeta(3) = 1.20205531` | `1.20205690` |
| 4 | 0.9239388718 | `1/zeta(4)` | `pi = 3.14159226` via `pi = (90/density)^(1/4)` | `3.14159265` |

(**Conjecture** as a table: recomputed, with no generator in `lab/`.) Two things follow. The 3D sponge stack does **not** give pi: it gives Apery's constant `zeta(3)`, which is not known to be a rational multiple of any power of pi - the same parity barrier that blocks Catalan's constant (**Conjecture**; no impossibility proof exists, and none is expected to be easy). And every even dimension is a fresh pi route: `zeta(4) = pi^4/90` recovers pi to seven figures at `N = 100000`, an order better than the 2D count at the same `N`, on the same table.

## Pi to the fourth, out of one design

Every route above runs through `gcd`. One more pi lives in the stack and never mentions coprimality at all. Take the design with filled corners `(0,0)` and `(1,1)` - code 9, the diagonal - and truncate its parity pattern to an `n x n` grid: cell `(i, j)` is filled when `i` and `j` have the same parity. The filled fraction `rho(n) = fill(n)/n^2` tends to `1/2`, and the fluctuation around that limit is exact at every `n`: zero at every even `n`, and exactly `1/(2*n^2)` at every odd `n` - `fill(2m) = 2*m^2` on the nose, while `fill(2m-1) = m^2 + (m-1)^2` and the numerator of `rho - 1/2` collapses to `1`. (**Proved**, elementary.) Weight the fluctuations into a Dirichlet series and only the odd terms survive, each equal to `1/(2*n^(s+2))` - the fluctuation times the weight, two different things:

```
Z(s) = Sum_{n >= 1} (rho(n) - 1/2) / n^s = (1/2) * lambda(s+2),  Re(s) > -1
```

with `lambda(s) = (1 - 2^(-s)) * zeta(s)` the Dirichlet lambda function. At `s = 2` and `s = 4`:

```
Z(2) = pi^4/192 = 0.50733901580...
Z(4) = pi^6/1920 = 0.50072353832...
```

**Proved**; `lab/gaussian-zeta` checks the fluctuation exact in rational arithmetic to `n = 80`, the reductions `1/192` and `1/1920` exact, both constants matched at 90 digits with two independent pi routines and two independent lambda routes agreeing to `1.3e-82` and `5.2e-87`, and the series summed straight off counted fills with no closed form anywhere - gap `8.3e-11` at `n <= 999`, the size of the neglected tail.

One caution is load-bearing: the constant belongs to the corner set, not the symmetry class. The orbit mate - corners `(0,1)` and `(1,0)`, code 6 - has fluctuation exactly `-1/(2*n^2)` on odd `n`, so its `Z(2)` is `-pi^4/192`: same shape under cube symmetry, opposite sign under truncation to `n` cells, the fixed-`n` caveat [the core](core.md) already records. (**Verified**, `n = 1..40`, `lab/gaussian-zeta`.) And the machine is genuinely different from the stack above - a Dirichlet series over one design's own fill fluctuation across truncations, with no visibility and no Mobius anywhere in it. A second, disjoint way for the grid to know pi.

## Where the fractal versions stand

Restricting the count from the full grid to the cells of a fractal keeps the pi but changes the constant. The box-bound theorem in [the coprimality spine](coprime.md), "Closed above dimension one", proves the density for every design with `k > q`, so the row below is a theorem and the conjectural tail of the family is only its `k <= q` members. Each one is a rational multiple of `6/pi^2`, the multiplier being the design's own bracket against the base factor, exactly as [the coprimality spine](coprime.md) derives it (**Proved**, exact arithmetic). The one member of the family whose sequence reached the OEIS is the only one tabulated here; the rest are counted by that page and deliberately not given their constants anywhere in this tree, because their novelty is still live.

| entry | density | as a multiple of `6/pi^2` |
|---|---|---|
| A396934, Sierpinski triangle | `16/(3*Pi^2) = 0.5403796`, **Proved** | `8/9` |

**A coincidence to name before a reader mistakes it for support.** The first row's `16/(3*Pi^2)` has a namesake in the other direction: `1 - 16/(3*Pi^2) = 0.4596204` is [OEIS A395134](https://oeis.org/A395134), an 1891 geometric-probability constant of Zerr, and the A396934 entry carries the crossref. That is a **numeric coincidence and not evidence for this row**. The two constants are complements of one another by arithmetic alone - both recomputed by `lab/half-ball-mismatch`, `16/(3*Pi^2) = 0.5403796460924681` and `1 - 16/(3*Pi^2) = 0.4596203539075319` - and Zerr's problem is not this lane's problem. The row is a theorem by the box-bound theorem and gains nothing from the crossref. A395134 at source, **Verified**: name *"Decimal expansion of the probability that the line that passes through two points selected independently and uniformly at random in a half-disk intersects the arc at two points"*, formula *"Equals 1 - 16/(3*Pi^2)"*, digits `4, 5, 9, 6, 2, 0, 3, 5, 3, 9, ...`, and the Zerr attribution is a link rather than a formula line - *"George B. McClellan Zerr, Solution to Problem 11134, Mathematical questions and solutions from the 'Educational Times', Vol. 55 (1891), p. 161"*.

**The coincidence has a mechanism and a fence. The reading above is unchanged - it is still a coincidence and still not evidence for this row - but the two `16/(3*Pi^2)` are now known to draw their `Pi^2` from different places, and no other dimension can repeat the collision.** Zerr's side decomposes. Let `H` be the upper unit half-disk, `|H| = Pi/2`. The Blaschke-Petkantschin formula for two points gives `P(the line through them crosses the diameter) = I_diam / (3*Area(H)^2)`, where `I_diam` is the chord-cube integral over lines meeting the flat face. Parameterising those lines by crossing point `a` and angle, and substituting `u = cos(phi)`, the inner integral is `Integral_{-1}^{1} (-a*u + sqrt(1 - a^2 + a^2*u^2))^3 du`, and that integral is the constant `2` for every `a` in `[-1,1]`: the odd terms of the expanded cube vanish, and the even part is the exact derivative `d/du [ u*R(u)^(3/2) ]` with `R(u) = 1 - a^2 + a^2*u^2`, which evaluates at the endpoints by `R(+-1) = 1`. Hence `I_diam = 4`, a pure integer, and `P = 4/(3*Pi^2/4) = 16/(3*Pi^2)`. **Proved**, elementary throughout - parity, one binomial expansion, the product rule, the fundamental theorem of calculus - with a second proof by differentiation under the integral sign in `a` and a 50-digit check at 50 values of `a`; `lab/half-ball-mismatch` regenerates the three symbolic residuals, exactly zero, and the 50-digit check. So Zerr's `Pi^2` is `Area(half-disk)^2 = Pi^2/4` and nothing else; the design's `Pi^2` is `zeta(2) = Pi^2/6` and nothing else, `(8/9)/zeta(2) = (8/9)*(6/Pi^2)`. One number, two unrelated mechanisms.

**The mismatch theorem.** The correspondence cannot generalise, and that is the useful half. A design's density is `delta = B(F)*(1/zeta(D))*Prod_{p|q}(1 - p^(-D))^(-1)`, a rational multiple of `1/zeta(D)`: for even `D` that is `rational/Pi^D`, for odd `D` it is a rational multiple of an irrational `zeta(D)` not known to be algebraic over `Q(Pi)`. The half-ball probabilities have their `Pi` pinned low - Version L gives `rational/Pi^2` at even `d` and a pure rational at odd `d`, Version H gives `Q + Q/Pi^2` at even `d` and `Q + Q*Pi` at odd `d`. At even `D >= 4`, equating `rational/Pi^D` with any `Q`-linear combination of `{1, 1/Pi^2}` makes `Pi` algebraic, contradicting Lindemann. **Proved.** At `D = 3` against Version L, `rational/zeta(3) = 3/8` forces `zeta(3)` rational, contradicting Apery. **Proved.** At `D = 3` against Version H, `rational/zeta(3) = a + b*Pi` would make `zeta(3)` algebraic over `Q(Pi)`, which no theorem forbids and every standing conjecture does. **Conjecture-conditional**, and it is the one gap in the theorem. At odd `D >= 5` the argument is the `D = 3` one with `zeta(D)` in place of `zeta(3)`, so it is conditional on `zeta(D)` irrational, which Rivoal and Zudilin give only for infinitely many odd `D` and not for each. At `D = d = 2` both sides live in `Q(1/Pi^2)` and a match is possible; the gasket's is the only one found, checked over all base-2 and base-3 designs at `D = 2` against every Version L value to `d = 24`. **Verified** by `lab/half-ball-mismatch`: 11 base-2 and 502 base-3 designs, base-2 numerators `4, 16/3, 6, 8`, exactly one match, `16/3` at `d = 2`, carried by 3 designs. The corollary worth carrying: hunting for a Euclidean body whose chord probability reproduces some other design's density is provably futile above `D = 2`, so that search is closed rather than merely unfinished.

| Version L, `d` | `P(flat face)` | form |
|---|---|---|
| 2 | `16/(3*Pi^2) = 0.5403796` | `rational/Pi^2` |
| 3 | `3/8` | rational |
| 4 | `128/(45*Pi^2) = 0.2882025` | `rational/Pi^2` |
| 5 | `15/64` | rational |
| 6 | `1024/(525*Pi^2) = 0.1976246` | `rational/Pi^2` |
| 7 | `175/1024` | rational |

| Version H, `d` | `P(hyperplane misses the flat face)` | form |
|---|---|---|
| 2 | `1 - 16/(3*Pi^2)` | `Q + Q/Pi^2` |
| 3 | `4 - 19845*Pi/16384 = 0.1947689081` | `Q + Q*Pi` |
| 4 | `4 - 549978112/(14189175*Pi^2) = 0.0727502984` | `Q + Q/Pi^2` |
| 5 | `16 - 178919214166875*Pi/35184372088832 = 0.0244047160` | `Q + Q*Pi` |
| 6 | `16 - 10363195833496113250304/(65656392092180764875*Pi^2) = 0.0074784083` | `Q + Q/Pi^2` |
| 7 | `64 - 403492347953923610203877211975*Pi/19807040628566084398385987584 = 0.0021206659` | `Q + Q*Pi` |

Version L is the classical family and its parity law follows from Wallis recurrences, `f(2) = 16/(3*Pi^2)` with `f(2k+2)/f(2k) = 4k(k+1)/((2k+1)(2k+3))`, and `f(3) = 3/8` with `f(2k+3)/f(2k+1) = (2k+1)(2k+3)/((2k+2)(2k+4))`; it is **Verified** in exact rationals to `d = 11` by `lab/half-ball-mismatch`, with an independent `10^7`-sample random-point check at `d = 2..7`. Version H is new here and is **Verified** at `d = 2..7`, derived by an unoriented-normal Blaschke-Petkantschin reduction and integrated in closed form by `lab/half-ball-mismatch`, cross-checked by 60- and 80-digit quadrature agreeing to `2.3e-62`, `7.2e-64` and `1.5e-63` at `d = 3, 4, 5`, and by an independent `10^8`-sample random-point estimate whose deviations `2.39e-6`, `-6.74e-6`, `-3.55e-6` sit inside one sigma of `3.96e-5`, `2.6e-5`, `1.54e-5`. The closed forms at `d = 6` and `d = 7` are `16 - 10363195833496113250304/(65656392092180764875*Pi^2) = 0.0074784083` and `64 - 403492347953923610203877211975*Pi/19807040628566084398385987584 = 0.0021206659`, so the Version H parity law rests on six terms; as a law for every `d` it is **Conjecture**.

[coprime.md](coprime.md) upgrades the restricted densities, as above. The clean result is the unrestricted one: on the full stack the density is a theorem, and pi drops out of it by counting.

> A hunt for pi in one carpet's area finds a rational. Pi is one level up, in the agreement between scales - and agreement between scales is coprimality.
