---
title: The toolpath
lead: Self-similar curves on the triangular lattice as one-stroke print paths, and the sharp turn a printer slows for: the edge-covering kind never turns 60 degrees, every point-avoiding replacement curve of order 3 to 13 (order 12 without mirrors) turns 120 degrees at some level, the junction law that makes that a finite check, the exact sharp-turn count of the Gosper curve, and the bead axis, which on the Gosper curve fades as `7^(-k/2)`, slower than Hilbert's, while Peano's holds at `1/2`.
figure: research-toolpaths
slug: toolpaths
---

Write `w = e^(i pi/3)`. A replacement curve on the triangular lattice [`Z[w]`](/wiki/eisenstein-integers/) is a generator, `n` unit steps in directions `g_0, ..., g_(n-1)` (multiples of 60 degrees) from `0` to a chord `c` with `N(c) = n`, and a flag per step saying which copy of the generator replaces that step at the next level: `F` as it is, `R` run backwards, and, when `c/conj(c)` is a unit, `M` mirrored across the chord and `MR` both; these four are every symmetry of a segment, and the mirror stays on the lattice only at the orders `n = r^2` and `3 r^2`, so `3, 4, 9, 12, 16` among those below. Level `k` is `n^k` unit steps from `0` to `c^k`, a curve of [similarity dimension](/wiki/fractal-dimension/) 2. A turn is the change of heading between two steps: 0, 60 or 120 degrees, since 180 retraces a step. A curve is gentle when no level turns 120 degrees; a printer head slows at a sharp turn, so the goal is a gentle self-similar infill.

The generator is the lab study `lab/py/toolpath-turns` (`uv run python research/lab/py/toolpath-turns/toolpath.py <verb>`), verbs `census`, `plain`, `gosper`, `bead` and `rate`. [Arndt](https://arxiv.org/abs/1607.02433) searches only the edge-covering kind, as L-systems of turns, and gives the Gosper curve, which is not of that kind, as an L-system in his section 1.3; the flowsnake read as a radix tile is in the [radix dial](beneath.md).

## Two kinds of curve

Two families share the name plane-filling. An edge-covering curve runs every edge of a region once and never crosses itself; it passes an interior point three times, once per pair of its six edges. A point-avoiding curve visits every point at most once; the Gosper curve is one.

**An edge-covering curve on the triangular lattice never turns 60 degrees at an interior point. Proved.** Around a point its six edges are paired by three passes, and a non-crossing pairing of six points on a circle never pairs two edges with exactly one edge between them, since that edge is then fenced in with no partner; a 60-degree turn is exactly such a pair, a straight pass pairs opposite edges and a 120-degree turn adjacent ones. So every turn of the kind is 0 or 120 degrees, which is the turn alphabet Arndt's triangular-grid search uses (his section 2.3), and a curve of that kind that turns at all turns sharply. The question with content is the point-avoiding kind.

## The junction law

Write a turn type as a triple `(t, f, f')`: the turn `t` between two steps and their flags. At the next level a step with flag `f` becomes a copy whose first and last steps sit at fixed offsets from its heading, so a turn of type `(t, f, f')` becomes one junction turn `t + a(f') - b(f)` with a new flag pair, and each copy adds the generator's own turns with flags carried along.

**The turn types of every level lie in the closure of the level-1 types under that map and the copy rule, a set of at most 96 triples; a curve is gentle at every level if and only if no triple in the closure turns 120 degrees. Proved.** Level `k+1` holds copies of level `k`, so a sharp turn or a revisited point at one level persists at every later level. Gentleness at every level is therefore a finite check (`closure` in the study), and self-avoidance is the only part that needs levels. Where a forward copy meets a reversed one the junction turn equals the parent turn; two forward copies shift it by `g_0 - g_(n-1)`.

## The census

**At orders 3, 4, 7, 9, 12 and 13, the norms from 3 to 13, every point-avoiding replacement curve turns 120 degrees at some level; mirrors are included at 3, 4 and 9, are off the lattice at 7 and 13, and are left out at 12. Verified** (`census`, `plain 12`). Each of these norms has one Eisenstein integer up to units and conjugation, so one chord per order covers every curve up to congruence. The search runs every gentle generator walk to the chord, every flag assignment that is gentle by the closure and point-avoiding at level 2, and expands the rest. A pair is a generator with its flags; a class is a pair with its reversal and, where the chord allows it, its mirror image.

| order | chord | flags | gentle walks | pairs gentle at every level, avoiding at level 2 | classes | first crossing |
|---|---|---|---|---|---|---|
| 3 | `1+w` | `F R M MR` | 0 | 0 | 0 | none |
| 4 | `2` | `F R M MR` | 0 | 0 | 0 | none |
| 7 | `2+w` | `F R` | 20 | 10 | 5 | level 3, all |
| 9 | `3` | `F R M MR` | 94 | 11696 | 2972 | level 3, all |
| 12 | `2+2w` | `F R` | 2646 | 3554 | 889 | level 3, all |
| 13 | `3+w` | `F R` | 6794 | 9680 | 4840 | level 3, all |

At order 12 with mirrors the census stops after walk 379 of 2646, with 477775 pairs, all crossing at level 3; at order 16 with `F, R` it stops after walk 97426 of 122964, with 93057 pairs, all crossing at level 3; order 16 with mirrors is not run. Every failure is a crossing: no pair at these orders is gentle by the closure and still avoiding at level 3. **The same holds at orders 12 with mirrors and 16 with or without them. Conjecture.**

## The Gosper count

The Gosper curve is the generator `0, 5, 3, 4, 0, 0, 1` to the chord `3-w` with flags `F R R F F F R`. **At level `k` it has `7^(k-1)` straight joints, `4*7^(k-1) - 1` turns of 60 degrees and `2*7^(k-1)` of 120 degrees. Verified** at levels 1 to 6 by `gosper`, with the curve point-avoiding through level 5. Its mirror image `0, 1, 3, 2, 0, 0, 5` to the chord `2+w`, with the same flags, is the flowsnake direction sequence [A229214](https://oeis.org/A229214); `gosper` checks its first 49 terms.

## The bead axis

A printed bead has an axis but no arrow, so its orientation lives in the second harmonic `h2 = sum e^(2 i theta)` over the steps; the fourth harmonic is `1` on every square-lattice step and the sixth on every triangular one, so neither compares anything. With `F, R` flags the level-`k` harmonic is `G^k` for the generator's own sum `G`, since reversal keeps directions.

**On the Gosper curve `abs(h2)^2 = 7^k` exactly, since `G = 3 + w^2` has norm 7, so its axis anisotropy `abs(h2)/7^k` is `7^(-k/2)`. Proved**, and Verified at levels 1 to 6 by `gosper`. **Hilbert's curve has `h2 = -1` at levels 1 to 6 and Peano's has `h2 = (9^k - 1)/2` at levels 1 to 4. Verified** (`bead`). Per step that is Hilbert near `1/4^k`, Gosper `7^(-k/2)` and Peano a constant `1/2`: Hilbert balances its axis fastest.

## The printer's line

**If a replacement curve of order `n` first turns 120 degrees at level `L`, then level `k >= L` has at least `n^(k-L)` such turns. Proved**: level `k` holds `n^(k-L)` disjoint copies of level `L`, each moved by one of the four segment symmetries, which keep every turn size. So sharp turns never thin out; the only lever is their density.

**Up to traversal direction, two order-7 replacement curves to the chord `2+w` avoid points through level 4: the Gosper curve's mirror image, with `2*7^(k-1)` turns of 120 degrees per `7^k` unit steps, `2/7` per step, and `0, 2, 2, 0, 0, 0, 4` with flags `R R F F R F F`, with `3*7^(k-1)`. Verified** (`rate 7`, levels 1 to 5). Every other chord of norm 7 is a rotation of `2+w` or of its mirror image `3-w`, so the Gosper curve has the fewest sharp turns at order 7. So at the orders the census exhausts, with `M, MR` searched only at 3, 4 and 9, no one-generator replacement curve is a point-avoiding path with no sharp turn; curves built from several motifs lie outside the census.

## What stays open

- A proof at every order: the census suggests every gentle point-avoiding curve crosses by level 3, which would make the claim a finite statement about two levels.
- Order 16 with `F, R` needs only an uncut `plain 16`; order 12 and 16 with mirrors need a compiled census; order 19 is the next beyond.
- The sharp-turn density minimum over replacement curves beyond order 7.
- Self-similar curves built from several motifs.
