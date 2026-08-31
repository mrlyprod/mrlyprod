# Sequences

Integer sequences fall out of the fractal work: cell counts, coprimality counts, Euler characteristics, design counts. This page is the ledger they are cited from. Every entry below meets the same standard, and nothing is listed that does not.

The registry behind this page is `mrlylab::ledger`, which reads every measure of every design as a sequence and renders this page through `cargo run -p mrlylab --bin ledger`; the b-files behind the submitted entries live in `lab/oeis-terms`. The [sequences demo](../demos/sequences/) searches the same registry live in the browser: type terms, a name, a record or a code and read the design that writes them; the [tour](../demos/tour/) walks a dozen of these sequences with a live picture each, the odd-side law first. The formal census of these sequences, with the fill law, the general exposed-face recurrence and the A056040 identification, is the [sequence-census paper](https://github.com/carlomitchener/carlomitchener/tree/main/research/sequence-census). The fractal work itself stays on [the front page](README.md) and the pages that cite this ledger.

## WHY IT MATTERS

- Three sequences out of this work are with the OEIS - [A395241](https://oeis.org/A395241), [A396934](https://oeis.org/A396934) and [A398348](https://oeis.org/A398348) - so the enumeration half of the fractal work is checkable by strangers, against a catalogue nobody here controls.
- A collision with an existing entry is the useful outcome, not the failure. Two of this page's identifications hand a page something it lacked: [A332705](https://oeis.org/A332705) is the carpet face-count law derived independently in [slices.md](slices.md), and [A299916](https://oeis.org/A299916)'s recurrence gives the slice census a closed form and a dimension the lane could not derive.
- The census entries are the other direction, an existing sequence read as a new object: [A129824](https://oeis.org/A129824) counts fill classes, and the identity that two base-2 designs are the same fractal exactly when they share a popcount profile is what makes it the count of distinct base-2 fractals in dimension `D`, proved below rather than fitted.
- The bar is the point of the lane. Two independent generators sharing no code and no method, every stored b-file term diffed against a generator, and a second cold reading of every entry; nothing is listed that does not meet it, and the entries that fall short say so in their own `status` field.

## THE BAR

- **Two independent generators.** The terms are produced twice, by programs that share no code and no method - a brute-force enumeration and a structural rewrite (digit automaton, Mobius inversion, Burnside over a different group representation). Where a generator merely paraphrases another it does not count, and independence is argued explicitly in the study README.
- **Checked against the b-file.** Every term the stored b-file holds is diffed against a generator, and the two generators are diffed against each other and against the b-file over the whole range both can reach - the far end of the file where the cost allows it, and a long prefix rather than a token sample where a brute-force method runs out of room.
- **Re-checked cold.** A second reading re-runs the generators from the study, re-derives the constants, re-greps the OEIS dump with fresh windows, and checks each claim against the files. Corrections land in the study, and are noted here where they change what a reader should believe.

Novelty is tested by fixed-string search against a local copy of [the OEIS stripped dump](https://oeis.org/stripped.gz) (398821 lines, 398817 sequences) at the full leading window, at one- and two-term shifts, at interior windows, and at simple transforms of the terms. A hit is a collision to be explained, not a failure.

**Every novelty absence recorded below is Conjecture.** A dump is a snapshot and the search behind those absences is no longer in this tree, so a null result is evidence only about the OEIS as the snapshot stood, and every absence needs a live re-read at [oeis.org](https://oeis.org) before it is submitted or repeated. A null search against a dump older than a submission misses that submission; A398348 ([bijection.md](bijection.md)) is on this page as the worked example.

This page is the ledger of the entries that reached the OEIS, and nothing below rests on anything outside this tree. Two results that would otherwise be carried nowhere are recorded here:

- Four term sets are **already in the OEIS and are not to be submitted**, each **Verified** against its own record: the Menger sponge surface faces `6, 72, 1056, 18048 = 2*20^L + 4*8^L` are [A332705](https://oeis.org/A332705); the Sierpinski carpet perimeter `4, 16, 80, 496` is [A381517](https://oeis.org/A381517); the carpet void cells `0, 1, 17, 217` are [A016185](https://oeis.org/A016185), which is `9^n - 8^n` outright, and the same run sits as an interior window of the table [A229896](https://oeis.org/A229896); and the axis-permutation fractal-orbit count `4, 12, 80, 3984` is [A003180](https://oeis.org/A003180), Boolean functions up to the symmetric group, at a one-term shift, that entry beginning `2, 4, 12, 80, 3984`. The first two are carried elsewhere in this tree; the last two are recorded nowhere else.
- One idea is carried nowhere else and is recorded here: the visibility of magic stacks, with exact mixed-scale 2-adic factors. It has no terms and no generator behind it and is an idea, not a result. **Conjecture.** Two neighbouring ideas do survive on their own pages, the Laplacian degeneracy family `3^(L-3) + 1` in [complexity.md](complexity.md) (`lab/laplacian-degeneracy`) and the Eisenstein zeta with `L(2, chi_-3)` in [bases.md](bases.md) (`lab/eisenstein-visibility`). A third does not: [cuts.md](cuts.md) carries `4*(L+5)*3^(L-1)` only as a retraction, since against `mrly_bang_d3_127` it matches the slice maximum, the minimum and the total at no level at all, checked at `L = 1..6`. That formula is **Refuted** and is not submittable (`mrlymath::three::diagonal` test `the_one_two_seven_cut_matches_no_closed_form`).

## PUBLISHED RECORDS

Four OEIS records carry this work, live at the OEIS. The crossrefs inside those records are the OEIS's own and are not re-resolved in [REFS.md](REFS.md).

| record | name | role |
|---|---|---|
| [A103532](https://oeis.org/A103532) | Number of divisors of 240^n. | Not this page's sequence; it carries a signed contribution from this tree reading `a(n)` as the filled-cell count of the generalized Menger sponge with subdivision `2n+1`, and the two Bourke links. |
| [A395241](https://oeis.org/A395241) | a(n) = n^2*(4*n + 3). | The complement of A103532 in the odd cube, ledger entry below. |
| [A396934](https://oeis.org/A396934) | Number of pairs (i,j) with 0 <= i,j < 2^n, i AND j = 0, and gcd(i,j) = 1. | Ledger entry below; its b-file and later terms carry other contributors' extensions. |
| [A398348](https://oeis.org/A398348) | Number of toroidal n X n X n binary arrays, allowing rotation and/or reflection of the layers along each axis as well as all permutations of the axes. | The `design_D3` ledger entry below, with a b-file to `n = 14`. |

## THE LEDGER

### A395241 - void subcubes of the odd sponge tile

The generalized Menger tile at odd subdivision `m = 2n+1`: `a(n)` counts the removed subcubes, equivalently the cells of an `m^3` grid (coordinates from 1) with at least two even coordinates. The record's b-file runs to `n = 10000` and the closed form matches every one of its 10001 terms. **Verified.**

| field | value |
|---|---|
| status | live at the OEIS |
| terms | `0, 7, 44, 135, 304, 575, 972, 1519, 2240, 3159, 4300, 5687` |
| formula | `a(n) = n^2*(4*n + 3)`, offset 0 |
| g.f. | `x*(7 + 16*x + x^2)/(1-x)^4` |
| recurrence | `a(n) = 4*a(n-1) - 6*a(n-2) + 4*a(n-3) - a(n-4)` for `n > 3` |
| crossrefs | `a(n) = A011934(2*n)`; partial sums of A154105; `a(n) + A103532(n) = (2*n+1)^3` |
| record | [A395241](https://oeis.org/A395241) |

The independent generator bores `n^2` square channels along each axis and takes the size of the union of drilled voxels: no closed form, no parity test, and channel crossings resolved by the set rather than by inclusion-exclusion. All six novelty greps returned nothing, as did four further windows tried on the second reading, and both are **Conjecture** under the standing caveat above. The sequence is the even bisection of A011934, with A103532 the odd bisection: a declared crossref, not a duplicate.

### A396934 - coprime points of the Sierpinski triangle

Pairs `(i,j)` with `0 <= i,j < 2^n`, `i AND j = 0`, and `gcd(i,j) = 1`. The condition `i AND j = 0` picks out exactly the `3^n` points of the n-th Sierpinski step (odd `binomial(i+j,i)`, by Kummer); `a(n)` counts the coprime ones. Note `gcd(0,k) = k`, so `(0,1)` and `(1,0)` count.

| field | value |
|---|---|
| status | live at the OEIS, name and terms matching the record |
| terms | `0, 2, 4, 12, 34, 122, 362, 1130, 3406, 10506, 31550, 95260` |
| formula | none, provably: no linear constant-coefficient recurrence exists at any order, **Proved** in [coprime.md](coprime.md) by the no-linear-recurrence corollary; the empirical order 1..9 hunt over the 21 known terms agreed before the proof |
| density | `a(n)/3^n -> 16/(3*Pi^2) = 0.5403796`, **Proved** in [coprime.md](coprime.md), closed above dimension one; `a(20)/3^20 = 0.5403761` (`lab/oeis-terms`) |
| witness | `lab/oeis-terms`, the b-file to `n = 20` |
| record | [A396934](https://oeis.org/A396934) |

The b-file reaches `n = 20`. The independent generator indexes by the row `m = i + j = i OR j` and uses `gcd(i,j) = gcd(i,m)`, enumerating only odd submasks of odd rows and doubling by a fixed-point-free involution; it never forms a complement and never evaluates `gcd(i,j)`. Two near misses were checked and both diverge, **Verified** against their records: A347825 agrees with `a(n)/2` for five terms, and A004662 and A018413 agree with `a(n)-1` for five terms.

### design_D3 - three-dimensional design classes

Number of `n x n x n` binary arrays up to symmetry, the group being independent cyclic rotation and reflection of the layers along each axis together with all permutations of the three axes: `D_n^3` semidirect `S_3`, of order `48*n^3`. Burnside gives `a(n) = (1/|G|) * Sum_{g in G} 2^c(g)`.

| field | value |
|---|---|
| status | live at the OEIS as A398348 |
| terms | `2, 22, 111618, 6005363762644688, 7089215977519836239803174210135872, 10157435539019790383692007859901914095646506996125324171134976` (eight produced here, `n = 1..8`, where `a(7)` has 100 digits and `a(8)` has 150; the record's b-file reaches `n = 14`) |
| formula | none claimed, none found |
| crossrefs | A255016, the two-dimensional toroidal parent (`1, 2, 6, 26, 805, 172112, ...`); A000616, where `A000616(3) = 22 = a(2)` |
| witness | `lab/oeis-terms`, the b-file to `n = 14` |
| record | [A398348](https://oeis.org/A398348) |

Eight terms were produced here, so all eight were compared rather than a prefix, and brute-force orbit enumeration independently confirms `n = 1` and `n = 2`. A third generator, written from the definition for the submission and sharing no code with the other two, reproduces all eight; it is what promoted `a(7)` and `a(8)` out of single-method status and into the b-file. A fourth route, a Burnside-free flood fill over the orbits of all `2^27` colourings, reports `a(3) = 111618` without using Burnside's lemma at all; that route has no generator in `lab/`, so as independent confirmation it is **Conjecture**. The independent generator carries a group element as an affine map `x -> M*x + t` with `M` a signed permutation matrix and gets the cycle count from `c(g) = (1/m) * Sum_{k=1..m} |Fix(g^k)|`, so it never builds a cell-image array and never walks a cycle. The two-dimensional analogue under the identical convention is A255016, which fixes the group convention as the true three-dimensional promotion of that parent; `lab/oeis-terms` is three-dimensional only, so that convention check is **Verified** on [bijection.md](bijection.md) where the `D = 2` Burnside lives and not here. Note that `a(3) = 111618` is the same number the census reports as the full base-3, dimension-3 design space.

This sequence is A398348, whose data is that run verbatim, with a b-file to `n = 14` and a crossref naming A255016 as the two-dimensional case. A null search against a dump older than the submission reports the line absent; that is a report on a stale dump, never evidence of novelty. Tagged in [DISCOVERIES.md](DISCOVERIES.md) and carried in [README.md](README.md) and [bijection.md](bijection.md).

### The odd-side fills

At odd side `n = 2k - 1` an axis splits into `k` low positions and `k - 1` high, so a base-2 design fills `sum over its corners of k^(zeros) (k - 1)^(ones)`, a polynomial in `k` of degree `D`; **Proved** in [DISCOVERIES.md](DISCOVERIES.md), generator `mrlymath::formulas::counting`. The six designs of the plane read as the polygonal numbers, and the corner, the sponge and the solid of the cube as the cubes, the divisor counts of `240^n` and the odd cubes. Every row below is read by `mrlylab::ledger::terms` from `k = 2`, the first odd side past the unit cell, and checked term by term against its record; `shift` is the record's index less the ledger's `k`.

| design | key | closed form | terms from `k = 2` | record | shift | status |
|---|---|---|---|---|---|---|
| corner | `mrly_bang_d2_1.fills.side` | `k^2` | `4, 9, 16, 25, 36, 49, 64, 81` | [A000290](https://oeis.org/A000290) | 0 | **Proved** |
| tree | `mrly_bang_d2_3.fills.side` | `2k^2 - k` | `6, 15, 28, 45, 66, 91, 120, 153` | [A000384](https://oeis.org/A000384) | 0 | **Proved** |
| carpet | `mrly_bang_d2_7.fills.side` | `3k^2 - 2k` | `8, 21, 40, 65, 96, 133, 176, 225` | [A000567](https://oeis.org/A000567) | 0 | **Proved** |
| void | `mrly_bang_d2_9.fills.side` | `2k^2 - 2k + 1` | `5, 13, 25, 41, 61, 85, 113, 145` | [A001844](https://oeis.org/A001844) | -1 | **Proved** |
| corner and centre | `mrly_bang_d2_11.fills.side` | `3k^2 - 3k + 1` | `7, 19, 37, 61, 91, 127, 169, 217` | [A003215](https://oeis.org/A003215) | -1 | **Proved** |
| solid | `mrly_bang_d2_15.fills.side` | `4k^2 - 4k + 1` | `9, 25, 49, 81, 121, 169, 225, 289` | [A016754](https://oeis.org/A016754) | -1 | **Proved** |
| corner | `mrly_bang_d3_1.fills.side` | `k^3` | `8, 27, 64, 125, 216, 343, 512, 729` | [A000578](https://oeis.org/A000578) | 0 | **Proved** |
| sponge | `mrly_bang_d3_23.fills.side` | `4k^3 - 3k^2` | `20, 81, 208, 425, 756, 1225, 1856, 2673` | [A103532](https://oeis.org/A103532) | -1 | **Proved** |
| sponge | `mrly_bang_d3_23.voids.side` | `4k^3 - 9k^2 + 6k - 1` | `7, 44, 135, 304, 575, 972, 1519, 2240` | [A395241](https://oeis.org/A395241) | -1 | **Verified** |
| void | `mrly_bang_d3_129.fills.side` | `2k^3 - 3k^2 + 3k - 1` | `9, 35, 91, 189, 341, 559, 855, 1241` | [A005898](https://oeis.org/A005898) | -1 | **Proved** |
| solid | `mrly_bang_d3_255.fills.side` | `8k^3 - 12k^2 + 6k - 1` | `27, 125, 343, 729, 1331, 2197, 3375, 4913` | [A016755](https://oeis.org/A016755) | -1 | **Proved** |

The side axis is code specific, not orbit invariant: a flip of one axis swaps `k` and `k - 1`, so code 9, the void, reads `2k^2 - 2k + 1` where its orbit mate code 6 reads `2k^2 - 2k`, and code 11 reads `3k^2 - 3k + 1` where code 7, the carpet of the same orbit, reads `3k^2 - 2k`. The catalog lists the least code of every orbit, and the table names the code each record needs.

### The level axis

At side `n = 3` the fill of a level is the tile's fill to the power `L` and the voids are the grid less the fill, while the exposed faces obey `V(L + 1) = occ V(L) - 2 sum P S^L` over the axes, `occ` the tile's filled cells, `P` its adjacent filled pairs along the axis and `S` the cross positions whose two end cells are both filled; `mrlymath::formulas::exposure` closes it in every dimension and `mrlymath::formulas::exposure_recurrence` spells the recurrence. **Proved**, and checked against the rendered census on every code of the cube to level 3. The sponge's slice count is A299916 from its second term, the recurrence being the record's; **Verified** to level 4 by `mrlymath::formulas::cut_fills`.

| design | key | closed form | terms from `L = 1` | record | shift | status |
|---|---|---|---|---|---|---|
| carpet | `mrly_bang_d2_7.fills.level` | `8^L` | `8, 64, 512, 4096, 32768, 262144, 2097152, 16777216` | [A001018](https://oeis.org/A001018) | 0 | **Proved** |
| carpet | `mrly_bang_d2_7.voids.level` | `9^L - 8^L` | `1, 17, 217, 2465, 26281, 269297, 2685817, 26269505` | [A016185](https://oeis.org/A016185) | 0 | **Proved** |
| carpet | `mrly_bang_d2_7.surface.level` | `a(L) = 11 a(L-1) - 24 a(L-2)` | `16, 80, 496, 3536, 26992, 212048, 1684720, 13442768` | [A381517](https://oeis.org/A381517) | 0 | **Proved** |
| void | `mrly_bang_d2_9.fills.level` | `5^L` | `5, 25, 125, 625, 3125, 15625, 78125, 390625` | [A000351](https://oeis.org/A000351) | 0 | **Proved** |
| corner and centre | `mrly_bang_d2_11.fills.level` | `7^L` | `7, 49, 343, 2401, 16807, 117649, 823543, 5764801` | [A000420](https://oeis.org/A000420) | 0 | **Proved** |
| sponge | `mrly_bang_d3_23.fills.level` | `20^L` | `20, 400, 8000, 160000, 3200000, 64000000, 1280000000, 25600000000` | [A009964](https://oeis.org/A009964) | 0 | **Proved** |
| sponge | `mrly_bang_d3_23.surface.level` | `a(L) = 28 a(L-1) - 160 a(L-2)` | `72, 1056, 18048, 336384, 6531072, 129048576, 2568388608, 51267108864` | [A332705](https://oeis.org/A332705) | 0 | **Proved** |
| sponge | `mrly_bang_d3_23.triangles.level` | none | `42, 306, 2250, 16578` to the cell budget | [A299916](https://oeis.org/A299916) | 1 | **Verified** |
| solid | `mrly_bang_d3_255.fills.level` | `27^L` | `27, 729, 19683, 531441, 14348907, 387420489, 10460353203, 282429536481` | [A009971](https://oeis.org/A009971) | 0 | **Proved** |

## CANDIDATES, NOT ENTRIES

The first row below does not meet this page's bar: one method generates it, a second generator agrees only to `D = 6`, it is not checked against the live OEIS, and it has no b-file. It is **Conjecture**, listed so the work is not repeated, and it is not a record. The second row is no longer a candidate at all. Its two cases collapse to `D!/floor(D/2)!^2` by one line of factorials - at odd `D`, `C(D, j)*(j+1) = D!/(j!*j!)` with `j = (D-1)/2` - so the row is the swinging factorial [A056040](https://oeis.org/A056040), present in the dump the novelty search ran against, with far more terms than the row has. It stays listed as the worked example of what a term search misses when only a sequence's bisections look familiar: an existing entry met from a new direction, **Verified** against its record, and nothing to submit.

| candidate | terms | what is claimed | status |
|---|---|---|---|
| `D = 4` octahedral central-diagonal slice census | `6, 132, 1848, 29040, 441408, 6772128` | recurrence `a(n) = 11a(n-1) + 66a(n-2)`, dominant root `(11 + sqrt(385))/2`, slice dimension `2.483635500`; the ladder in [cuts.md](cuts.md), generator in `lab/slice-ladder-controls` | novelty must be checked against the A299916 family first; siblings at `D = 5` and `D = 6` queue behind it. **Conjecture** |
| ambient hypersimplex vertex counts | `2, 6, 6, 30, 20, 140, 70, 630, 252, 2772, 924, 12012, 3432` | `C(D, D/2)` at even `D` and `C(D, (D-1)/2)(D+1)/2` at odd `D`, both equal to `D!/floor(D/2)!^2`; the level-1 slice of the base-3 Menger analog IS that set, and at even `D` it is the hypersimplex vertex count | the row is [A056040](https://oeis.org/A056040) from `D = 2`, a collision explained and not an entry to submit; one generator agrees at `D = 2..14`, and it is `lab/slice-ladder-controls`. **Verified** |

## ESTABLISHED ENTRIES THIS BUILDS ON

Two existing OEIS sequences anchor the census work rather than the sequence work, and research pages should cite them in that role.

| entry | counts | where it lands | witness |
|---|---|---|---|
| A000616 | designs up to symmetry, as the dimension grows at base 2: `3, 6, 22, 402, 1228158, 400507806843728` | the shape-class column of the fill-class census; `A000616(3) = 22` is also `design_D3(2)` | `mrlymath::bang::counting::sequence` |
| A129824 | fill classes, `Prod_{k=0}^{D} (1 + C(D,k))`: `2, 4, 12, 64, 700, 17424, ...` at offset 0 | the count of distinct base-2 fractals in dimension D | `lab/design-census`, the column to `D = 8` |

A129824 is an identity, not a numerical coincidence. **Proved.** Two base-2 designs are the same fractal - equal fill at every side `n` and level `L`, hence equal fractal dimension - exactly when they share a popcount profile, the number of filled corners of each Hamming weight; A129824 counts the possible shapes `(k_0..k_n)` of a collection of subsets of an n-set, which is precisely such a profile. The proof is the linear independence of the `D+1` functions `E^(D-w) * O^w` in `n`. The closed form reproduces all 16 published terms with no shift; two independent fill generators agree on every design at `D = 2` and `D = 3`, and the class count matches A129824 for `D = 1..4` with zero profile collisions (`lab/design-census`). A000616 is recomputed by `mrlymath::bang::counting::sequence`; its offset is `-1`, so `A000616(3) = 22`.

Both censuses live in `lab/design-census`: the fill-class census behind A129824, and the coprimality census read in [coprime.md](coprime.md).

## THE RECORDS

Every OEIS id cited on this tree, read against the live entry on its name, its offset and its first terms. `key` names the design sequence an entry is, in the registry's `design.measure.axis` spelling, and `shift` that record's index less the ledger's. The registry walks 1282 designs across 9 dimension and base pairs and holds 7692 closed rows and 5044 convolved rows of 8 terms each; the grid tiers render on demand within a budget of 500000 cells a term.

| record | name | offset | first terms | key | shift | status |
|---|---|---|---|---|---|---|
| [A000029](https://oeis.org/A000029) | Number of necklaces with n beads of 2 colors, allowing turning over (bracelets) | 0 | `1, 2, 3, 4, 6, 8, 13, 18, 30, 46, 78, 126` |  |  | **Verified** |
| [A000070](https://oeis.org/A000070) | a(n) = Sum_{k=0..n} p(k) where p(k) = number of partitions of k | 0 | `1, 2, 4, 7, 12, 19, 30, 45, 67, 97, 139, 195` |  |  | **Verified** |
| [A000244](https://oeis.org/A000244) | Powers of 3: a(n) = 3^n | 0 | `1, 3, 9, 27, 81, 243, 729, 2187, 6561, 19683, 59049, 177147` |  |  | **Proved** |
| [A000290](https://oeis.org/A000290) | The squares: a(n) = n^2 | 0 | `0, 1, 4, 9, 16, 25, 36, 49, 64, 81, 100, 121` | `mrly_bang_d2_1.fills.side` | 0 | **Proved** |
| [A000351](https://oeis.org/A000351) | Powers of 5: a(n) = 5^n | 0 | `1, 5, 25, 125, 625, 3125, 15625, 78125, 390625, 1953125, 9765625, 48828125` | `mrly_bang_d2_9.fills.level` | 0 | **Proved** |
| [A000370](https://oeis.org/A000370) | Number of NPN-equivalence classes of Boolean functions of n or fewer variables | 0 | `1, 2, 4, 14, 222, 616126, 200253952527184` |  |  | **Verified** |
| [A000384](https://oeis.org/A000384) | Hexagonal numbers: a(n) = n*(2*n-1) | 0 | `0, 1, 6, 15, 28, 45, 66, 91, 120, 153, 190, 231` | `mrly_bang_d2_3.fills.side` | 0 | **Proved** |
| [A000420](https://oeis.org/A000420) | Powers of 7: a(n) = 7^n | 0 | `1, 7, 49, 343, 2401, 16807, 117649, 823543, 5764801, 40353607, 282475249, 1977326743` | `mrly_bang_d2_11.fills.level` | 0 | **Proved** |
| [A000567](https://oeis.org/A000567) | Octagonal numbers: n*(3*n-2) | 0 | `0, 1, 8, 21, 40, 65, 96, 133, 176, 225, 280, 341` | `mrly_bang_d2_7.fills.side` | 0 | **Proved** |
| [A000578](https://oeis.org/A000578) | The cubes: a(n) = n^3 | 0 | `0, 1, 8, 27, 64, 125, 216, 343, 512, 729, 1000, 1331` | `mrly_bang_d3_1.fills.side` | 0 | **Proved** |
| [A000616](https://oeis.org/A000616) | a(-1)=1 by convention; for n >= 0, a(n) = number of irreducible Boolean functions of n variables | -1 | `1, 2, 3, 6, 22, 402, 1228158, 400507806843728` |  |  | **Proved** |
| [A001018](https://oeis.org/A001018) | Powers of 8: a(n) = 8^n | 0 | `1, 8, 64, 512, 4096, 32768, 262144, 2097152, 16777216, 134217728, 1073741824, 8589934592` | `mrly_bang_d2_7.fills.level` | 0 | **Proved** |
| [A001024](https://oeis.org/A001024) | Powers of 15: a(n) = 15^n | 0 | `1, 15, 225, 3375, 50625, 759375, 11390625, 170859375, 2562890625, 38443359375` |  |  | **Verified** |
| [A001316](https://oeis.org/A001316) | Gould's sequence: number of odd entries in row n of Pascal's triangle | 0 | `1, 2, 2, 4, 2, 4, 4, 8, 2, 4, 4, 8` |  |  | **Verified** |
| [A001481](https://oeis.org/A001481) | Numbers that are the sum of 2 squares | 1 | `0, 1, 2, 4, 5, 8, 9, 10, 13, 16, 17, 18` |  |  | **Verified** |
| [A001844](https://oeis.org/A001844) | Centered square numbers: a(n) = 2*n*(n+1)+1 | 0 | `1, 5, 13, 25, 41, 61, 85, 113, 145, 181, 221, 265` | `mrly_bang_d2_9.fills.side` | -1 | **Proved** |
| [A002407](https://oeis.org/A002407) | Cuban primes: primes which are the difference of two consecutive cubes | 1 | `7, 19, 37, 61, 127, 271, 331, 397, 547, 631, 919, 1657` |  |  | **Verified** |
| [A003136](https://oeis.org/A003136) | Loeschian numbers: numbers of the form x^2 + xy + y^2 | 1 | `0, 1, 3, 4, 7, 9, 12, 13, 16, 19, 21, 25` |  |  | **Verified** |
| [A003180](https://oeis.org/A003180) | Number of equivalence classes of Boolean functions of n variables under action of symmetric group | 0 | `2, 4, 12, 80, 3984, 37333248, 25626412338274304` |  |  | **Verified** |
| [A003215](https://oeis.org/A003215) | Hex (or centered hexagonal) numbers: 3*n*(n+1)+1 | 0 | `1, 7, 19, 37, 61, 91, 127, 169, 217, 271, 331, 397` | `mrly_bang_d2_11.fills.side` | -1 | **Proved** |
| [A003463](https://oeis.org/A003463) | a(n) = (5^n - 1)/4 | 0 | `0, 1, 6, 31, 156, 781, 3906, 19531, 97656, 488281, 2441406, 12207031` |  |  | **Verified** |
| [A004016](https://oeis.org/A004016) | Theta series of planar hexagonal lattice A_2 | 0 | `1, 6, 0, 6, 6, 0, 0, 12, 0, 6, 0, 0` |  |  | **Verified** |
| [A004018](https://oeis.org/A004018) | Theta series of square lattice: number of ways of writing n as a sum of 2 squares | 0 | `1, 4, 4, 0, 4, 8, 0, 0, 4, 4, 8, 0` |  |  | **Verified** |
| [A004662](https://oeis.org/A004662) | Powers of 3 written in base 8 | 0 | `1, 3, 11, 33, 121, 363, 1331, 4213, 14641, 46343, 163251, 531773` |  |  | **Verified** |
| [A005418](https://oeis.org/A005418) | Number of (n-1)-bead black-white reversible strings | 1 | `1, 2, 3, 6, 10, 20, 36, 72, 136, 272, 528, 1056` |  |  | **Verified** |
| [A005728](https://oeis.org/A005728) | Number of fractions in Farey series of order n | 0 | `1, 2, 3, 5, 7, 11, 13, 19, 23, 29, 33, 43` |  |  | **Verified** |
| [A005898](https://oeis.org/A005898) | Centered cube numbers: n^3 + (n+1)^3 | 0 | `1, 9, 35, 91, 189, 341, 559, 855, 1241, 1729, 2331, 3059` | `mrly_bang_d3_129.fills.side` | -1 | **Proved** |
| [A009964](https://oeis.org/A009964) | Powers of 20 | 0 | `1, 20, 400, 8000, 160000, 3200000, 64000000, 1280000000, 25600000000, 512000000000` | `mrly_bang_d3_23.fills.level` | 0 | **Proved** |
| [A009971](https://oeis.org/A009971) | Powers of 27 | 0 | `1, 27, 729, 19683, 531441, 14348907, 387420489, 10460353203, 282429536481, 7625597484987` | `mrly_bang_d3_255.fills.level` | 0 | **Proved** |
| [A011934](https://oeis.org/A011934) | a(n) = abs(1^3 - 2^3 + 3^3 - 4^3 + ... + (-1)^(n+1)*n^3) | 0 | `0, 1, 7, 20, 44, 81, 135, 208, 304, 425, 575, 756` |  |  | **Verified** |
| [A016185](https://oeis.org/A016185) | a(n) = 9^n - 8^n | 0 | `0, 1, 17, 217, 2465, 26281, 269297, 2685817, 26269505, 253202761, 2413042577, 22791125017` | `mrly_bang_d2_7.voids.level` | 0 | **Proved** |
| [A016754](https://oeis.org/A016754) | Odd squares: a(n) = (2n+1)^2, also centered octagonal numbers | 0 | `1, 9, 25, 49, 81, 121, 169, 225, 289, 361, 441, 529` | `mrly_bang_d2_15.fills.side` | -1 | **Proved** |
| [A016755](https://oeis.org/A016755) | Odd cubes: a(n) = (2*n + 1)^3 | 0 | `1, 27, 125, 343, 729, 1331, 2197, 3375, 4913, 6859, 9261, 12167` | `mrly_bang_d3_255.fills.side` | -1 | **Proved** |
| [A018413](https://oeis.org/A018413) | Divisors of 363 | 1 | `1, 3, 11, 33, 121, 363` |  |  | **Verified** |
| [A034474](https://oeis.org/A034474) | a(n) = 5^n + 1 | 0 | `2, 6, 26, 126, 626, 3126, 15626, 78126, 390626, 1953126, 9765626, 48828126` |  |  | **Verified** |
| [A047999](https://oeis.org/A047999) | Sierpinski's triangle (or gasket): Pascal's triangle read by rows mod 2 | 0 | `1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0` |  |  | **Verified** |
| [A048883](https://oeis.org/A048883) | a(n) = 3^wt(n), where wt(n) = A000120(n) | 0 | `1, 3, 3, 9, 3, 9, 9, 27, 3, 9, 9, 27` |  |  | **Verified** |
| [A054247](https://oeis.org/A054247) | Number of n X n binary matrices under action of dihedral group of the square D_4 | 0 | `1, 2, 6, 102, 8548, 4211744, 8590557312, 70368882591744, 2305843028004192256` |  |  | **Verified** |
| [A065473](https://oeis.org/A065473) | Decimal expansion of the strongly carefree constant: Product_{p prime} (1 - (3*p-2)/(p^3)) | 0 | `2, 8, 6, 7, 4, 7, 4, 2, 8, 4, 3, 4` |  |  | **Verified** |
| [A069403](https://oeis.org/A069403) | a(n) = 2*Fibonacci(2*n+1) - 1 | 0 | `1, 3, 9, 25, 67, 177, 465, 1219, 3193, 8361, 21891, 57313` |  |  | **Verified** |
| [A103532](https://oeis.org/A103532) | Number of divisors of 240^n | 0 | `1, 20, 81, 208, 425, 756, 1225, 1856, 2673, 3700, 4961, 6480` | `mrly_bang_d3_23.fills.side` | -1 | **Proved** |
| [A125833](https://oeis.org/A125833) | Numbers whose base-5 representation is 333333.......3 | 0 | `0, 3, 18, 93, 468, 2343, 11718, 58593, 292968, 1464843, 7324218, 36621093` |  |  | **Verified** |
| [A128625](https://oeis.org/A128625) | Expansion of (1+3*x)/(1-5*x) | 0 | `1, 8, 40, 200, 1000, 5000, 25000, 125000, 625000, 3125000, 15625000, 78125000` |  |  | **Verified** |
| [A129824](https://oeis.org/A129824) | a(n) = Product_{k=0..n} (1 + binomial(n,k)) | 0 | `2, 4, 12, 64, 700, 17424, 1053696, 160579584, 62856336636, 63812936890000, 168895157342195152, 1169048914836855865344` |  |  | **Proved** |
| [A141148](https://oeis.org/A141148) | Number of aperiodic ternary necklaces with n beads of each color and no adjacent beads of the same color | 1 | `2, 3, 14, 65, 346, 1929, 11442, 70310, 445928, 2896239, 19186738, 129184583` |  |  | **Verified** |
| [A154105](https://oeis.org/A154105) | a(n) = 12*n^2 + 18*n + 7 | 0 | `7, 37, 91, 169, 271, 397, 547, 721, 919, 1141, 1387, 1657` |  |  | **Verified** |
| [A192908](https://oeis.org/A192908) | Constant term in the reduction by (x^2 -> x + 1) of a polynomial family; a(n) = 2*Fibonacci(2n-2) + 1 | 0 | `1, 1, 3, 7, 17, 43, 111, 289, 755, 1975, 5169, 13531` |  |  | **Verified** |
| [A229896](https://oeis.org/A229896) | Sizes of logical groups of the same integer in A229895 | 1 | `1, 1, 4, 1, 5, 27, 1, 7, 37, 256, 1, 9, 61, 369, 3125, 1, 11, 91, 671, 4651, 46656, 1, 13, 127, 1105, 9031, 70993, 823543, 1, 15, 169, 1695, 15961, 144495, 1273609, 16777216, 1, 17, 217, 2465, 26281, 269297, 2685817, 26269505, 387420489` |  |  | **Verified** |
| [A255016](https://oeis.org/A255016) | Number of toroidal n X n binary arrays, allowing rotation and/or reflection of rows and/or columns as well as matrix transposition | 0 | `1, 2, 6, 26, 805, 172112, 239123150, 1436120190288, 36028817512382026` |  |  | **Verified** |
| [A268240](https://oeis.org/A268240) | Pascal's tetrahedron of trinomial coefficients read mod 2 | 0 | `1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1` |  |  | **Verified** |
| [A299916](https://oeis.org/A299916) | a(n) = A299914(2n+1); the six-pointed-star holes of the Menger slice, by a comment | 0 | `1, 6, 42, 306, 2250, 16578, 122202, 900882, 6641514, 48963042, 360969210, 2661166386` | `mrly_bang_d3_23.triangles.level` | 1 | **Verified** |
| [A332705](https://oeis.org/A332705) | Number of unit square faces (or surface area) of a stage-n Menger sponge | 0 | `6, 72, 1056, 18048, 336384, 6531072, 129048576, 2568388608, 51267108864, 1024536870912` | `mrly_bang_d3_23.surface.level` | 0 | **Proved** |
| [A347825](https://oeis.org/A347825) | Number of ways to cut a 2 X n rectangle into rectangles with integer sides up to symmetries of the rectangle | 0 | `1, 2, 6, 17, 61, 220, 883, 3597, 15232, 65130, 282294, 1229729` |  |  | **Verified** |
| [A361870](https://oeis.org/A361870) | Array read by downward antidiagonals: nonequivalent 2-colorings of the cells of an n-dimensional hypercube with edges k cells long | 0 | `2, 2, 1, 2, 2, 1, 2, 3, 2, 1, 2, 6` |  |  | **Verified** |
| [A381517](https://oeis.org/A381517) | Perimeter of the Sierpinski carpet at iteration n | 0 | `4, 16, 80, 496, 3536, 26992, 212048, 1684720, 13442768, 107437168, 859182416, 6872514544` | `mrly_bang_d2_7.surface.level` | 0 | **Proved** |
| [A395134](https://oeis.org/A395134) | Decimal expansion of the probability that the line that passes through two points selected independently and uniformly at random in a half-disk intersects the arc at two points. | 0 | `4, 5, 9, 6, 2, 0, 3, 5, 3, 9, 0, 7` |  |  | **Verified** |
| [A395241](https://oeis.org/A395241) | a(n) = n^2*(4*n + 3) | 0 | `0, 7, 44, 135, 304, 575, 972, 1519, 2240, 3159, 4300, 5687` | `mrly_bang_d3_23.voids.side` | -1 | **Verified** |
| [A396922](https://oeis.org/A396922) | E.g.f. A(x) satisfies A( x / A(log(A(log(A(log(A(x))))))) ) = exp(x) | 0 | `1, 1, 3, 40, 1421, 87896, 7921207, 951512332, 144407735033, 26715045346048` |  |  | **Verified** |
| [A396934](https://oeis.org/A396934) | Number of pairs (i,j) with 0 <= i,j < 2^n, i AND j = 0, and gcd(i,j) = 1 | 0 | `0, 2, 4, 12, 34, 122, 362, 1130, 3406, 10506, 31550, 95260` |  |  | **Verified** |
| [A398348](https://oeis.org/A398348) | Number of toroidal n X n X n binary arrays, allowing rotation and/or reflection of the layers along each axis as well as all permutations of the axes | 1 | `2, 22, 111618, 6005363762644688, 7089215977519836239803174210135872, 10157435539019790383692007859901914095646506996125324171134976` |  |  | **Verified** |

## OPEN QUESTIONS

- **The A396934 density is closed.** `16/(3*Pi^2)` was conjectured while the per-prime mechanism was proved and the interchange-of-limits step was not, and the audit of an outside proof note for A396934 is what kept it a conjecture. The box-bound theorem in [coprime.md](coprime.md) now gives `A(n)/k^n -> delta` for every design with `k > q`, A396934 included (`k = 3 > q = 2`), so the constant is a theorem, **Proved**. Lemma B survives only at `k <= q`, where it stays **Conjecture**.
- **`design_D3` has no formula and none has been found.** Its entry records `none claimed, none found`, and only eight terms were produced here, so all eight were compared rather than a prefix.
- **Novelty here rests on a dump.** A dump is a snapshot, and `design_D3` on this page is the worked example of what a stale one costs; every absence on this page stays **Conjecture** until a live re-read.

## DOCS

- `mrlylab::ledger` - the registry: every measure of every design as a sequence, the curated records with their shifts, and this page, rendered by `cargo run -p mrlylab --bin ledger` and pinned by a test.
- `lab/oeis-terms` - the b-files behind the submitted entries: A396934 to `n = 20`, A398348 to `n = 14`.
- `lab/design-census` - the fill-class and coprimality censuses behind the established entries.
- `lab/slice-ladder-controls` - the generator behind both candidate rows.
- [DISCOVERIES.md](DISCOVERIES.md) - where the sequence findings are adjudicated and tagged; this page carries no findings file of its own.
- [REFS.md](REFS.md) - every sequence id and named reference on these pages, resolved to a canonical URL with a confidence tag.

## THE REST OF THE TREE

- [README](README.md) is the front door: the parity cube, the Kronecker product, and the index of every page below.
- These are the pages that cite this ledger, and the results it is drawn from:

- [core.md](core.md) - what a design is, the headline counts, and the three genera.
- [bijection.md](bijection.md) - designs are Boolean functions up to cube symmetry; the strongest theorem in this tree, and the source of `design_D3`.
- [coprime.md](coprime.md) - the coprimality spine: exact base-local factors on every design, the census behind them, and the theorem that closes the A396934 density.
- [slices.md](slices.md) - the diagonal slice of the solid cube: the `6n` census, centered-hexagonal vertices, and the splitting-prime rule.
- [method.md](method.md) - how the results here are produced and checked, worked through on the odd-side fill polynomial.

## SOURCES

Every sequence id and named reference on these pages is resolved, with a confidence tag, in [REFS.md](REFS.md). The load-bearing external anchors:

- [A000616](https://oeis.org/A000616) - NP-equivalence classes of Boolean functions; the design count in every dimension, and the parent of `design_D3` at `n = 2`.
- [A255016](https://oeis.org/A255016) - toroidal binary arrays; the two-dimensional parent `design_D3` promotes.
- [A129824](https://oeis.org/A129824) - fill classes; the count of distinct base-2 fractals in dimension D.
- [A011934](https://oeis.org/A011934) and [A103532](https://oeis.org/A103532) - the alternating sums of cubes whose two bisections are A103532 and A395241.
- [Kummer's theorem](https://en.wikipedia.org/wiki/Kummer%27s_theorem) - `binomial(i+j,i)` is odd iff `i AND j = 0`, which is what makes A396934 a statement about the Sierpinski triangle.
- [Burnside's lemma](https://en.wikipedia.org/wiki/Burnside%27s_lemma) - the orbit-counting average `design_D3` runs on.
- [The OEIS stripped dump](https://oeis.org/stripped.gz) - the local copy every novelty search on this page was run against.
- [Bourke's fractal page](https://paulbourke.net/fractals/mrlymath/) and the [source PDF](https://paulbourke.net/fractals/mrlymath/mrlymath.pdf) - the published rendering of these families, linked from three of the four records.

## LICENCE

Text [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/); code [MIT](https://opensource.org/license/mit). The sequences themselves belong to the OEIS and its contributors.
