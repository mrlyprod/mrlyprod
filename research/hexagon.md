# The hexagon moire

Cut the solid cube of odd side `n` through its centre, perpendicular to the main diagonal, and the section is a regular hexagon; [the slices page](slices.md) counts that mesh and the fills that live on it. Do it at every odd side at once, hold the layers up to the light, and the hexagons interfere. This page is the arithmetic of that interference: which lines the stack lights, which pairs of layers agree, how the agreement decays, and which constants survive to the limit. The layers are the four historical families of [slices](slices.md) - carpet, net, tree and void - and everything below is either an exact statement about one layer or an exact statement about a pair.

Every claim carries a tag. **Proved** means a proof is given or restated here; **Verified** means recomputed from scratch by the study named; **Conjecture** means a fit with no derivation. One generator prints every number on this page, `lab/hexagon-moire`, which runs in under a minute and whose sections are the sections below; a number it does not print is not on this page, and the one place a shelf paper is the source instead says so. [The volume demo](../demos/volume/) stacks a cube design into a solid field and cuts it on any plane, the central diagonal cut being this hexagon, and [the slices demo](../demos/slices/) draws one layer triangle by triangle.

Three frames run through the page and they are not the same frame. The **ideal frame** is the cut plane's own coordinate square: a point is `(X, Z)` in `[0,1]^2`, the layer's cell is `x = floor(4nX)`, `z = 2*floor(2nZ)`, `y = 6n - 2 - x - z`, the point is on the layer when `0 <= y < 4n`, and every layer is resampled onto one common grid. The **lattice frame** is the rendered raster of the cube's own cells. The **cell frame** is the cube's own cell lattice read one layer at a time, nothing resampled and no raster, every quantity a ratio of exact cell counts. The plane `x + y + z = 6n - 2` sits half a cell off the true centre, so each layer's hexagon is displaced by `1/(2n)` from the last, and that displacement is real geometry rather than an artifact: a scan raster that ignores it misreads the crosshair strengths, and one quantity below - the star's decay coefficient - is a different number in each of the three.

## The cut ink of one layer

**The cut ink laws of all four families are exact closed forms. Proved.** With `chi = (-1)^((3n-1)/2)`, the inked share of the hexagon at odd side `n` is

```
carpet  1/2 + chi/8 + 1/(2n) - chi/(8n^2)
net     1 - carpet
tree    1/4 + (1/3 - chi/12)/n + (1 - chi)/(6n^2)
void    1/4 - chi/(4n) + 1/(2n^2)
```

and the generator checks each against the counted hexagon in exact rational arithmetic: 28 of 28 layers match for all four families at every odd `n <= 55`. The character `chi` is the same `chi_4` the [Walsh spectrometer](https://github.com/carlomitchener/carlomitchener/tree/main/research/walsh-spectrometer) proves the quasipolynomial by, and the `1/n` and `1/n^2` orders of that law are proved on [slices](slices.md) under the slice ink. The wider range `n <= 101` has no generator, so nothing is claimed there.

The centre cell is the first place the families separate. Over the same 28 layers the carpet's centre cell is ink in 14, the net's in 14, the tree's in 14, and the void's in 28 of 28 - the void's centre is ink at every odd `n`, by a two-line parity argument on the three coordinates. That single cell is the seed of the two star sections below: the carpet's centre agrees with the background and fades, the void's never does. **Verified.**

## Layer pairs and the doubling law

Correlate two layers on the full hexagon, exactly, cell by cell. The seven pairs the generator reads blind are

| pair | `r` | `cov` |
|---|---:|---:|
| `(5,9)` | -0.14179450 | -0.03500978 |
| `(5,7)` | -0.08542646 | -0.01976104 |
| `(9,17)` | -0.12773715 | -0.03098711 |
| `(13,25)` | -0.12529193 | -0.03011364 |
| `(17,51)` | +0.21659159 | +0.05119124 |
| `(5,15)` | +0.21710273 | +0.05156877 |
| `(3,9)` | +0.20367849 | +0.04224750 |

(Verified.) The coprime pairs are not zero, and that is the whole point: on the square stack they are exactly zero, and the reason the hexagon loses it is a theorem of its own, stated under WHERE THE EXACTNESS STOPS.

**The corrected law on the hexagon is dyadic. Verified.** What breaks coprime independence is a hidden half-cell-shifted overtone at doubled frequency, `chi_4(n) s(2nX + 1/2)/8`, that the plane constraint forces into every carpet slice, plus the hexagon's non-product tent marginal; together they couple layer `m` to layers `2m +- 1` and `m +- 2` regardless of gcd. The sign is forced: `sign r(m, 2m +- 1) = -chi_4(m) chi_4(2m +- 1)`, and the generator holds it on 18 of 18 pairs from `(3, 5)` to `(601, 1201)` across all four residue branches. Two of those rows, the largest computed, read `(501, 1001)` at `r = -0.11732362` and `(601, 1201)` at `r = -0.11729091`.

**The doubling magnitude reads between `0.11711630` and `0.11715991`. Verified.** Richardson extrapolation `r = r_inf + a/m + b/m^2` on sliding triples of `m = [157, 201, 301, 401, 501, 601]` returns `r_inf = -0.11715991, -0.11711630, -0.11714296, -0.11711630`, and the two branch extrapolations in `1/m` land on `0.1171270` at `(501, 601)` and `0.1171274` at `(103, 403)`. That kills the rival `19/162 = 0.11728395`, which sits above every reading in that spread and above both branch extrapolations.

**The exact doubling constant is `253/2160 = 0.11712963`. Conjecture.** It matches the two branch extrapolations to `2.6e-6` and `2.2e-6` where `19/162` fails at `1.57e-4`. A derivation of the smear is missing, and until one exists this is a fit that survived its rivals, not a theorem.

## The other pair families

**The breakage is rule-specific in the limit. Verified.** Run the same exact correlation across the four families:

| family | doubling `(201,401)` | adjacent `(199,201)` | adjacent `(249,251)` | echo `(67,201)` | echo `(99,297)` |
|---|---:|---:|---:|---:|---:|
| carpet | -0.11761160 | -0.08115576 | -0.08150224 | +0.21473346 | +0.21476417 |
| tree | +0.00049938 | -0.07016649 | -0.07025454 | +0.14984772 | +0.14929980 |
| void | +0.00107260 | +0.00786627 | +0.00824952 | +0.07721358 | +0.07619141 |

So the persistent doubling coupling is carpet and net only, the tree keeps a neighbour coupling at `(m, m+2)` and no doubling, and the void is essentially independent at both. The gcd echo survives in all four. The controls agree that this is signal and not raster noise: at the coprime pairs `(101, 173)` the three families read `-0.00248023`, `+0.00033407` and `+0.00018183`, and at `(97, 251)` they read `+0.00212976`, `+0.00022204` and `+0.00008585`.

**The adjacent-pair limit is `-11/135 = -0.0814815` and the gcd-echo limit is `29/135 = 0.2148148`. Conjecture.** Both are numerology-grade fits against the carpet rows in the table above, and each needs its own lattice integral before it is anything more.

## The quarter line and the crosshairs

Stack the carpet's layers in the ideal frame at `N = 55`, 3601 samples per axis, and read one-sided bands of width `0.018` against the hexagon mean `0.543621`:

| line | left | right |
|---|---:|---:|
| `X = 0.2500` | +0.066465 | -0.090881 |
| `X = 0.7500` | -0.090863 | +0.066539 |
| `X = 0.3333` | +0.020969 | -0.058390 |
| `X = 0.2000` | +0.023744 | -0.022475 |
| `X = 0.5000` | -0.001222 | -0.001219 |
| `Z = 0.2500` | -0.010910 | -0.011128 |
| `X-Z = 0.2500` | +0.002907 | -0.067923 |
| `X+Z = 1.2500` | -0.045236 | +0.028988 |

**The quarter-line law. Verified.** The strongest interior lines of the stacked hexagram sit at quarter-cell coordinates `a/4`, generally `a/(4b)` with `b` odd, and the step across such a line is an exact one-sided `+-1/8` that every layer votes for identically, because the overtone's `chi_4` sign meets the layer's own `chi_4` and squares away. Read straight at the line the plateau converges: mean magnitude `0.12210, 0.12340, 0.12412, 0.12446` at `N = 151, 301, 601, 1201` against the limit `1/8`, the one-sided pair at `N = 1201` reading `+0.12320` and `-0.12573`. The odd-fraction crosshairs at `(-1)^a/(4q)` follow behind. The `X`, `Z` and `X+Z` profiles are numerically identical on the render by the slice's permutation symmetry, so "in five directions but never horizontal" is false, and the missing-`Z`-overtone statement holds only in the rectangle-cell frame.

**The three 60-degree crosshair families obey a limit law. Verified.** The line at coordinate `a/q` carries strength `(-1)^a/(4q)` for odd `q` and nothing for even `q`. In the coarse cut model, 200003 samples along a line, the excesses converge:

| `A` | `N = 55` | `N = 5555` | predicted |
|---|---:|---:|---:|
| `1/3` | -0.093077 | -0.083726 | -0.083333 |
| `2/3` | +0.065218 | +0.082917 | +0.083333 |
| `1/5` | -0.059505 | -0.050272 | -0.050000 |
| `2/5` | +0.032526 | +0.049651 | +0.050000 |
| `1/7` | -0.058065 | -0.036036 | -0.035714 |
| `1/2` | -0.014353 | -0.000217 | +0.000000 |
| `1/4` | -0.023394 | -0.000231 | +0.000000 |

The `N = 55` column is the whole lesson about frames: `1/7` reads `-0.058065` where the `N = 5555` reading is `-0.036036` against a predicted `-0.035714`, and the even denominators read `-0.014353` and `-0.023394` where the prediction is zero. That gap is the per-layer registration drift of `1/(2n)`, and it is what a drifted scan raster misreads. In registration-correct frames the rays are visible in the real render as well, the `X + Z = 1.25` line at `N = 55` reading `-0.045236` and `+0.028988` and the `X = 1/3` one-sided bands reading `+0.020969` and `-0.058390`. A null claiming no rays at all was about the scan geometry and not about the object.

## The void star

**The void slice stack keeps its star forever. Verified.** In the ideal frame at `N = 55`, 3601 samples per axis, centred bands of half-width `0.004`, against a hexagon mean of `0.278670` whose limit is `1/4`:

| line | ink | ratio to mean |
|---|---:|---:|
| `X = 0.50` | 0.513667 | 1.843 |
| `Z = 0.50` | 0.399416 | 1.433 |
| `X+Z = 1.00` | 0.486492 | 1.746 |
| `X-Z = 0.00` | 0.491844 | 1.765 |
| `2X+Z = 1.50` | 0.491525 | 1.764 |

The law behind the table is six central lines at plateau ink `1/2` against a background of `1/4`, ratio 2, every layer voting on all six, with the model-frame `Z` arm weaker at `3/8` - the `Z = 0.50` row - and a centre dot that is ink at every odd `n`, the `28/28` of the first section. Nothing about this decays. The carpet's star, by contrast, is a finite-layer artifact and fades; the two snowflake stacks differ by a theorem, not by a constant.

**Void and carpet have complementary line spectra on the cut. Verified.** The void's lines sit at even-denominator twisted positions `X = a/(2b)` with `b` odd, exactly where the carpet is silent, and the void is silent at the carpet's odd rationals. The tree carries the only untwisted crosshair family plus a permanent ratio-2 line at `2X + Z = 3/2`, and ignores its free axis. The net is the exact pixelwise complement of the carpet, since "at most one odd" and "at least two odd" exhaust the cases - the same partition that makes carpet and net tile one hexagon in [slices](slices.md).

## The twist that kills the ray family

**The `chi_4` twist kills the pair-resonance ray family. Verified.** Average `chi_4(n) T(nx)` over odd `n <= N` with `T` the layer's tent, and the average falls like `1/N` at every `x`, rational or not:

| `x` | `N = 55` | `N = 2001` | `N = 40001` | untwisted at `N = 40001` |
|---|---:|---:|---:|---:|
| `0` | +0.00000 | +0.000999 | +0.0000500 | +1.000000 |
| `1/3` | +0.04762 | +0.000333 | +0.0000833 | -0.111111 |
| `2/3` | -0.04762 | -0.000333 | -0.0000833 | +0.111111 |
| `1/5` | +0.02857 | +0.000599 | +0.0000300 | -0.039968 |
| `1/7` | -0.00000 | +0.001284 | +0.0000357 | -0.020363 |
| `1/2` | +0.00000 | +0.000000 | +0.0000000 | +0.000000 |
| `1/4` | +0.00000 | +0.000500 | +0.0000250 | +0.000025 |
| `1/9` | -0.00000 | +0.000333 | +0.0000167 | -0.012294 |
| `0.414214` | +0.09630 | +0.000421 | +0.0000443 | -0.000020 |

The last row is irrational and behaves like the rest. So the snowflake stack has no analogue of the carpet stack's bright main diagonal: in the coarse cut model the `A = C` excess goes `-0.003580` at `N = 55` to `-0.000437` at `N = 555` to `-0.000052` at `N = 5555`, with the background falling `0.550865, 0.507266, 0.500942` to its limit `1/2`. What is left is only the three single-wave crosshair families parallel to the hexagon's edge directions, one per lattice axis, at odd-denominator rational coordinates - the families the previous section measures.

## The ghost star

**The ghost star at the hexagon's centre is a finite-layer artifact. Verified.** Each layer's centre is entirely ink or entirely paper, flipping with `n mod 4`, so 28 layers give exactly `1/2`, which is also the limiting background; the star is the difference between the two, and it goes to zero. On a 1200 by 2399 raster of the carpet's cut layers at odd `n <= 111`, with 2147316 hexagon pixels of which 25440 are star and 1981584 background:

| layers | star | background | star minus background |
|---|---:|---:|---:|
| 5 | 0.57832 | 0.67217 | -0.09385 |
| 28 | 0.51730 | 0.54832 | -0.03102 |
| 56 | 0.50873 | 0.52629 | -0.01756 |

The decay is `(ln L)/L`, and the coefficient is where the frame bites. In the ideal frame, main diagonal `X = Z`, band half-width `0.01`, 2801 samples per axis, `excess * L` runs `-0.7779, -0.9942, -1.1194, -1.2519` at `L = 28, 100, 200, 400`, with slopes against `ln L` of `-0.1807` from 100 to 200 and `-0.1911` from 200 to 400. In the lattice frame, the band `|x - y| <= 0.01` of the cube side read against the exact ink law layer by layer, `excess * L` runs `-0.9337, -1.0212, -1.1387, -1.1923, -1.2791, -1.3645` at `L = 14, 28, 56, 100, 200, 400`, with slopes `-0.1252` from 100 to 200 and `-0.1233` from 200 to 400.

**The cell frame, defined.** Registration: the cube's own cell lattice, one layer at a time, nothing resampled onto a common grid, so the half-cell displacement `1/(2n)` between consecutive layers is carried rather than averaged away. Band normalisation: the star intensity is the ink share of the exact arm `x = y` of the cut, a diameter of `2n` cells with no width to choose, and the background intensity is the layer's whole-hexagon ink share, the Proved closed form of the first section; the excess is star minus background, and the `L`-layer excess is its mean over the first `L` odd layers.

**The star arm's ink is exactly `1/2 + chi_8(n)/(2n)`. Proved.** The arm `x = y` at odd `n` is the `2n` cells with `z = 6n - 2 - 2x`, and `0 <= z < 4n` pins `n <= x <= 3n - 1`. On the arm the two block parities agree, so "at most one of three odd" collapses to `floor(x/4)` even, and with `f(m) = 4 floor(m/8) + min(m mod 8, 4)` the inked count is `f(3n) - f(n) = n + chi_8(n)`, where `chi_8` is the real character mod 8 of `Q(sqrt 2)`, `+1` at `n = 1, 7` and `-1` at `n = 3, 5`. The generator checks that rational against the counted arm at every odd `n <= 2001`, 1001 of 1001, and the three arms `x = y`, `y = z`, `z = x` return the same rational at every odd `n <= 221`, 111 of 111, by the slice's permutation symmetry.

**In the cell frame at even `L` the decay coefficient is exactly `-1/4`, with an exact constant beside it. Proved.** Writing `excess_L` for the `L`-layer excess, at even `L`

```
L * excess_L = -(ln L)/4 + C + O(1/L^2)
C = ln(1 + sqrt 2)/(2 sqrt 2) - G/8 - gamma/4 - (ln 2)/2 = -0.2937605857
```

| `L` | `excess * L` | `excess * L + (ln L)/4` | residual `* L^2` |
|---|---:|---:|---:|
| 28 | -1.126964 | -0.2939128437 | -0.11937029 |
| 100 | -1.445065 | -0.2937725615 | -0.11975831 |
| 400 | -1.791627 | -0.2937613344 | -0.11978958 |
| 1600 | -2.138200 | -0.2937606325 | -0.11979154 |
| 6400 | -2.484774 | -0.2937605886 | -0.11979188 |

The proof is one subtraction of closed forms. The per-layer excess is `-chi/8 + (chi_8(n) - 1)/(2n) + chi/(8 n^2)`; summed over the first `L` odd `n` the first term is `0` at even `L` and `+1/8` at odd `L`, the character part converges to `L(1, chi_8)/2 = ln(1 + sqrt 2)/(2 sqrt 2)` by the class number formula for `Q(sqrt 2)`, the third converges to `-G/8` with `G` Catalan's constant, and `-sum 1/(2n)` over the odd `n` below `2L` is `-(ln L)/4 - gamma/4 - (ln 2)/2 + O(1/L^2)`, the `1/(4L)` terms of the two harmonic sums cancelling exactly. Every limb of `C` is a closed form that subtraction hands over, so `C` is derived and not a decimal recognised after the fact, and the whole of the ghost star's decay is the `1/(2n)` of the hexagon's ink law read against an arm that has none. The generator prints `L(1, chi_8) = 0.62322524` from its own series against `ln(1 + sqrt 2)/sqrt 2 = 0.62322524`.

**Even `L` is a hypothesis and not decoration. Proved.** The `-chi/8` term is the whole of the difference: at odd `L` it sums to `+1/8`, so the law reads `C + 1/8` in place of `C`, and what is left is `+-1/(4L)` rather than `O(1/L^2)`. The `-1/4` coefficient is untouched by parity. The generator's odd ladder, columns `L * excess_L + (ln L)/4 - C` and that miss less `1/8` times `L`:

| `L` | miss | `(miss - 1/8) * L` |
|---|---:|---:|
| 27 | +0.115715 | -0.250708 |
| 99 | +0.122472 | -0.250244 |
| 401 | +0.125623 | +0.249934 |
| 1601 | +0.125156 | +0.249984 |
| 6399 | +0.124961 | -0.250004 |

**The `1/L^2` term is exact and reads `L` mod 4. Proved.** Three tails carry it: the character tail `-(1/2) sum chi_8(n)/n` over odd `n > 2L` is `-1/(8L^2)` at `L = 0 mod 4` and `+1/(8L^2)` at `L = 2 mod 4`, since the sign pattern `+--+` on the four odd residues starts at `n = 2L + 1` and flips with that residue; the Catalan tail contributes `+1/(64 L^2)` and the harmonic remainder `-1/(96 L^2)`, both blind to the residue. So the coefficient is `-1/8 + 1/64 - 1/96 = -23/192` at `L = 0 mod 4` and `+1/8 + 1/64 - 1/96 = +25/192` at `L = 2 mod 4`. The residual column above is every row at `L = 0 mod 4` and falls by `4.00` at all six doublings from `L = 100` onto `-23/192 = -0.11979167`, the `L = 6400` row overshooting at the floor of double precision where the residual itself is `3e-9`; the generator's `L = 2 mod 4` ladder reads `+0.13020778, +0.13020819, +0.13020825` at `L = 802, 1602, 3202` against `+25/192 = +0.13020833`.

**There is no frame-free decay coefficient. Refuted.** Widen the star from the exact arm to a band of half-width `W` cells about it, change nothing else, and the coefficient walks over a family:

| `W` | 0 | 2 | 4 | 6 | 8 | 10 | 12 | 16 | 20 | 24 | 32 | 64 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| coefficient | `-1/4` | `-1/6` | `-1/10` | `-3/28` | `-5/36` | `-3/22` | `-3/26` | `-9/68` | `-5/42` | `-13/100` | `-17/132` | `-33/260` |

Every entry is an exact rational the measurement converges on rather than a fit: recomputed to `L = 3200`, four times the sweep, `W = 0` through `12` land on `-1/4, -1/6, -1/10, -3/28, -5/36, -3/22, -3/26` within `5.1e-8`, every gap falling by four per doubling of `L` from `L = 800`. **Verified.** The sweep's own `8.1e-7` is the residual of a best-rational search over denominators to `400`, which sits within about `1/400^2 = 6.2e-6` of a generic real, so it is a search residual and never was an error bound. Beside them a band whose half-width is a fixed fraction of the cube side rather than a fixed number of cells reads `-0.1252` and `-0.1233`, and the ideal frame, which resamples every layer onto one 2801-point grid, reads `-0.1807` and `-0.1911` and is still moving at `L = 400`. Resample the layers or do not, and pick the star's width: more than a dozen coefficients come out of one star, so the coefficient is a property of the measurement and not of the stack. What survives the frame change is the `(ln L)/L` order, the sign, and the largest coefficient of them all, the cell frame's `-1/4`, which is the only one with a closed form.

**The width family at even `W` is `-(W + 2b)/(8(W + 1))`. Conjecture.** `b` is the block parity of the band edge, `1` when `floor(W/4)` is even and `0` when it is odd, the same parity bit the space rule reads. Even `W` is a hypothesis: `x - y` is even on the arm, so half-width `1` is the same point set as the arm itself and reads `-1/4` where the formula says `-3/16`, and `W = 3` reads `-1/6` against `-5/32`. The twelve widths of the table are fit and test at once, since the formula was read off them, so the support that counts is held out: `W = 14, 18, 28, 36`, predicted `-7/60, -5/38, -7/58, -9/74` before any measurement, come in at `L = 1600` within `4.0e-8`, 4 of 4. The family accounts for both ends, for its smallest magnitude `-1/10` at `W = 4`, one block, and for the limit: as `W` grows it tends to exactly `-1/8`, which is what the fixed-fraction band of the lattice frame measures at `-0.1252` and `-0.1233`. No derivation exists, so this predicts rather than proves.

## The constants

**The slice stacks' surviving constants are Leibniz, odd Basel and Catalan. Proved.** Every character series below runs over the exact ink laws of the first section, so the limits are theorems about those closed forms and not fits; the carpet split converges at balanced layer counts, and `N = 55` is 28 layers.

| quantity | closed form | value | measured |
|---|---|---:|---:|
| carpet split `M (I1 - I3 + 1/4)` | `pi/4 + pi^2/32` | 1.0938233009 | 1.0826656664 at `N = 55` |
| void background `M (mean - 1/4)` | `(pi + pi^2)/16` | 0.8131998159 | 0.8065045723 at `N = 55` |
| tree background | `pi/48 + pi^2/48 + G/6` | 0.4237275377 | |
| flat-stack control | `pi^2 ln 2 / (7 zeta(3))` | 0.8130217042 | |

The third and fourth rows are the reason the void's constant has to be stated to eight places: `(pi + pi^2)/16 = 0.8131998159` and `pi^2 ln 2/(7 zeta(3)) = 0.8130217042` collide in the fourth decimal and are different numbers, the second belonging to the flat square stack and not to this one.

**The Catalan statement holds only along one residue class. Conjecture.** `M (mean ink - 1/2 - eps/2) -> G/8 = 0.1144956993` along `N = 3 mod 4`, measured `0.1144757884` at `N = 55`; along `N = 1 mod 4` the limit is `G/8 - 1/8 = -0.0105043007`, measured `-0.0104828892` at `N = 53`. `G = 0.9159655942` is Catalan's constant from its own series.

**Eisenstein is absent from the base-2 slice stack. Verified.** `L(2, chi_-3) = 0.7813024129`, the L-value [the bases page](bases.md) finds under base 3, appears nowhere in the list above; the two characters this slice generates are `chi_4`, from the ink law, and the mod-8 character of `Q(sqrt 2)`, from the star arm above, and the hexagonal geometry contributes rational tent integrals rather than an Eisenstein L-function. The hexagon in this page is a shape, not an Eisenstein lattice.

## Where the exactness stops

**The flat analogue is exact and proved on the shelf. Proved.** For the square parity carpets at odd scales, [moire-correlation-laws](https://github.com/carlomitchener/carlomitchener/tree/main/research/moire-correlation-laws) proves that the Pearson correlation of two layers is a closed form in `gcd(m, n)` which vanishes if and only if the scales are coprime, for each of the four fields the plane rule generates. The same paper proves why none of that reaches this page: the plane rule "ink unless both indices are odd" expands as `3/4 + (sigma_1 + sigma_2)/4 - sigma_1 sigma_2 /4`, and the pair term is the entire mechanism, while the space rule "fill if at most one of three indices is odd" expands as `1/2 + (sigma_1 + sigma_2 + sigma_3)/4 - sigma_1 sigma_2 sigma_3/4`, whose three pairwise coefficients are all exactly zero. A triple product takes the pair term's place, and a planar section of the space rule inherits nothing.

What replaces the exactness is the dyadic overtone of the second section, and the numbers depend on the mask. The paper's own hexagonal remark reports measurements on rendered sections, mean `-0.037` over a range `[-0.205, +0.147]`, and labels them measurements on rendered images rather than theorems, the extreme values mask-dependent. **Conjecture.** The exact full-hexagon computation on this page reads `(5, 9) = -0.14179450` and `(5, 7) = -0.08542646` for the same two pairs, so any number published for a hexagon layer pair must pin the mask convention it was measured under. What is robust across masks is the shape of the failure: coprime pairs correlate, the gcd echo survives, and the sign is the dyadic law's.

## Where the numbers live

`lab/hexagon-moire` prints every number above in one run: the exact cut ink of every odd `n <= 55` against the four closed forms in rational arithmetic, the exact full-hexagon Pearson correlations and the doubling sign law, the ideal-frame stack at 3601 samples per axis for the quarter line and the void star, the twisted averages and the coarse crosshair model at 200003 samples, the rendered ghost star on its 1200 by 2399 raster, its decay in the ideal, lattice and cell frames with the arm ink law in exact rationals to `n = 2001`, the cell frame's ladders at `L = 0 mod 4`, `L = 2 mod 4` and odd `L`, the swept band widths with the odd-`W` rows, the recompute to `L = 3200` and the four held-out widths, and the constants from the `mrlynum` series with the partial character sums at `N = 53` and `N = 55`. Every domain is the source's own. The mesh those layers are counted on, and the fills that live on it, are [slices](slices.md); the ink law's quasipolynomial is proved in [the Walsh spectrometer lane](https://github.com/carlomitchener/carlomitchener/tree/main/research/walsh-spectrometer); the flat stack this one is measured against is the [moire-correlation-laws](https://github.com/carlomitchener/carlomitchener/tree/main/research/moire-correlation-laws) lane; and the L-value it does not contain is [bases](bases.md).
