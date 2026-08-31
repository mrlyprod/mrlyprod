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

A design's fractal is one tile substituted into itself. Replace the repeated tile by an ordered word of different tiles - `A_w = A_(c_1) (x) ... (x) A_(c_L)`, first factor outermost - and every observable on this page can be asked a new question: does it depend on the order of the word, or only on the multiset of factors? An observable is **order-blind** if only the multiset matters, **order-sensitive** if two orderings of one multiset can differ. Base 2, `D = 2`, the 15 non-empty codes, corner order as in [the core](core.md). Every count in this section is printed by `lab/magic-words`, which draws each word twice, once inside the study and once through `mrlymath::bang::magic`, and gets the same cells both ways.

| observable | length 2, of 105 multisets | length 3, of 210 multisets | status |
|---|---|---|---|
| fill, side, density | 0 sensitive | 0 sensitive | **Proved** order-blind |
| main-diagonal count | 0 sensitive | 0 sensitive | **Proved** order-blind |
| boundary | 0 sensitive | 36 sensitive | **Proved** order-blind at length 2 |
| connected components | 74 sensitive | 188 sensitive | **Proved** order-sensitive |
| Euler characteristic | 78 sensitive | 188 sensitive | **Verified** |
| holes | 10 sensitive | 100 sensitive | **Verified** |

The status column tags the law; every count in the table is **Verified**, `lab/magic-words`. Components are 4-connected as everywhere above, holes are the bounded 4-connected components of the complement, and the Euler characteristic is `N - A + Q` over filled cells, face-adjacent filled pairs and full `2 x 2` blocks.

Denominators are the multisets admitting two or more distinct orderings; a constant word cannot exhibit order sensitivity and is excluded. Length 2 is 225 words over 15 codes and `C(16,2) = 120` multisets, 105 of them with two or more orderings once the 15 constant words are set aside. Length 3 is 1000 words over the 10-code library of every code of fill 2 or 3, the 15 less the four one-cell codes and the full tile, and `C(12,3) = 220` multisets, 210 with two or more by the same subtraction; it is the only 10-code subset of the 15, out of 3003, whose length-3 column is the one printed above (**Verified**, `lab/magic-words`).

- Fill, side and density are products of per-factor quantities, so they commute. (**Proved**.)
- The main diagonal factors, `diag(A (x) B) = diag(A) (x) diag(B)`, so its count is a product too. (**Proved**; the sweep of all `15^3` words of length 3 has zero violations, **Verified**, `lab/magic-words`.)
- Scope guard: that is the diagonal *count*, not the diagonal *profile*. The full anti-diagonal profile is order-sensitive on 99 of 105 multisets at length 2, minimal witness the one-cell codes 1 and 2. Take the whole profile, never one coefficient. (**Verified**, `lab/magic-words`, for the count of 99.)
- Boundary at length 2 is order-blind because interior is: on the `4 x 4` grid only the four central cells can be interior, and their requirements pair up under the swap, `(S_1, S_4)` against `(S_4, S_1)` and `(S_2, S_3)` against `(S_3, S_2)`, which is symmetric in the two factors. (**Proved**; the sweep of all 256 ordered code pairs, the 15 non-empty codes with the empty one, has zero violations, **Verified**, `lab/magic-words`.) It fails from length 3.
- Scope guard: boundary in that table counts the filled cells with a void or exterior neighbour, which is what the interior argument controls. The race's exposed faces per cell, `(2*D*N - 2*E)/N`, is a second reading of the same word, and that one is order-sensitive already at length 2, on 78 of 105 multisets and 188 of 210 at length 3. (**Verified**, `lab/magic-words`.)
- Components are order-sensitive, minimal witness the multiset `{3, 6}`, both factors of fill 2: `comp(A_3 (x) A_6) = 4` and `comp(A_6 (x) A_3) = 2`. The inner tile's contacts decide whether adjacent outer copies merge. (**Proved** by the witness, which is a `4 x 4` array checked by hand.)
- Among the `k = 2` designs, adjacent (codes 3, 5, 10, 12) times adjacent commutes, diagonal (codes 6, 9) times diagonal commutes at 4, and adjacent times diagonal never commutes, always 4 against 2. Two observations settle all three. Copies of the inner tile sitting in two cells of the outer tile can merge only when those cells are face-adjacent, and then exactly when the inner tile has a contact in that direction. A diagonal pair has no face-adjacent cells and no contact in either direction, so as the inner letter it leaves every filled cell isolated, `comp(X (x) D) = 2 fill(X)`, and as the outer letter its two copies never merge, `comp(D (x) Y) = 2 comp(Y)`. Adjacent times diagonal is then `2 * 2 = 4` and diagonal times adjacent is `2 * 1 = 2`, and diagonal times diagonal is 4 in both orders. Two dominoes merge exactly when they share an orientation, a condition symmetric in the pair, so they commute, at 1 when the orientations agree and at 2 when they do not. (**Proved**; **Verified**, `lab/magic-words`, on all 15 pairs.)
- Every `k >= 3` tile at base 2, `D = 2` is connected and carries both a vertical and a horizontal contact, so any two of them give one component in either order. (**Proved**; all 10 pairs among codes 7, 11, 13, 14, 15.) This is the mixed-word form of the single-component result above.

### Contact is order-blind, and decided factor by factor

Write `h(X)` for the number of rows at which two side-by-side copies of `X` touch, and `v(X)` for the columns at which two stacked copies touch. Then `h(A_w) = prod_i h(A_(c_i))` and `v(A_w) = prod_i v(A_(c_i))`, because the outer columns of a Kronecker product are the Kronecker products of the outer columns and the inner product of Kronecker products is the product of the inner products. (**Proved**; the sweep of all `15^3` words has zero mismatches, **Verified**, `lab/magic-words`.) So whether adjacent copies touch at all is order-blind even where the component count is not - the order-blind part of the boundary story is the contact law, not the interior count.

The race's one Proved line is the `h = v = 0` case: code 9, the diagonal, has no contact in either direction, so no two cells of its fractal are ever face-adjacent at any level and boundary per cell is exactly `2*D`.

### The observables are rational series

Each observable in the table is a rational series in the word: there are a row vector `lambda`, a column vector `gamma` and one square matrix `M_c` per code with `phi(A_w) = lambda M_(c_1) ... M_(c_L) gamma`.

| observable | Hankel rank | distinct matrices | basis words |
|---|---|---|---|
| components | 4 | 6 | `e, 3, 5, 15` |
| Euler characteristic | 4 | 6 | `e, 3, 5, 15` |
| boundary | 8 | 9 | `e, 3, 5, 7, 11, 13, 14, (7,14)` |
| holes | 11 | 10 | `e, 3, 5, 7, 11, 13, 14, 15, (7,6), (7,7), (11,11)` |

**Verified**, `lab/magic-words`: the construction is exhaustive on all 54240 words of lengths 1 to 4 over the 15 non-empty codes, plus 120 seeded words of length 5 to 7, zero mismatches, by Hankel-basis elimination in exact rational arithmetic, with the basis words, the matrices, `lambda` and `gamma` all outputs of the elimination rather than inputs to it. For components `lambda = (1,0,0,0)` and `gamma = (1,1,1,1)^T`.

- The component matrix depends only on the class of the factor and there are six classes; 14 of the 15 class pairs do not commute, and that noncommutation is the algebraic source of every order-sensitive count in the table above.
- The one commuting pair is the two zero-contact classes, `k = 1` and the diagonal pairs, whose matrices have rank 1 with `M_(6,9) = 2 M_(1,2,4,8)`.
- The vertical and horizontal domino classes carry different matrices, so the representation is not blind to the full square symmetry group, only to the reflection fixing each axis.
- The tension is the whole interest. The number of components still able to merge with a neighbouring copy is `2^(L-1)` on `(15^(L-1), 3)`, whose product is `2^(L-1)` full-width rows and so carries exactly `2^(L-1)` components as well; that is the largest merge count found over all words of length 1 to 4, and the family attains it at every length checked. The component count itself reaches `2 * 4^(L-1)` on `(15^(L-1), 6)`, where the outer full tiles move a cell by even offsets and the innermost letter fills only its two cells of odd `i + j`, so the product is the checkerboard and no two filled cells are face-adjacent - yet the Hankel rank stays 4, because rank is the dimension of a span of functions and not the size of any geometric bookkeeping. (**Proved** for both growths, the row family from the contact law and the checkerboard from the parity of the innermost letter; **Verified**, `lab/magic-words`, at `L = 2..10`, with the merge count maximised exhaustively to length 4.)
- `2 * 4^(L-1)` is not merely reached, it is the ceiling: one cell taken from each component is an independent set, and the `2^L` grid has a perfect matching, so no independent set in it exceeds half the cells and no subset of it has more than `2 * 4^(L-1)` components. The checkerboard family attains the ceiling at every length. (**Proved**; attained exhaustively over all words of length 2 to 4 and through the representation at length 5, **Verified**, `lab/magic-words`.)
- The intermediate rate `2 * 3^(L-1)` belongs to `(7^(L-1), 6)`, the same construction with the gasket in place of the full tile: a zero-contact inner tile keeps adjacent outer copies apart and its own two cells are not adjacent either, so every filled cell is isolated and the count is the fill. (**Proved**; **Verified**, `lab/magic-words`, at `L = 2..10`.)

### The component exponent along a word

Every count above sits at a fixed length. Since the word is a matrix cocycle the next question is a rate: along an infinite word `w`, does `(1/L) log comp(A_(w_1 ... w_L))` converge, and if it does, is the limit a function of the letter frequencies alone? Base 2, `D = 2`, two letters at a time, and `lab/magic-words` prints every number below.

**The constant-word functional. Proved.** `comp(A_(c^L)) = comp(A_c)^L` at every code, by the splits already above: the one-cell codes have fill 1, a domino word of one orientation is a full line, the five codes with `k >= 3` are connected and carry both contacts so every level is one piece, and the diagonal class has no contact in either direction so nothing ever merges and its count is its fill. So `comp(A_c) = 2` exactly on the diagonal class `{6, 9}` and 1 on the other thirteen. The per-letter rate is therefore `log 2` on the diagonal class and 0 elsewhere, and the one linear functional reproducing every constant word is `Phi(f) = (f_6 + f_9) log 2`. It is the component analogue of the scale-dimension frequency average of [magic](magic.md), built the same way out of per-letter data, but a plain average rather than a ratio of two averages, because the observable is a count and not a dimension. It is a value on the frequency simplex, defined at irrational frequencies, and reading it needs no word. (**Verified**, `lab/magic-words`, all 15 codes at `L = 1..7`.)

**Fifty-nine of the 105 letter pairs carry an exact closed form. Proved.** (**Verified**, `lab/magic-words`, per pair on all 32766 words of length at most 14 against the representation and on all 254 words of length at most 7 against the drawn cells, zero mismatches.)

| pair | `comp(A_w)` | pairs |
|---|---|---|
| unit and domino | `2^(k - m)`, `k` the last unit place, `m` the unit count | 16 |
| unit and diagonal | `2^(number of diagonal letters)` | 8 |
| two dominoes of unlike orientation | `2^(L - r)`, `r` the terminal run | 4 |
| domino and diagonal | `2^k`, `k` the place of the last diagonal letter | 8 |
| domino and the full tile | `2^(n - j)`, `n` the number of full letters, `j` their terminal run | 4 |
| gasket class and the full tile | `1` | 4 |
| inside one class | `1`, and `2^L` inside the diagonal class | 15 |

That is 9 of the 15 pairs of distinct classes and all 6 pairs inside one class. The other 46 - the gasket class against the unit, domino and diagonal classes, and the full tile against the unit and diagonal classes - close too, by three suffix lemmas and two formulas, in [the other forty-six](#the-other-forty-six) below; the rest of this section is about the 59 and states nothing about them.

Three mechanisms cover the table, and two of them are geometry rather than algebra; the same-class row and the gasket-against-full row are the contact split and the `k >= 3` line already above.

- **The rank-1 telescope. Proved.** On `{3, 6}` the last two columns of both matrices vanish and `lambda = e_0`, so the cocycle lives on the leading block `A = [[0,1],[-2,3]]`, `B = [[2,0],[4,0]]` with `lambda = (1,0)`, `gamma = (1,1)^T`. Here `A gamma = gamma`, `A p = 2 p` at `p = (1,2)^T`, and `B = p q^T` with `q^T = (2,0)`, `q^T p = 2`, so `B A^n B = 2^(n+1) B`. Writing `w = 3^(a_0) 6 3^(a_1) ... 6 3^(a_m)` and collapsing the interior gaps gives `comp = 2^(L - a_m)`, the last-diagonal-place law.
- **The zero-contact cut. Proved.** Contacts multiply, so a suffix has `h = v = 0` as soon as it contains one letter with `h = 0` and one with `v = 0`; `h = 0` on codes 1, 2, 4, 5, 6, 8, 9, 10 and `v = 0` on codes 1, 2, 3, 4, 6, 8, 9, 12. Copies of such a suffix tile in adjacent cells of the outer tile can never merge, so `comp(A_w) = fill(A_prefix) * comp(A_suffix)` at the last suffix with both contacts zero. (**Verified**, `lab/magic-words`: 49420 of all 54240 words of length 1 to 4 admit the cut, zero mismatches.) It proves four rows of the table outright, cutting at the last unit place for a unit against a domino, at the last letter for a unit against a diagonal, at the start of the terminal run for crossed dominoes, and at the last diagonal place for a domino against a diagonal, each leaving a suffix of one component or two. Its scope is the honest part: the other 4820 words have nonzero contact in one direction at every suffix, and every word over a domino and the full tile is among them.
- **The row-block law. Proved.** On `{3, 15}` the letter 3 forces the row digit to 0 and leaves the column digit free, and 15 forces neither, so the filled set is a product `R x [0, 2^L)` with `R` the rows whose digit vanishes at every 3-place, `|R| = 2^n`. Each row of `R` is a full horizontal line, so two merge exactly when they are vertically adjacent, and `r`, `r + 1` both lie in `R` exactly when `r` sits inside a block of `2^j` consecutive rows, `j` the terminal run of 15s; the next row up flips the forced digit and leaves `R`, so the blocks are apart. Hence `comp = 2^(n - j)`. Transposing gives `{5, 15}`. This is the pair the cut law provably cannot reach.

**Where both letters occur with positive frequency the exponent exists and is a function of the frequency vector alone. Proved**, from the closed forms, one pair family at a time; the hypothesis is not decoration and the next claim is why. If `f_diagonal > 0` then the last diagonal place `k_L` satisfies `k_L / L -> 1`, since the diagonal count is the same at `k_L` and at `L`; if both letters occur then the terminal run `r(L)/L -> 0`, since a terminal run of `delta L` equal letters would pin the other letter's density at `(1 - delta) f`; the unit count over `L` tends to `f_unit` and the terminal run of full letters over `L` tends to 0 by the same argument.

| pair | exponent at interior `f` | `Phi(f)` | fill exponent |
|---|---|---|---|
| unit and domino | `f_domino log 2` | 0 | `f_domino log 2` |
| unit and diagonal | `f_diagonal log 2` | `f_diagonal log 2` | `f_diagonal log 2` |
| two dominoes of unlike orientation | `log 2` | 0 | `log 2` |
| domino and diagonal | `log 2` | `f_diagonal log 2` | `log 2` |
| domino and the full tile | `f_full log 2` | 0 | `(1 + f_full) log 2` |
| gasket class and the full tile | 0 | 0 | `f_gasket log 3 + f_full log 4` |

**The constant word is a degenerate probe: `Phi` is refuted on 7 of the 9 named class pairs and exact on the other 2. Proved**, by the table, at every interior `f`. Two scope lines travel with it and neither is optional. First, the interior formula extends to no vertex on any of the seven: it reads `log 2` there while the constant word at that vertex has count 1 and rate 0, so no continuous frequency functional whatever reproduces the truth on the closed simplex, and the failure is a wrong shape rather than a wrong coefficient. Second, on five of the seven - 28 of the 32 letter pairs - the exponent equals the fill exponent, saturating the trivial ceiling `comp <= fill`, so the value carries nothing the order-blind fill law did not already give, and the whole content there is in the finite-length correction rather than the rate. **Of the 59 named pairs only the domino against the full tile gives an exponent strictly between the per-letter values and the fill ceiling. Proved:** at equal frequencies it is `(log 2)/2` against per-letter values 0, 0 and a fill ceiling of `(3/2) log 2`.

**Order-blindness on the named class. Proved.** Since the exponent depends only on `f` at interior `f`, it is the same along Thue-Morse, along every periodic word with those frequencies, along every strictly ergodic word in which both letters occur - unique ergodicity gives the frequencies and minimality gives them full support - and at almost every Bernoulli word with `0 < p < 1`. So a difference against `Phi` on this class is not evidence of non-stationary behaviour: the tree's own stationary controls, the periodic words, fail the prediction in exactly the same way and by exactly the same amount. What the difference refutes is the frequency functional, not stationarity.

**Along Thue-Morse the reading is exact. Proved**, and existence is earned from the closed form rather than assumed. The word has no three equal letters in a row, since `t_(2n) = t_n` and `t_(2n+1) = 1 - t_n` force a change at every even place, so every terminal run has length at most 2; pairing `(t_(2k), t_(2k+1))`, which always sums to 1, gives exactly `L/2` of each letter at every even length, not merely in the limit. Hence the prefix exponent differs from its limit by at most `2/L` in each family, and at `L = 2^20` the exact prefix rates in `log 2` units are `1/2` or `524287/1048576` in the families whose limit is `(log 2)/2`, `1` or `1048575/1048576` in the families whose limit is `log 2`, and 0 for the gasket class against the full tile, the alternative in each case being the other of the two letter readings. (**Verified**, `lab/magic-words`, at every length to `2^20`: letter counts exactly equal at even length, longest terminal run exactly 2 for either letter, the terminal run of one letter taking the value 0 on 524288 prefixes, 1 on 349526 and 2 on 174762, and the run-boundary word `t_n xor t_(n+1)` equal to the period-doubling word on all 1048575 terms.)

**Without positive frequency for both letters the exponent is not a function of the frequency vector, and need not exist at all. Proved**, over `{3, 6}` at `f = (1, 0)` by three words, checked against the closed form to length 14 (**Verified**, `lab/magic-words`). The constant word `3^L` has rate 0. The word carrying the diagonal letter at the square places has prefix rate 1 at every `L = n^2` and `n/(n + 2)` at `L = (n+1)^2 - 1`, so its rate is `log 2`. The word carrying it at the powers of 2 has prefix rate 1 at every `L = 2^k` and `2^k/(2^(k+1) - 1)` at `L = 2^(k+1) - 1`, printed exactly as `4/7, 8/15, ..., 8192/16383`, so its upper rate is `log 2`, its lower rate is `(log 2)/2` and the limit does not exist. Same frequency vector, three different answers. The scope this does not reach is the one worth naming: those two orbit closures are countable and uniquely ergodic but not minimal, their only invariant measure being the point mass at `3^inf`, while along every minimal word over this pair carrying both letters the limit does exist and equals `log 2`, because minimality bounds the gaps between diagonal letters and so forces the last diagonal place over `L` to 1. A stated rate that drops the interior hypothesis is refuted by at least one of the three.

### The other forty-six

The pairs the table above leaves open are the gasket class against the unit, domino and diagonal classes and the full tile against the unit and diagonal classes. All 46 close, and the mechanism is a suffix recursion rather than anything spectral. Three lemmas do the work, all three geometry, and `lab/magic-words` checks every formula below against both the representation and the drawn cells.

- **A heavy suffix letter does nothing. Proved.** If `A_c` is connected and carries both a horizontal and a vertical contact - exactly codes 7, 11, 13, 14, 15 - then `comp(A_wc) = comp(A_w)` at every `w`. In `A_w (x) A_c` each filled cell becomes a block congruent to `A_c`, connected by hypothesis; two blocks at horizontally adjacent cells meet exactly when some row of `A_c` has both end cells filled, `h >= 1`, two at vertically adjacent cells exactly when `v >= 1`, and two at non-adjacent cells are disjoint and never touch, so the block graph is isomorphic to the cell graph. In the representation this is `M_(7,11,13,14) gamma = M_15 gamma = gamma` (**Verified**, `lab/magic-words`).
- **A zero-contact suffix letter collapses the count to a fill. Proved.** `A_1` is one cell and `A_6` is two cells meeting only at a corner, so `A_w (x) A_c` is a set of isolated cells and `comp(A_wc) = fill(A_w)` at a unit letter, `2 fill(A_w) = fill(A_wc)` at a diagonal letter. It is the zero-contact cut above read at one letter, and in the representation it is `M_(1,2,4,8) gamma = phi` and `M_(6,9) gamma = 2 phi` at `phi = (1,2,2,4)^T`.
- **A domino suffix letter counts runs. Proved.** `comp(A_wp) = H(A_w)`, the number of maximal horizontal runs of filled cells, at a row domino `p`, and the transpose at a column domino. The run counts obey `H(e) = 1`, `H(A_wp) = H(A_w)` and `H(A_wq) = H(A_w) + fill(A_w)` at a gasket letter `q`, read row by row, since each of 7, 11, 13, 14 has exactly one full row and one one-cell row while each of 3, 12 has one full row and one empty row.

**The 46 closed forms. Proved.** (**Verified**, `lab/magic-words`, on all 753572 words of length at most 13 against the representation and on all 11684 words of length at most 7 against the drawn cells, zero mismatches.)

| pair | `comp(A_w)` | pairs |
|---|---|---|
| gasket and unit | `3^(g - j)`, `g` the gasket count, `j` its terminal run | 16 |
| gasket and diagonal | `2^d 3^(g - j)`, `d` the diagonal count | 8 |
| full and unit | `4^(F - j)`, `F` the full count, `j` its terminal run | 4 |
| full and diagonal | `2^d 4^(F - j)` | 2 |
| gasket and domino | `1 + sum of fill(A_(w_1..i-1)) over the gasket places i <= m`, `m` the last domino place | 16 |

The first four rows are one statement in four readings: at a zero-contact letter `z` against a heavy letter, `comp(A_w) = fill(A_(w_1..p))` at `p` the last `z` place, and 1 when `w` carries no `z` at all, since the heavy lemma strips the terminal heavy run and the zero-contact lemma reads what is left. Its hypothesis is `h(z) = v(z) = 0`, which holds exactly on codes 1, 2, 4, 8, 6, 9. The fifth row iterates instead: the heavy lemma strips back to the last domino place `m`, the domino lemma turns the count into `H(A_(w_1..m-1))`, and the run recursion telescopes to the sum. Its hypothesis is that the domino carries exactly one contact direction and the gasket carries both; the eight pairs with a column domino are the transpose of the eight with a row domino, which is the domino lemma's own second half and not an extension of its first.

**At interior frequency the exponent exists on all 46, is order-blind, and equals the fill exponent. Proved.** Let `w` be an infinite word over any one of the 46 pairs and suppose both letter frequencies exist and are strictly positive. Then `chi(w) = lim (1/L) log comp(A_(w_1..w_L))` exists, depends only on the frequency vector, and equals `f_a log k_a + f_b log k_b` at `k_c` the fill of the letter. The interior hypothesis is used exactly once, on terminal runs: if the terminal run `j(L)` of the heavy letter were at least `eps L` along a subsequence, the light letter's count would be frozen on `[L - j, L]`, giving `f_light j = o(L)` and hence `j = o(L)` because `f_light > 0`. That kills the exponent `j` in the first four forms outright. In the fifth the suffix past the last gasket place `i*` has the shape `q p^a q^b`, and freezing the gasket count on `[i*, m]` gives `a = o(L)` while freezing the domino count on `[m, L]` gives `b = o(L)`, so `log fill(A_(w_1..i*-1)) = n_p(L) log 2 + n_q(L) log 3 - o(L)`, which the sandwich below turns into the rate. Nothing whatever is claimed for a word whose letter frequencies fail to exist.

**The sandwich. Proved.** Over a gasket-and-domino pair write `T = fill(A_(w_1..i*-1))` at `i*` the last gasket place at or before the last domino place. Then `T < comp(A_w) <= 1 + (3/2) T`. The lower bound is the `i*` term plus the leading 1; the upper is that consecutive gasket terms grow by a factor of at least 3, so the whole sum is at most `(3/2) T`. (**Verified**, `lab/magic-words`, on six named words - Thue-Morse in both readings, both phases of the period-2 word, a fair coin at a fixed seed and `3^2000 7^2000 3^2000` - with `comp/T` measured in `[1.0004, 2.0000]` and no violation at any length.)

**Every letter pair now carries an exponent at interior frequency. Proved**, by the 46 forms here and the 59 forms above, one pair family at a time, on all 105.

**The Thue-Morse value, exactly. Proved.** Over each of the 16 gasket-and-domino pairs, `(3, 7)` and `(5, 7)` among them, the Thue-Morse word has `chi = (1/2) log 6` under either letter reading, with the certificate `|log comp(A_(w_1..L)) - (L/2) log 6| <= log 108 + (1/2) log(3/2) < 4.885` at every `L >= 4`. The word has no three equal letters in a row, so `a <= 2` and `b <= 2` in the sandwich's suffix and `fill(A_(w_1..L))/T = 3 * 2^a 3^b` lies in `[6, 108]`; it is balanced, `|n_q(L) - L/2| <= 1/2`, so `|log fill(A_(w_1..L)) - (L/2) log 6| <= (1/2) log(3/2)`; adding gives the certificate and dividing by `L` gives the limit. The value is exact and not a fit: the study prints `(1/2) log 6` as `0.895879734614027` nats and `1.292481250360578` in `log 2` units, floats labelled as such. (**Verified**, `lab/magic-words`: all 16 pairs and both readings to `L = 2^14`, largest deviation `4.273459` nats against the certificate constant `4.884864`, and the prefix rate in `log 2` units reading `1.291967463826` at `L = 4096` and `1.292352803727` at `L = 2^14` in one reading, `1.291291597694` and `1.292183837194` in the other, all floats.)

- Scope guard, and it is the one this result is easiest to overstate: the exponents agree, the counts do not. `comp/fill` is `3^(-j)` on a gasket-and-unit word and so is unbounded below, and nothing here says the component count is a bounded fraction of the cell count - only that the rate saturates the trivial ceiling `comp <= fill`. Along Thue-Morse over `(3, 7)` that ratio oscillates and does not converge: over `1 <= L <= 2^14` its minimum is `0.0113766545`, above the proved floor `1/108`, its largest value at `L >= 5` is `43397/186624` in one reading and `151/648` in the other, and the `0.2325367033` it takes at `L = 4096` is one sampled term of the oscillation, never a limit and never a maximum. (**Verified**, `lab/magic-words`, exact rationals where the value is exact and every float labelled.)
- The hypothesis rides inside the sentence and not beside it. The window `comp/fill` in `(1/108, 5/12]`, which follows from `comp <= (5/2) T` and `fill >= 6 T`, is a statement about `L >= 4` and is false at `L = 1`, where the one-letter word 3 reads `1/2`; exactly one length in the whole sweep breaks it and it is that one.

**`Phi` is refuted on 78 of the 105 letter pairs and exact on 27. Proved**, at every interior frequency, against the value of `Phi` and never against a word. The 46 new pairs are refutations to a pair - `f_gasket log 3` against 0 on gasket and unit, and so on down the table - and they join the 32 refutations of the 59. The same count carries the upgrade the 59 alone could not: the exponent saturates the fill ceiling on 89 of the 105 pairs and falls short on 16, so **the domino against the full tile is the unique class pair on the whole alphabet, and not merely among the named 59, whose exponent sits strictly between the constant-word values and the fill ceiling. Proved.** (**Verified**, `lab/magic-words`, all four counts re-derived pair by pair from the closed forms rather than copied.)

**The interior hypothesis is sharp on a pair carrying no diagonal letter. Proved**, over `(3, 7)` at the boundary frequency `f = (1, 0)`, by three named words and the fifth closed form. The constant word `3^L` has rate 0. The word carrying the gasket at the square places has rate `log 2`. The word carrying it at the powers of 2 has upper rate `log 2`, lower rate `(log 2)/2` and no limit - and more than that: `comp` is pinned to `fill(A_(w_1..i*-1))` with `i* = 2^k` for every `L` in `(2^k, 2^(k+1)]`, so the prefix rate is `(i*/L) log 2 + O((log L)/L)` and its accumulation set is the whole interval `[(1/2) log 2, log 2]`, not the two endpoints. The `{3, 6}` witness of the same shape above needed a diagonal letter and this one does not, so the pathology is not a property of the diagonal class. (**Verified**, `lab/magic-words`: the prefix rate at the powers of 2 reads `0.502367981` at `L = 2048`, `1.002164269` at `L = 2049` and `0.500219405` at `L = 32768`, and over the single block `4097 <= L <= 8192` it sweeps from `1.00123` down to `0.50073`, all floats in `log 2` units.)

- A by-product with a stationary control in it: `comp(A_((7,3)^k)) = (6^k + 4)/5`, reading 2, 8, 44, 260, 1556, 9332, whose per-letter rate is `(1/2) log 6` again. Every closed form on the other 89 pairs gives a count of the form `2^a 3^b`, so the gasket against a domino is the only family whose counts are not smooth; the largest at `L = 8` over `(3, 7)` is `1094 = 2 x 547`. (**Verified**, `lab/magic-words`, to `k = 8` against both the closed form and the representation.)
- What stays open, named: words over these pairs whose letter frequencies do not exist. The tripling word `W_(k+1) = W_k 7^|W_k| 3^|W_k|` over `(3, 7)` keeps both letters at density at least `1/4` and still has its prefix rate range over `[0.4792, 1.4379]` in `log 2` units on `1024 <= L <= 4096` with no narrowing, so positive lower density is not enough. The closed form is exact there; the non-existence of that limit is numerics. (**Verified**, `lab/magic-words`, and not proved.) The accumulation set of `comp/fill` along Thue-Morse is unidentified, plausibly the attractor of the two affine maps read along the word (**Conjecture**).

### What the cone buys, and what it does not

The route these rates were expected to come by was ergodic: find a common invariant cone, get a Hilbert-metric contraction along a uniquely ergodic driving word, read the limit off the contraction. Half of that is true, and the half that is true proves nothing here.

**The cone exists and the gasket-and-domino pair is primitive in it. Proved**, and no line of the section above uses it. `phi = (1,2,2,4)^T` is a common right eigenvector, `M_c phi = k_c phi` at every code, so it normalises the row orbit `lambda M_(c_1) ... M_(c_k)` and that orbit only; the side matters, since `phi` is not a left eigenvector of anything. On `{3, 7}` the normalised orbit stays in the plane `n_4 = 0`, where `comp/fill = 1 - b - c` at `(b, c) = (n_2, n_3)` and the two letters act by `N_gasket(b,c) = ((1+b)/3, (1+c)/3)` and `N_domino(b,c) = ((1+b)/2, 0)`. The set `S = {0 <= b <= 1, 0 <= c <= 1/2, b + c <= 1}` is invariant under both, and the length-3 word gasket-domino-gasket maps it strictly inside itself, vertex images `(5/9,1/3)`, `(11/18,1/3)`, `(5/9,1/3)`, `(7/12,1/3)` with `b + c` at most `17/18`; three is minimal, since domino-gasket sends `(1,0)` to `(2/3,1/3)` on the face. (**Verified**, `lab/magic-words`, in exact rationals against the raw matrices along `7,3,7,7,3,3,7`.) The stronger reading is refuted rather than proved: entrywise positivity in the standard basis is the wrong test and fails, neither `M_3` nor `M_7` being non-negative and none of the 8190 products of length at most 12 being entrywise positive.

**And the route is dead anyway, for a reason no cone can fix. Proved**, and the obstruction is named three times over. The observable is a forward orbit `lambda P_L gamma`, not the nested decreasing family of images a projective contraction argument compresses. `gamma` is a fixed vector of both heavy matrices, so the normalised gasket map multiplies `comp/fill` by exactly `1/3` and drives the observation functional into its own invariant face: `gamma` sits on the boundary of the dual cone and the orbit walks to it, which is why `comp/fill` has no uniform positive lower bound over all words. And the decisive one, the number a norm theorem returns is the wrong number. Along `3^inf` the largest entry of `M_3^L` is exactly `2^(L+2) - 2`, so the matrix-norm exponent is `log 2`, while `comp(A_(3^L)) = 1` at every `L` and the component exponent is 0. (**Verified**, `lab/magic-words`, at `L = 1, 2, 4, 8, 16, 32` reading 6, 14, 62, 1022, 262142, 17179869182.)

- The class this rules out, named: no argument bounding `(1/L) log lambda P_L gamma` through projective contraction of forward orbits, through the top Lyapunov or joint-spectral-radius exponent ([Furstenberg and Kesten 1960](REFS.md), [Jungers 2009](REFS.md)), or through unique ergodicity of the driving word alone, can reach `chi`. What the cone does buy is `comp/fill >= (1/18) 3^(-N)` for words in which gasket-domino-gasket recurs with gaps at most `N`, a bounded-gap hypothesis strictly stronger than the one the theorem needs.
- No ergodic engine is used above, and none is cited as one. The hypothesis in force is that both letter frequencies exist and are strictly positive, which is weaker than bounded gaps, weaker than linear recurrence and weaker than unique ergodicity ([Berthe and Delecroix 2014](REFS.md)). `log comp` is neither subadditive nor a norm, so the subadditive convergence theorems for uniquely ergodic driving do not apply to it, and by the witness above they would return a different number if they did.
- A headline that does not exist, said out loud so it is not written later: Thue-Morse is not a named word along which the component exponent fails to exist. It converges, exactly, to `(1/2) log 6`. The genuine non-existence witness is the gasket at the powers of 2 at the boundary frequency, whose orbit closure is countable and not minimal.

Positioned: connectedness of level-varying plane carpets has published necessary and sufficient conditions ([Cristea and Steinsky 2010](REFS.md)), which decide whether the count is 1; the closed forms above are counts and rates rather than a connectedness criterion, and no prior art for them was found in the literature searched, which is a report on a search and not a novelty claim.

> Matched cell for cell, noise breaks into hundreds of pieces and the rule stays whole - and where the rule is built to scatter, it beats noise at scattering too.
