---
title: Random
lead: Randomness from designs: the rounds, where every round that writes two cells or more is even and a census reads which splits reach the alternating group; the design ladder, three no-go rungs for generators built from designs alone; the mess, the nonlinear step, whose Sources all sit below the nonlinearity ceiling past five neighbours; and four chapters on top: the moire limit, the schedules, the mixing and the machine.
figure: research-random
slug: random
---

A generator on a grid of `N` bits permutes the `2^N` grids, so two questions precede any statistical test: which permutations its rounds can make, and whether one round looks random. This page asks them of every design split of a torus bit grid at `3 x 3` and `4 x 4`. Every round that writes two cells or more is even, so no chain of such rounds and stamps reaches the symmetric group. One round is far from random: on 512 states the nonlinear rounds average 136 cycles where a uniform permutation has 6.8. On the nonlinear cells the group of a round and all stamps is the full alternating group exactly when none of five design laws holds, with no cell of either catalog unexplained; the affine cells never reach it.

`lab/rs/rounds` prints every number down to the chapter headlines, which come from the studies in `lab/rs`.

## The machine

The grid is the torus `Z_n x Z_n`, `n >= 3`: `N = n^2` cells, a state `x` in `V = F_2^N`, `M = 2^N` states. A **split** is a cell set `A`, the fill or written side; its complement `B` is the void or read side.

The **mess** writes `A` from `B`. A neighbourhood is a base-3 level-1 design with its centre popped, in the code of [core](../../core/notes/core.md): `carpet 495` (the eight Moore offsets), `net 186` (the plus) or `void 341` (the diagonals). A cell `a` of `A` counts the live cells of `B` around it and writes `[count in S]`, with `S` a Source of [automata](../../automata/notes/automata.md): `primes {2,3,5,7}`, `binary {1,2,4,8}`, `fibonacci {0,1,2,3,5,8}` or `odds {1,3,5,7}`. `odds` makes the mess linear and is the control.

A **round** is `R = t_c L F`:

- the Feistel step `F(x) = x + mess(x)`, which changes only `A` and reads only `B`;
- a spread `L`, a torus map `sigma` moving the bit at `p` to `sigma(p)`: `id`, `shift (i, j+1)`, `drop (i+1, j)`, `slide (i+1, j+1)`, `turn (j, n-1-i)`, `flip (j, i)`;
- a stamp `t_c(x) = x + c` by a design; the census stamps the split's own design, `c = A`.

The undo is the same steps reversed, `R^-1 = F L^-1 t_c`.

The catalog is enumerated, never picked: at `3 x 3` every split `1..510`, the nonempty proper base-3 level-1 codes; at `4 x 4` every two-letter base-2 word `k1 (x) k2`, `k1, k2` in `1..15`, the full word dropped, 224 masks. A split is kept once per orbit of the torus symmetries that commute with the spread; over three neighbourhoods, four Sources and six spreads that leaves 4320 cells at `3 x 3` and 2856 at `4 x 4`.

## The sign law

**Every round is a bijection, its Feistel step an involution, and the undo its inverse. Proved.** The mess depends only on `x_B`, which `F` keeps, so `F(F(x)) = x`; `L` and `t_c` are bijections and `t_c` is its own inverse. The census checks all three on every state: 0 faults in 7176 cells, the total `all` prints.

**Every round that writes two cells or more is an even permutation once `N >= 3`. Proved.**

- A stamp `t_c`, `c != 0`, is `2^(N-1)` transpositions: even once `N >= 2`.
- A spread is a product of coordinate swaps, each `2^(N-2)` transpositions: even once `N >= 3`.
- The Feistel step acts on each fibre `{x : x_B = y}` as the translation by `mess(y)`, so its sign is `(-1)^(w 2^(|A|-1))`, `w` the number of `y` with a nonzero mess: even once `|A| >= 2`, and `(-1)^w` when `|A| = 1`.

So rounds and stamps generate a subgroup of `A_M` unless some round writes one cell with a mess of odd weight; the design form of the parity fact of Coppersmith and Grossman 1975 and Even and Goldreich 1983 ([refs](../refs.md)), proved from scratch. One written cell reading `k` cells of `B` has weight `w = 2^(|B| - k) sum_{s in S} C(k, s)`, odd only when it reads all of `B`.

**In the catalog that happens only at `3 x 3`, with the carpet neighbourhood and `binary`: 9 odd cells, none at `4 x 4`. Proved; Verified.** At `4 x 4` a cell reads at most 8 of the 15 others. At `3 x 3` the carpet reads all 8, and `sum_{s in S} C(8, s)` is odd only for `binary`, `107`, against `148`, `150`, `128` for `primes`, `fibonacci`, `odds`, printed by `three`.

**The even null matches the uniform null in mean cycles and fixed points. Proved; Verified.** Rounds being even, the fair null is a uniform even permutation: `E[cycles | even] = H_M + (-1)^M / (M (M - 1))`, off by `3.8e-6` at `M = 512`, and for `M >= 3` the even permutations fixing a point are the even permutations of the other `M - 1` points, `(M - 1)!/2` of them, so the mean fixed-point count is `M ((M - 1)!/2) / (M!/2) = 1`; `null` checks both over every even permutation of `M = 2..8` points.

## The cycle census

A uniform permutation of `M` points has `H_M` cycles on average, `6.8165` at `M = 512` and `11.6676` at `M = 65536`; its longest cycle covers `0.62494` of the states at 512 (exact) and tends to the Golomb-Dickman constant `0.62433` (A084945); it has one fixed point.

**A single design round is far from a random permutation. Verified.**

| cells | count | cycles, mean | cycles, median | longest/M | fixed points |
|---|---|---|---|---|---|
| `3 x 3` nonlinear | 3240 | 136.43 | 116 | 0.05378 | 28.43 |
| `3 x 3` nonlinear, a symmetry fixes the split | 1476 | 184.80 | 160 | 0.02587 | 53.38 |
| `3 x 3` odds | 1080 | 96.77 | 72 | 0.06240 | 12.77 |
| `4 x 4` nonlinear | 2142 | 16759.65 | 14236 | 0.01298 | 3081.50 |
| `4 x 4` nonlinear, a symmetry fixes the split | 1773 | 18556.72 | 16336 | 0.00816 | 3706.66 |
| `4 x 4` odds | 714 | 14295.61 | 10304 | 0.00685 | 1092.20 |

A torus symmetry that fixes the split commutes with the round and roughly doubles the cycle count.

## The group

With every stamp allowed, rounds generate `G = <T, h>`, `T` the `M` translations and `h = L F` the round without its stamp. `T` is transitive, so every `G` is; the question is primitivity.

**A group containing every translation is imprimitive exactly when some proper nonzero subspace `U` has `h(x + u) + h(x)` in `U` for all `u` in `U`, all `x` and every generator `h`. Read at source.** This is the subspace fact of Caranti, Dalla Volta and Sala 2009, Section 2.1: a block through `0` is a subgroup and the blocks are its cosets. The smallest block through `0` and `v` is the closure of `{v}` under derivatives.

The census computes that closure for every `v != 0`, so a primitive verdict is a proof. A primitive cell is then searched for an element whose only cycle of length divisible by a prime `p <= M - 3` has length `p`; a power of it is a `p`-cycle, and by Jordan's theorem the group is `A_M` or `S_M`. The 512 elements `t_c h` come first, then 512 random words of length 2 to 4: at `3 x 3`, 135 of the 147 alternating cells are proved by one stamped round and 12 need a word; at `4 x 4`, 192 of 198 and 6.

## The law in the design

Five laws force `G != A_M`. Each is proved; the census shows that for nonlinear cells in both catalogs they are the only way to miss.

- **I, a read-closed set. Proved.** If a nonempty proper union `O` of `sigma`-orbits contains every cell of `A` whose mess depends on a cell of `O`, then `F_2^O` is a block. The identity spread always has `O = A`.
- **II, the mess module. Proved.** Cells of `A` that read the same live cells of `B` write the same bit, so every derivative of the mess lies in the span `E` of those classes; if the smallest `sigma`-invariant subspace containing `E` is proper it is a block, and a constant mess makes the round affine.
- **III, the complement. Proved.** If each cell's rule is self-complementary on its window, `[k in S] = [m - k in S]` for a cell reading `m` cells, the all-ones line is a block.
- **IV, the parity of a design partition. Proved.** For a partition of the cells, the grids even on every part form a block exactly when the part parities after a round depend only on those before it. The census tries the whole torus, the cosets of every torus subgroup and, at `4 x 4`, the `2 x 2` tiles.
- **V, a product. Proved.** If the cells split into `k` cosets of a torus subgroup that `sigma` permutes, and every cell of `A` reads only inside its own coset, then `G` lies in `Sym(2^(N/k)) wr S_k`, far below `M!/2`: primitive perhaps, not `A_M`.

**At `3 x 3` the group of every nonlinear cell is read off the laws. Verified.** Of 3240 nonlinear cells, 147 generate `A_512`, 6 are primitive products and 3087 are imprimitive, the first law holding being I for 2901, II for 183 and III for 3. No nonlinear cell is imprimitive without a law or primitive and undecided, and none generates `S_512`: the 9 odd cells are imprimitive. The 1080 `odds` cells are affine, so never `A_512`: 1068 are imprimitive and 12 primitive, and those 12 meet none of the five laws, each run on every cell; the product law holds in 28 `odds` cells, none of them primitive. The alternating cells are 49 each for `shift`, `drop` and `slide` and none for `id`, `turn` and `flip`.

**At `4 x 4` the same holds once the parity law is added. Verified.** Of 2142 nonlinear cells, 198 generate `A_65536`, 3 are primitive products and 1941 are imprimitive, the first law holding being I for 1802, II for 113, III for 8 and IV for 18; all 714 affine `odds` cells are imprimitive. Each of those 18 keeps a coset or tile parity, 13 of them the parity of the whole torus. `turn`, which fixes no cell at `4 x 4`, reaches the alternating group in 75 cells.

**A split and its complement written in turn reach the alternating group in a few cells. Verified.** With the identity spread and `F_A`, `F_B` alternating, the two-sided Feistel network, the 12 splits of `3 x 3` up to symmetry and complement give `A_512` in 12 of 108 nonlinear cells and the 23 words of `4 x 4` give `A_65536` in 10 of 207; every other cell is imprimitive.

The figure runs the level-2 carpet split `0x135f` (carpet, `primes`, `slide`; an alternating cell of 258 cycles) for 64 rounds from one live cell.

## Headline results of the chapters

- **The moire limit.** Proved: Thue-Morse correlations decay in binary runs, `abs C_t(h) <= 1.0093 . 0.7177^(r_2(h))`, so the moire XOR has `C_z(h) -> 0` and a Rajchman spectral measure though neither factor is, at the effective rate `abs C_z(h) <= 1.0093 . 3^(-G(h)/4)` for `h > 25`, `G(h)` of order `log log h/log log log h`, its constant not computed. Verified: the exact `max abs C_z` on every `[2^k, 2^(k+1))`, `k <= 24`. Conjecture: `sum_h C_z(h)^2` is finite.
- **The schedules.** Proved: a trap inside the written side that no mess breaks; a strongly connected second-difference graph with isolated live-read cells gives a primitive group on any torus, and a prime cycle in `(M/2, M-3]` makes a transitive group alternating or symmetric. Verified: `A_(2^25)` for one `5 x 5` pair. Refuted: connected reads, spanning second differences and no affine direction do not force primitivity. Conjecture: they do for pairs and onto masks.
- **The mixing.** Proved: with uniform stamps the input difference is a doubly stochastic chain, irreducible exactly when the group is 2-transitive; a round that writes two cells or more has `sigma_2 = 1`; a local schedule needs `Omega(n)` rounds. Verified: a hyperbolic spread on the board covers the torus 4 to 10 rounds after the first at `L = 2..6`, where `shift` takes 5 to 65. Refuted: `sigma_2 = 1` exactly when imprimitive. Conjecture: `O(L + log 1/eps)` for that spread.
- **The machine.** Verified: given the same slide, Rule 30 comes within 1.06 and 1.18 of the slide machine on neighbour reads at sides 16 and 32, while a three-offset XOR spread beats Rule 30 + slide by 1.70 and 2.40 and wins at every load tested; ChaCha8 stays faster per byte. Proved: `L^n = I` over `F_2`, so every such round is a bijection. Refuted: the ring ratios 2.77 and 5.69 as mixing; a torus-over-ring cone advantage.

## Chapters

- [The design ladder](design-ladder.md): Three no-go rungs for generators built from designs alone: one base, two bases, and rounds.
- [The moire limit](two-bases.md): Thue-Morse times the ternary parity: correlations that tend to zero.
- [The mess](mess.md): The nonlinear step as a symmetric Boolean function: past five neighbours every Source sits below the nonlinearity ceiling.
- [The schedules](schedules.md): Which schedules generate a primitive group, for every `n`, and the first `5 x 5` group, `A_(2^25)`.
- [The mixing](mixing.md): Rounds to pairwise independence: a doubly stochastic difference chain, a linear light cone, a hyperbolic spread that breaks it.
- [The machine](machine.md): The counter-mode machine against Rule 30 given the same slide: the slide machine barely wins, a three-offset XOR spread wins at every load, and ChaCha8 is still faster.

## Papers

- [Necessary Exits from the Automatic Class: A Second Base or an Exercised Nonlinearity](../papers/design-generators.md): a second base escapes every one-base test.
- [Design Rounds on the Torus](../papers/design-rounds.md): the sign law and the five design laws.

## Open

- **Rounds to random.** [Mixing](mixing.md) measures the rounds to pairwise independence; the rounds until a composite's cycle type matches `H_M`, `0.6243 M` and one fixed point stay open.
- **A complete law list.** Law IV was needed only at `4 x 4`; whether five laws close the nonlinear cells at `5 x 5`, where [schedules](schedules.md) proves primitivity for 7 of 81 line pairs, is open.
- **The converse as a theorem.** No law means `A_M` for nonlinear cells in both catalogs; a proof would run through O'Nan-Scott, like Caranti, Dalla Volta and Sala 2009b for AES-like ciphers.
