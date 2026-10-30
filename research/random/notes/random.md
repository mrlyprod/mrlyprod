---
title: Random
lead: Randomness from designs: the rounds, all even once they write two cells or more, and a census of which splits reach the alternating group; the design ladder, three no-go rungs for generators built from designs alone; the mess, every Source below the nonlinearity ceiling past five neighbours; and eight chapters: moire limit, schedules, mixing, machine, squares, readings, primes, major arcs.
figure: research-random
slug: random
---

A generator on a grid of `N` bits permutes the `2^N` grids, so two questions precede any statistical test, asked of every design split of the `3 x 3` and `4 x 4` torus: which permutations its rounds can make, and whether one round looks random.

`lab/rs/rounds` prints every number down to the chapter headlines, which come from the `lab/rs` studies.

## The machine

The grid is the torus `Z_n x Z_n`, `n >= 3`: `N = n^2` cells, a state `x` in `V = F_2^N`, `M = 2^N` states. A **split** is a cell set `A`, the fill or written side, with complement `B`, the void or read side.

The **mess** writes `A` from `B`. A neighbourhood is a base-3 level-1 design with its centre popped, in the code of [core](../../core/notes/core.md): `carpet 495` (the eight Moore offsets), `net 186` (the plus) or `void 341` (the diagonals). A cell of `A` counts the live cells of `B` around it and writes `[count in S]`, `S` a Source of [automata](../../automata/notes/automata.md): `primes {2,3,5,7}`, `binary {1,2,4,8}`, `fibonacci {0,1,2,3,5,8}` or `odds {1,3,5,7}`. `odds` makes the mess linear and is the control.

A **round** is `R = t_c L F`:

- the Feistel step `F(x) = x + mess(x)`, which changes only `A` and reads only `B`;
- a spread `L`, a torus map `sigma` moving the bit at `p` to `sigma(p)`: `id`, `shift (i, j+1)`, `drop (i+1, j)`, `slide (i+1, j+1)`, `turn (j, n-1-i)`, `flip (j, i)`;
- a stamp `t_c(x) = x + c` by a design; the census stamps the split's own design, `c = A`.

The undo is the same steps reversed, `R^-1 = F L^-1 t_c`.

The catalog is enumerated, never picked: at `3 x 3` every split `1..510`, the nonempty proper base-3 level-1 codes; at `4 x 4` every two-letter base-2 word `k1 (x) k2`, `k1, k2` in `1..15`, the full word dropped, 224 masks. A split is kept once per orbit of the torus symmetries that commute with the spread: over three neighbourhoods, four Sources and six spreads, 4320 cells at `3 x 3` and 2856 at `4 x 4`.

## The sign law

**Every round is a bijection, its Feistel step an involution, and the undo its inverse. Proved.** The mess depends only on `x_B`, which `F` keeps, so `F(F(x)) = x`; `L` and `t_c` are bijections, `t_c` its own inverse. The census checks all three on every state: 0 faults in 7176 cells.

**Every round that writes two cells or more is an even permutation once `N >= 3`. Proved.**

- A stamp `t_c`, `c != 0`, is `2^(N-1)` transpositions: even once `N >= 2`.
- A spread is a product of coordinate swaps, each `2^(N-2)` transpositions: even once `N >= 3`.
- The Feistel step acts on each fibre `{x : x_B = y}` as the translation by `mess(y)`, so its sign is `(-1)^(w 2^(|A|-1))`, `w` the number of `y` with a nonzero mess: even once `|A| >= 2`, and `(-1)^w` when `|A| = 1`.

So rounds and stamps generate a subgroup of `A_M` unless a round writes one cell with a mess of odd weight: the parity fact of Coppersmith and Grossman 1975 and Even and Goldreich 1983 ([refs](../refs.md)) in design form, proved from scratch. One written cell reading `k` cells of `B` has weight `w = 2^(|B| - k) sum_{s in S} C(k, s)`, odd only when it reads all of `B`.

**In the catalog that happens only at `3 x 3`, with the carpet neighbourhood and `binary`: 9 odd cells, none at `4 x 4`. Proved; Verified.** At `4 x 4` a cell reads at most 8 of the 15 others. The `3 x 3` carpet reads all 8, and `sum_{s in S} C(8, s)` is odd only for `binary`, `107`, against `148`, `150`, `128` for `primes`, `fibonacci`, `odds`.

**The even null matches the uniform null in mean cycles and fixed points. Proved; Verified.** The fair null is a uniform even permutation: `E[cycles | even] = H_M + (-1)^M / (M (M - 1))`, off by `3.8e-6` at `M = 512`; for `M >= 3` the even permutations fixing a point are the `(M - 1)!/2` even permutations of the other `M - 1` points, so the mean fixed-point count is `M ((M - 1)!/2) / (M!/2) = 1`; `null` checks both over every even permutation of `M = 2..8` points.

## The cycle census

A uniform permutation of `M` points has `H_M` cycles on average, `6.8165` at `M = 512` and `11.6676` at `M = 65536`, one fixed point, and a longest cycle covering `0.62494` of the states at 512 (exact), tending to the Golomb-Dickman constant `0.62433` (A084945).

**A single design round is far from a random permutation. Verified.**

| cells | count | cycles, mean | cycles, median | longest/M | fixed points |
|---|---|---|---|---|---|
| `3 x 3` nonlinear | 3240 | 136.43 | 116 | 0.05378 | 28.43 |
| `3 x 3` nonlinear, a symmetry fixes the split | 1476 | 184.80 | 160 | 0.02587 | 53.38 |
| `3 x 3` odds | 1080 | 96.77 | 72 | 0.06240 | 12.77 |
| `4 x 4` nonlinear | 2142 | 16759.65 | 14236 | 0.01298 | 3081.50 |
| `4 x 4` nonlinear, a symmetry fixes the split | 1773 | 18556.72 | 16336 | 0.00816 | 3706.66 |
| `4 x 4` odds | 714 | 14295.61 | 10304 | 0.00685 | 1092.20 |

A torus symmetry fixing the split commutes with the round and roughly doubles the cycle count.

## The group

With every stamp allowed, rounds generate `G = <T, h>`, `T` the `M` translations and `h = L F` the round without its stamp. `T` is transitive, so every `G` is.

**A group containing every translation is imprimitive exactly when some proper nonzero subspace `U` has `h(x + u) + h(x)` in `U` for all `u` in `U`, all `x` and every generator `h`. Read at source.** This is the subspace fact of Caranti, Dalla Volta and Sala 2009, Section 2.1: a block through `0` is a subgroup and the blocks are its cosets. The smallest block through `0` and `v` is the closure of `{v}` under derivatives.

The census computes that closure for every `v != 0`, so a primitive verdict is a proof. A primitive cell is then searched for an element whose only cycle of length divisible by a prime `p <= M - 3` has length `p`; a power of it is a `p`-cycle, and by Jordan's theorem the group is `A_M` or `S_M`.

## The law in the design

Five proved laws force `G != A_M`; for nonlinear cells in both catalogs the census shows they are the only way to miss.

- **I, a read-closed set. Proved.** A nonempty proper union `O` of `sigma`-orbits holding every cell of `A` whose mess reads a cell of `O` makes `F_2^O` a block; the identity spread always has `O = A`.
- **II, the mess module. Proved.** Cells of `A` reading the same live cells of `B` write the same bit, so every derivative of the mess lies in the span `E` of those classes; the smallest `sigma`-invariant subspace containing `E` is a block if proper, and a constant mess makes the round affine.
- **III, the complement. Proved.** If each cell's rule is self-complementary, `[k in S] = [m - k in S]` for a cell reading `m` cells, the all-ones line is a block.
- **IV, the parity of a design partition. Proved.** For a partition of the cells, the grids even on every part form a block exactly when the part parities after a round depend only on those before it. The census tries the whole torus, the cosets of every torus subgroup and, at `4 x 4`, the `2 x 2` tiles.
- **V, a product. Proved.** If the cells split into `k` cosets of a torus subgroup that `sigma` permutes and each cell of `A` reads only inside its coset, `G` lies in `Sym(2^(N/k)) wr S_k`, far below `M!/2`: primitive perhaps, not `A_M`.

**At `3 x 3` the group of every nonlinear cell is read off the laws. Verified.** Of 3240 nonlinear cells, 147 generate `A_512`, 6 are primitive products and 3087 are imprimitive, the first law holding being I for 2901, II for 183 and III for 3. None is imprimitive without a law or primitive and undecided, and none generates `S_512`: the 9 odd cells are imprimitive. The 1080 `odds` cells are affine, so never `A_512`: 1068 are imprimitive and 12 primitive, those 12 meeting none of the five laws, each run on every cell. Only `shift`, `drop` and `slide` give `A_512`, 49 cells each.

**At `4 x 4` the same holds once the parity law is added. Verified.** Of 2142 nonlinear cells, 198 generate `A_65536`, 3 are primitive products and 1941 are imprimitive, the first law holding being I for 1802, II for 113, III for 8 and IV for 18; all 714 affine `odds` cells are imprimitive. `turn`, which fixes no cell, gives `A_65536` in 75 cells.

**A split and its complement written in turn reach the alternating group in a few cells. Verified.** With the identity spread, the 12 splits of `3 x 3` up to symmetry and complement give `A_512` in 12 of 108 nonlinear cells and the 23 words of `4 x 4` give `A_65536` in 10 of 207; every other cell is imprimitive.

The figure runs the level-2 carpet split `0x135f` (carpet, `primes`, `slide`; an alternating cell of 258 cycles) for 64 rounds from one live cell.

## Headline results of the chapters

- **The moire limit.** Proved: Thue-Morse correlations decay in binary runs, `abs C_t(h) <= 1.0093 . 0.7177^(r_2(h))`, so the moire XOR has `C_z(h) -> 0` and a Rajchman spectral measure though neither factor is, at the effective rate `abs C_z(h) <= 1.0093 . 3^(-G(h)/4)` for `h > 25`, `G(h)` of order `log log h/log log log h`, its constant not computed. Verified: the exact `max abs C_z` on every `[2^k, 2^(k+1))`, `k <= 24`. Conjecture: `sum_h C_z(h)^2` is finite.
- **The schedules.** Proved: a trap inside the written side that no mess breaks; a strongly connected second-difference graph with isolated live-read cells gives a primitive group on any torus, and a prime cycle in `(M/2, M-3]` makes a transitive group alternating or symmetric. Verified: `A_(2^25)` for one `5 x 5` pair. Refuted: strongly connected reads, spanning second differences and no affine direction do not force primitivity. Conjecture: they do for pairs and onto masks.
- **The mixing.** Proved: with uniform stamps the input difference is a doubly stochastic chain, irreducible exactly when the group is 2-transitive; a round that writes two cells or more has `sigma_2 = 1`; a local schedule needs `Omega(n)` rounds. Verified: a hyperbolic spread on the board covers the torus 4 to 10 rounds after the first at `L = 2..6`, where `shift` takes 5 to 65. Refuted: `sigma_2 = 1` exactly when imprimitive. Conjecture: `O(L + log 1/eps)` for that spread.
- **The machine.** Verified: at the antipodal load, given the same slide, Rule 30 comes within 1.06 and 1.18 of the slide machine on neighbour reads at sides 16 and 32, the torus-over-ring cone advantage is 2.77 and 4.675, and a three-offset XOR spread beats Rule 30 + slide by 1.70 and 2.40, winning at every load tested; ChaCha8 stays faster per byte. Proved: `L^n = I` over `F_2`, so every such round is a bijection. Refuted: the ring ratios 2.77 and 5.69 as a mixing advantage.
- **Along the squares.** Proved: the Fourier exponent `eta` of all 57 designs, bracketed within `7.80e-5`. Verified: the 30 statistics that light most fit `N^(-2 eta)` within `0.05`. Refuted: a spectral invariant ranking the designs, since the mirror pair `6:5`, `6:12345` shares one measure and reads `X2 = 13152` against `714`.
- **The readings.** Proved: a monomial reading keeps its design's dilation identity, so normal `t(n^2)` returns exactly `1`. Refuted: a reading with no near-head passing the battery, `t` along `n^2 + n` failing at `N = 2^26` with `z = 14.42` against `2.81`. Conjecture: a free Thue-Morse pair with linear remainder of slope divisible by 3 decays as `N^(-0.415)`.
- **Along the primes.** Proved: for Thue-Morse at `2^k` the `q | 6` part of the major-arc sum is `-3^floor(k/2)`. Verified: the sum cut at `q <= 1024` meets the census of the primes below `2^32` within `2.35 sigma` for all 55 distinct designs. Refuted: the `q = 3` term alone as the main term, `31.68 sigma` off at `2^31`.

## Chapters

- [The design ladder](design-ladder.md): Three no-go rungs for generators built from designs alone: one base, two bases, and rounds. A polynomial reading of degree 2 or more leaves the automatic class but not the battery: centred monomials keep exact identities, and only Rudin-Shapiro along `n^2 + n` remains a conjectural fourth rung.
- [The moire limit](two-bases.md): Thue-Morse times the ternary parity: correlations that tend to zero.
- [The mess](mess.md): The nonlinear step as a symmetric Boolean function: past five neighbours every Source sits below the nonlinearity ceiling.
- [The schedules](schedules.md): Which schedules generate a primitive group, for every `n`.
- [The mixing](mixing.md): Rounds to pairwise independence.
- [The machine](machine.md): The counter-mode machine against Rule 30 given the same slide.
- [Along the squares](squares.md): A digit-parity design read along the squares turns random at a rate its spectrum fixes, with a strength it does not.
- [The readings](readings.md): A normal reading of a design can still fail the battery.
- [Along the primes](primes.md): A design read along the primes is biased by its major-arc sum.
- [The major arcs](major-arcs.md): The circle method's major arcs, summed along orbit trees, predict a design's bias along the squares.

## Papers

- [Necessary Exits from the Automatic Class: A Second Base or an Exercised Nonlinearity](../papers/design-generators.md)
- [Design Rounds on the Torus](../papers/design-rounds.md)
- [Normal Is Not Random: Digit Designs Read Along Polynomials](../papers/polynomial-readings.md)

## Open

- **Rounds to random.** [Mixing](mixing.md) measures the rounds to pairwise independence; the rounds until a composite's cycle type matches `H_M`, `0.6243 M` and one fixed point stay open.
- **A complete law list.** Law IV was needed only at `4 x 4`; whether five laws close the nonlinear cells at `5 x 5`, where [schedules](schedules.md) proves 7 of 81 line pairs primitive, is open.
- **The converse as a theorem.** A proof that no law means `A_M` for nonlinear cells in both catalogs would run through O'Nan-Scott, like Caranti, Dalla Volta and Sala 2009b for AES-like ciphers.
