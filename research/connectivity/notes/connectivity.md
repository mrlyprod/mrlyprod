---
title: Structure against noise
lead: Self-similar designs raced against matched random cell sets on components and boundary.
figure: research-connectivity
slug: connectivity
---

A design's fractal occupies some number of cells of a grid. A random set can occupy exactly the same number of cells of the same grid. Race the two on measurable geometry - how many connected pieces, how much boundary per cell - and the question of what the [parity](https://en.wikipedia.org/wiki/Parity_(mathematics)) rule buys gets a number instead of an adjective.

Everything on this page is measurement at finite sizes; the one Proved line of the race is flagged where it occurs. The [graphs demo](/graphs/) draws the cell network of any design live, flat, in the cube and on the diagonal slice, with its tips, junctions, pieces and length beside it.

## The race

Five designs, each built by substituting its level-1 tile into itself and cross-checked against the digit rule that defines it - codes and corner order as in [the core](../../core/notes/core.md):

| design | base | `dim` | rule |
|---|---|---|---|
| gasket | 2 | 2 | code 7, corners `(0,0)`, `(0,1)`, `(1,0)` |
| diagonal | 2 | 2 | code 9, corners `(0,0)`, `(1,1)` |
| seven-of-eight | 2 | 3 | every corner but one |
| carpet | 3 | 2 | 8 of 9 cells - `bang dim 2, code 7` read at base 3 |
| sponge | 3 | 3 | the Menger sponge, 20 of 27 cells |

The control is matched exactly, not approximately: a uniform sample of exactly `N` distinct cells, the occupied count asserted equal to the design's on every draw - not a Bernoulli field with matching probability. Components are face-connected, 4-neighbour in 2D and 6-neighbour in 3D. Boundary per cell is `(2*dim*N - 2*E)/N` with `E` the number of face-adjacent occupied pairs and the grid exterior counting as void, so an isolated cell contributes the full `2*dim`. Every random figure below is a mean and a sample standard deviation: 400 seeds up to 729 cells, 200 at the 2187-cell `128 x 128` gasket, the 2401-cell `16^3` seven-of-eight and the 128-cell `128 x 128` diagonal, 100 at 6561 cells and at the `27^3` sponge, 20 at the `81^3` sponge (`lab/py/percolation-race`).

Two independently written passes race the designs inside one program, `lab/py/percolation-race` - [Kronecker](https://en.wikipedia.org/wiki/Kronecker_product) powers with union-find and a PCG64 control, and substitution over coordinate sets with breadth-first search and a Mersenne Twister control - and agree on every comparison to within a standard deviation; the widest gap printed is 0.2061 standard deviations. The numbers printed here are the second pass.

## Connectivity

Every self-similar design above except the diagonal - built to scatter, and raced on boundary instead - is a single component at every size tested. The matched random set is not close, and the gap widens with size in every family. (**Verified**, `lab/py/percolation-race`.)

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

The largest-component fraction says the same thing from the other side: the design holds 1.0000 of its cells in one piece in every row, while random holds `0.0411 +/- 0.0098` at the `32 x 32` gasket, 0.0011 at `256 x 256`, and 0.0169 at the `81^3` sponge. (**Verified**, `lab/py/percolation-race`.) The result holds at base 2 and base 3, in two dimensions and three, and no gap narrows as the grid grows, which rules out a small-size artifact at the sizes reached.

## Boundary

At the same matched cell counts, the connected designs also expose less surface than random, and one design - built to scatter - exposes the most surface possible. (**Verified**, `lab/py/percolation-race`, except the one Proved line.)

| design | grid | design boundary | random boundary | maximum |
|---|---|---|---|---|
| gasket | `32 x 32` | 2.0082 | `3.0814 +/- 0.0639` | 4 |
| gasket | `256 x 256` | 2.0003 | `3.6006 +/- 0.0103` | 4 |
| diagonal | `32 x 32` | 4.0000 | `3.8808 +/- 0.0897` | 4 |
| diagonal | `128 x 128` | 4.0000 | `3.9677 +/- 0.0232` | 4 |
| seven-of-eight | `8^3` | 1.8542 | `2.4941 +/- 0.0589` | 6 |
| carpet | `81 x 81` | 0.8633 | `1.5330 +/- 0.0124` | 4 |
| sponge | `27^3` | 2.2560 | `3.6535 +/- 0.0141` | 6 |

The diagonal at level `level` is `2^level` isolated cells on a `2^level x 2^level` grid - no two cells of the pattern are ever face-adjacent - so its boundary per cell is exactly `2*dim = 4`, the theoretical maximum, at every level. (**Proved**, by construction; the exact 4.0000 is the measured confirmation.) The random set at the same density falls short of the maximum at every size tested.

One number in this table deserves its own flag: the gasket's 2.0082 is a level-5 figure, not a constant. It drifts 2.0082, 2.0027, 2.0009, 2.0003 at sides 32, 64, 128, 256, approaching 2. (**Verified**, `lab/py/percolation-race`.)

## How decisive, honestly

Means hide ties, so the per-draw counts are part of the result (`lab/py/percolation-race`).

- Gasket, `32 x 32`: 0 of 400 random draws reach one component, and 0 of 100 at `256 x 256`. Decisive at every size.
- Seven-of-eight, `8^3`: 73 of 400 random draws are *also* a single component, and random's largest fraction is already 0.9937 - at density 0.67 a random set percolates too, so the 3D win at that size is a win in the mean, not per draw. One level up, at `16^3`, it is 0 of 200 and decisive.
- Diagonal, `32 x 32`: 61 of 400 random draws also hit boundary 4.0000, because at density 0.031 a random set is often already an independent set. The design wins the mean by 0.119, about 1.3 standard deviations; at `128 x 128` the ties are 27 of 200. A real edge, and a small one.

And one scope line the summary sentence invites getting wrong: at the dispersing extreme, random has *fewer* components than the design - 30.10 against 32 at `32 x 32`, 125.93 against 128 at `128 x 128` - because the diagonal is `N` isolated cells by construction. "Random wins at neither extreme" is true only of the metric named at each extreme: connectivity at the percolating one, boundary at the dispersing one. (**Verified**, `lab/py/percolation-race`.)

## Where the honest line falls

What is reached: grids to `256 x 256` in 2D and `81^3` - 160000 occupied cells - in 3D, with the seed count thinning from 400 to 20 at the largest size; one control ensemble, uniform over cell sets of the exact matched size. Nothing here is a limit theorem. The claim this page supports is finite and plain: at matched grid and matched cell count, over the five designs and the sizes tested, self-similar structure holds together where noise shatters, and the one design built to scatter, scatters perfectly. One program, `lab/py/percolation-race`, runs both passes, prints every figure above in about nine seconds and exits nonzero on any failed check; no log is kept.

## Chapters

- [Order in the mixed product](order-in-the-mixed-product.md): Replacing the repeated tile by a word of different tiles, which observables depend on the order of the word, and the component exponent along a word.
- [Word order, closed and bounded](word-order-closure-and-spectral-radius.md): The remaining word pairs closed by a suffix recursion, what the invariant cone buys, and the joint spectral radius of the cocycle.
