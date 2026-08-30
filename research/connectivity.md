# Structure against noise

A design's fractal occupies some number of cells of a grid. A random set can occupy exactly the same number of cells of the same grid. Race the two on measurable geometry - how many connected pieces, how much boundary per cell - and the question of what the parity rule buys gets a number instead of an adjective.

Every claim below carries a tag. **Proved** means a proof is given or restated here; **Verified** means recomputed from scratch by a lab study; **Conjecture** means neither. Everything on this page is measurement at finite sizes; the one Proved line of the race is flagged where it occurs. The [graphs demo](../demos/graphs/) draws the cell network of any design live, flat, in the cube and on the diagonal slice, with its tips, junctions, pieces and length beside it.

## The race

Five designs, each built by substituting its level-1 tile into itself and cross-checked against the digit rule that defines it - codes and corner order as in [the core](core.md):

| design | base | `D` | rule |
|---|---|---|---|
| gasket | 2 | 2 | code 7, corners `(0,0)`, `(0,1)`, `(1,0)` |
| diagonal | 2 | 2 | code 9, corners `(0,0)`, `(1,1)` |
| seven-of-eight | 2 | 3 | every corner but one |
| carpet | 3 | 2 | 8 of 9 cells - `mrly_bang_d2_7` read at base 3 |
| sponge | 3 | 3 | the Menger sponge, 20 of 27 cells |

The control is matched exactly, not approximately: a uniform sample of exactly `N` distinct cells, the occupied count asserted equal to the design's on every draw - not a Bernoulli field with matching probability. Components are face-connected, 4-neighbour in 2D and 6-neighbour in 3D. Boundary per cell is `(2*D*N - 2*E)/N` with `E` the number of face-adjacent occupied pairs and the grid exterior counting as void, so an isolated cell contributes the full `2*D`. Every random figure below is a mean and a sample standard deviation: 400 seeds up to 729 cells, 200 at the 2187-cell `128 x 128` gasket, the 2401-cell `16^3` seven-of-eight and the 128-cell `128 x 128` diagonal, 100 at 6561 cells and at the `27^3` sponge, 20 at the `81^3` sponge (`lab/percolation-race`).

Two independently written passes race the designs inside one program, `lab/percolation-race` - Kronecker powers with union-find and a PCG64 control, and substitution over coordinate sets with breadth-first search and a Mersenne Twister control - and agree on every comparison to within a standard deviation; the widest gap printed is 0.2061 standard deviations. The numbers printed here are the second pass.

## Connectivity

Every self-similar design above except the diagonal - built to scatter, and raced on boundary instead - is a single component at every size tested. The matched random set is not close, and the gap widens with size in every family. (**Verified**, `lab/percolation-race`.)

| design | grid | cells | density | design comps | random comps |
|---|---|---|---|---|---|
| gasket | `32 x 32` | 243 | 0.2373 | 1 | `134.39 +/- 7.34` |
| gasket | `64 x 64` | 729 | 0.1780 | 1 | `478.09 +/- 12.62` |
| gasket | `128 x 128` | 2187 | 0.1335 | 1 | `1613.61 +/- 21.96` |
| gasket | `256 x 256` | 6561 | 0.1001 | 1 | `5257.23 +/- 32.94` |
| seven-of-eight | `8^3` | 343 | 0.6699 | 1 | `2.80 +/- 1.43` |
| seven-of-eight | `16^3` | 2401 | 0.5862 | 1 | `23.77 +/- 4.85` |
| carpet | `27 x 27` | 512 | 0.7023 | 1 | `9.43 +/- 2.89` |
| carpet | `81 x 81` | 4096 | 0.6243 | 1 | `146.58 +/- 12.52` |
| sponge | `27^3` | 8000 | 0.4064 | 1 | `560.80 +/- 22.75` |
| sponge | `81^3` | 160000 | 0.3011 | 1 | `31576.40 +/- 152.71` |

The largest-component fraction says the same thing from the other side: the design holds 1.0000 of its cells in one piece in every row, while random holds `0.0411 +/- 0.0098` at the `32 x 32` gasket, 0.0011 at `256 x 256`, and 0.0169 at the `81^3` sponge. (**Verified**, `lab/percolation-race`.) The result holds at base 2 and base 3, in two dimensions and three, and no gap narrows as the grid grows, which rules out a small-size artifact at the sizes reached.

## Boundary

At the same matched cell counts, the connected designs also expose less surface than random, and one design - built to scatter - exposes the most surface possible. (**Verified**, `lab/percolation-race`, except the one Proved line.)

| design | grid | design boundary | random boundary | maximum |
|---|---|---|---|---|
| gasket | `32 x 32` | 2.0082 | `3.0814 +/- 0.0639` | 4 |
| gasket | `256 x 256` | 2.0003 | `3.6006 +/- 0.0103` | 4 |
| diagonal | `32 x 32` | 4.0000 | `3.8808 +/- 0.0897` | 4 |
| diagonal | `128 x 128` | 4.0000 | `3.9677 +/- 0.0232` | 4 |
| seven-of-eight | `8^3` | 1.8542 | `2.4941 +/- 0.0589` | 6 |
| carpet | `81 x 81` | 0.8633 | `1.5330 +/- 0.0124` | 4 |
| sponge | `27^3` | 2.2560 | `3.6535 +/- 0.0141` | 6 |

The diagonal at level `L` is `2^L` isolated cells on a `2^L x 2^L` grid - no two cells of the pattern are ever face-adjacent - so its boundary per cell is exactly `2*D = 4`, the theoretical maximum, at every level. (**Proved**, by construction; the exact 4.0000 is the measured confirmation.) The random set at the same density falls short of the maximum at every size tested.

One number in this table deserves its own flag: the gasket's 2.0082 is a level-5 figure, not a constant. It drifts 2.0082, 2.0027, 2.0009, 2.0003 at sides 32, 64, 128, 256, approaching 2. (**Verified**, `lab/percolation-race`.)

## How decisive, honestly

Means hide ties, so the per-draw counts are part of the result (`lab/percolation-race`).

- Gasket, `32 x 32`: 0 of 400 random draws reach one component, and 0 of 100 at `256 x 256`. Decisive at every size.
- Seven-of-eight, `8^3`: 73 of 400 random draws are *also* a single component, and random's largest fraction is already 0.9937 - at density 0.67 a random set percolates too, so the 3D win at that size is a win in the mean, not per draw. One level up, at `16^3`, it is 0 of 200 and decisive.
- Diagonal, `32 x 32`: 61 of 400 random draws also hit boundary 4.0000, because at density 0.031 a random set is often already an independent set. The design wins the mean by 0.119, about 1.3 standard deviations; at `128 x 128` the ties are 27 of 200. A real edge, and a small one.

And one scope line the summary sentence invites getting wrong: at the dispersing extreme, random has *fewer* components than the design - 30.10 against 32 at `32 x 32`, 125.93 against 128 at `128 x 128` - because the diagonal is `N` isolated cells by construction. "Random wins at neither extreme" is true only of the metric named at each extreme: connectivity at the percolating one, boundary at the dispersing one. (**Verified**, `lab/percolation-race`.)

## Where the honest line falls

What is reached: grids to `256 x 256` in 2D and `81^3` - 160000 occupied cells - in 3D, with the seed count thinning from 400 to 20 at the largest size; one control ensemble, uniform over cell sets of the exact matched size. Nothing here is a limit theorem. The claim this page supports is finite and plain: at matched grid and matched cell count, over the five designs and the sizes tested, self-similar structure holds together where noise shatters, and the one design built to scatter, scatters perfectly. One program, `lab/percolation-race`, runs both passes, prints every figure above in about nine seconds and exits nonzero on any failed check; no log is kept.

## Order in the mixed product

A design's fractal is one tile substituted into itself. Replace the repeated tile by an ordered word of different tiles - `A_w = A_(c_1) (x) ... (x) A_(c_L)`, first factor outermost - and every observable on this page can be asked a new question: does it depend on the order of the word, or only on the multiset of factors? An observable is **order-blind** if only the multiset matters, **order-sensitive** if two orderings of one multiset can differ. Base 2, `D = 2`, the 15 non-empty codes, corner order as in [the core](core.md). No lab study regenerates the counts in this section: its Proved lines stand on their proofs, and every computed tag below is Conjecture until a generator exists.

| observable | length 2, of 105 multisets | length 3, of 210 multisets | status |
|---|---|---|---|
| fill, side, density | 0 sensitive | 0 sensitive | **Proved** order-blind |
| main-diagonal count | 0 sensitive | 0 sensitive | **Proved** order-blind |
| boundary | 0 sensitive | 36 sensitive | **Proved** order-blind at length 2 |
| connected components | 74 sensitive | 188 sensitive | **Proved** order-sensitive |
| Euler characteristic | 78 sensitive | 188 sensitive | **Conjecture** |
| holes | 10 sensitive | 100 sensitive | **Conjecture** |

Denominators are the multisets admitting two or more distinct orderings; a constant word cannot exhibit order sensitivity and is excluded. Length 2 is 225 words over 15 codes, 120 multisets, 105 of them with two or more orderings. Length 3 is 1000 words over a 10-code library, 220 multisets, 210 with two or more.

- Fill, side and density are products of per-factor quantities, so they commute. (**Proved**.)
- The main diagonal factors, `diag(A (x) B) = diag(A) (x) diag(B)`, so its count is a product too. (**Proved**; the sweep of all `15^3` words of length 3 with zero violations has no generator.)
- Scope guard: that is the diagonal *count*, not the diagonal *profile*. The full anti-diagonal profile is order-sensitive on 99 of 105 multisets at length 2, minimal witness the one-cell codes 1 and 2. Take the whole profile, never one coefficient. (**Conjecture** for the count of 99.)
- Boundary at length 2 is order-blind because interior is: on the `4 x 4` grid only the four central cells can be interior, and their requirements pair up under the swap, `(S_1, S_4)` against `(S_4, S_1)` and `(S_2, S_3)` against `(S_3, S_2)`, which is symmetric in the two factors. (**Proved**; the sweep of all 256 code pairs with zero violations has no generator.) It fails from length 3.
- Components are order-sensitive, minimal witness the multiset `{3, 6}`, both factors of fill 2: `comp(A_3 (x) A_6) = 4` and `comp(A_6 (x) A_3) = 2`. The inner tile's contacts decide whether adjacent outer copies merge. (**Proved** by the witness, which is a `4 x 4` array checked by hand.)
- Among the `k = 2` designs, adjacent (codes 3, 5, 10, 12) times adjacent commutes, diagonal (codes 6, 9) times diagonal commutes at 4, and adjacent times diagonal never commutes, always 4 against 2. (**Proved**; all 15 pairs.)
- Every `k >= 3` tile at base 2, `D = 2` is connected and carries both a vertical and a horizontal contact, so any two of them give one component in either order. (**Proved**; all 10 pairs among codes 7, 11, 13, 14, 15.) This is the mixed-word form of the single-component result above.

### Contact is order-blind, and decided factor by factor

Write `h(X)` for the number of rows at which two side-by-side copies of `X` touch, and `v(X)` for the columns at which two stacked copies touch. Then `h(A_w) = prod_i h(A_(c_i))` and `v(A_w) = prod_i v(A_(c_i))`, because the outer columns of a Kronecker product are the Kronecker products of the outer columns and the inner product of Kronecker products is the product of the inner products. (**Proved**; the sweep of all `15^3` words with zero mismatches has no generator.) So whether adjacent copies touch at all is order-blind even where the component count is not - the order-blind part of the boundary story is the contact law, not the interior count.

The race's one Proved line is the `h = v = 0` case: code 9, the diagonal, has no contact in either direction, so no two cells of its fractal are ever face-adjacent at any level and boundary per cell is exactly `2*D`.

### The observables are rational series

Each observable in the table is a rational series in the word: there are a row vector `lambda`, a column vector `gamma` and one square matrix `M_c` per code with `phi(A_w) = lambda M_(c_1) ... M_(c_L) gamma`.

| observable | Hankel rank | distinct matrices | basis words |
|---|---|---|---|
| components | 4 | 6 | `e, 3, 5, 15` |
| Euler characteristic | 4 | 6 | `e, 3, 5, 15` |
| boundary | 8 | 9 | `e, 3, 5, 7, 11, 13, 14, (7,14)` |
| holes | 11 | 10 | `e, 3, 5, 7, 11, 13, 14, 15, (7,6), (7,7), (11,11)` |

**Conjecture**, not Proved: the construction is exhaustive on all 54240 words of length at most 4 over the 15 non-empty codes, plus 120 random words of length 5 to 7, zero mismatches, by Hankel-basis elimination in exact rational arithmetic, but no lab study regenerates it. For components `lambda = (1,0,0,0)` and `gamma = (1,1,1,1)^T`.

- The component matrix depends only on the class of the factor and there are six classes; 14 of the 15 class pairs do not commute, and that noncommutation is the algebraic source of every order-sensitive count in the table above.
- The one commuting pair is the two zero-contact classes, `k = 1` and the diagonal pairs, whose matrices have rank 1 with `M_(6,9) = 2 M_(1,2,4,8)`.
- The vertical and horizontal domino classes carry different matrices, so the representation is not blind to the full square symmetry group, only to the reflection fixing each axis.
- The tension is the whole interest. The number of components still able to merge with a neighbouring copy grows like `2^(L-1)`, attained by the family `(15^(L-1), 3)`, and the component count itself reaches `2 * 3^(L-1)` - yet the Hankel rank stays 4, because rank is the dimension of a span of functions and not the size of any geometric bookkeeping. (**Proved** for both growths, **Conjecture** for the rank.)

> Matched cell for cell, noise breaks into hundreds of pieces and the rule stays whole - and where the rule is built to scatter, it beats noise at scattering too.
