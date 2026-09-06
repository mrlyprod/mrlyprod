# The hexagon moire

Cut the solid cube of odd side `n` through its centre, perpendicular to the main diagonal, and the section is a regular hexagon; [the slices page](slices.md) counts that mesh and the fills that live on it. Do it at every odd side at once, hold the layers up to the light, and the hexagons interfere. This page is the arithmetic of that interference: which lines the stack lights, which pairs of layers agree, how the agreement decays, and which constants survive to the limit. The layers are the four historical families of [slices](slices.md) - carpet, net, tree and void - and everything below is either an exact statement about one layer or an exact statement about a pair.

Every claim carries a tag. **Proved** means a proof is given or restated here; **Verified** means recomputed from scratch by the study named; **Conjecture** means a fit with no derivation. One generator prints every number on this page, `lab/hexagon-moire`, which runs in about thirty seconds and whose sections are the sections below; a number it does not print is not on this page, and the one place a shelf paper is the source instead says so. [The volume demo](../demos/volume/) stacks a cube design into a solid field and cuts it on any plane, the central diagonal cut being this hexagon, and [the slices demo](../demos/slices/) draws one layer triangle by triangle.

Two frames run through the page and they are not the same frame. The **ideal frame** is the cut plane's own coordinate square: a point is `(X, Z)` in `[0,1]^2`, the layer's cell is `x = floor(4nX)`, `z = 2*floor(2nZ)`, `y = 6n - 2 - x - z`, and the point is on the layer when `0 <= y < 4n`. The **lattice frame** is the rendered raster of the cube's own cells. The plane `x + y + z = 6n - 2` sits half a cell off the true centre, so each layer's hexagon is displaced by `1/(2n)` from the last, and that displacement is real geometry rather than an artifact: a scan raster that ignores it misreads the crosshair strengths, and one quantity below - the star's decay coefficient - has no value at all until the frame is named.

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

**The carpet star's exact decay coefficient is open and frame-dependent. Conjecture.** One frame measures `-1/8` per `ln L` and another `-0.18`, and the ideal frame's own slope is still drifting at `L = 400`. This is a definition problem wearing a measurement's clothes: the coefficient is not a property of the stack until the registration and the band normalization are fixed, and only the `(ln L)/L` decay order survives the frame change. Fixing the frame is the whole of the work here; extrapolating either number is not.

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

**Eisenstein is absent from the base-2 slice stack. Verified.** `L(2, chi_-3) = 0.7813024129`, the L-value [the bases page](bases.md) finds under base 3, appears nowhere in the list above; the only character this slice generates is `chi_4`, and the hexagonal geometry contributes rational tent integrals rather than a second L-function. The hexagon in this page is a shape, not an Eisenstein lattice.

## Where the exactness stops

**The flat analogue is exact and proved on the shelf. Proved.** For the square parity carpets at odd scales, [moire-correlation-laws](https://github.com/carlomitchener/carlomitchener/tree/main/research/moire-correlation-laws) proves that the Pearson correlation of two layers is a closed form in `gcd(m, n)` which vanishes if and only if the scales are coprime, for each of the four fields the plane rule generates. The same paper proves why none of that reaches this page: the plane rule "ink unless both indices are odd" expands as `3/4 + (sigma_1 + sigma_2)/4 - sigma_1 sigma_2 /4`, and the pair term is the entire mechanism, while the space rule "fill if at most one of three indices is odd" expands as `1/2 + (sigma_1 + sigma_2 + sigma_3)/4 - sigma_1 sigma_2 sigma_3/4`, whose three pairwise coefficients are all exactly zero. A triple product takes the pair term's place, and a planar section of the space rule inherits nothing.

What replaces the exactness is the dyadic overtone of the second section, and the numbers depend on the mask. The paper's own hexagonal remark reports measurements on rendered sections, mean `-0.037` over a range `[-0.205, +0.147]`, and labels them measurements on rendered images rather than theorems, the extreme values mask-dependent. **Conjecture.** The exact full-hexagon computation on this page reads `(5, 9) = -0.14179450` and `(5, 7) = -0.08542646` for the same two pairs, so any number published for a hexagon layer pair must pin the mask convention it was measured under. What is robust across masks is the shape of the failure: coprime pairs correlate, the gcd echo survives, and the sign is the dyadic law's.

## Where the numbers live

`lab/hexagon-moire` prints every number above in one run: the exact cut ink of every odd `n <= 55` against the four closed forms in rational arithmetic, the exact full-hexagon Pearson correlations and the doubling sign law, the ideal-frame stack at 3601 samples per axis for the quarter line and the void star, the twisted averages and the coarse crosshair model at 200003 samples, the rendered ghost star on its 1200 by 2399 raster in both frames, and the constants from the `mrlynum` series with the partial character sums at `N = 53` and `N = 55`. Every domain is the source's own. The mesh those layers are counted on, and the fills that live on it, are [slices](slices.md); the ink law's quasipolynomial is proved in [the Walsh spectrometer lane](https://github.com/carlomitchener/carlomitchener/tree/main/research/walsh-spectrometer); the flat stack this one is measured against is the [moire-correlation-laws](https://github.com/carlomitchener/carlomitchener/tree/main/research/moire-correlation-laws) lane; and the L-value it does not contain is [bases](bases.md).
