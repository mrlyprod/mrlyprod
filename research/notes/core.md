---
title: The core
lead: What a design is, the headline counts, and the three genera.
figure: research-core
slug: core
---

MrlyMath is two moves. Choose a rule that fills some corners of the parity cube `{0,1}^dim`; substitute that rule into itself by the Kronecker product. The Sierpinski carpet, the Menger sponge, their siblings and their antis are all one choice in move one carried through the same move two. Fix the dimension and the whole universe of rules is finite and already there - 4 of them in 1D, 16 in 2D, 256 in 3D - so the designs are not designed, they are enumerated.

Every claim below carries a tag. **Proved** means a proof is given or restated here; **Verified** means recomputed from scratch by a lab study; **Conjecture** means neither. The generator is `lab/rs/design-census`. The [universe demo](../../site/demos/universe/) draws that census live: every orbit in each dimension and base, with the Burnside counts beside it.

## Move one: a design is a parity rule

Fix a dimension `dim`. A *design* decides, for each cell of a `side^dim` grid, whether the cell is filled (`1`) or void (`0`), using only the *parity* of each coordinate. Equivalently, a design is a choice of which corners of the parity cube `{0,1}^dim` are filled: a subset `F` of `{0,1}^dim`.

That definition closes the universe immediately. The number of designs in dimension `dim` is the number of subsets of a `2^dim`-element set, `2^(2^dim)`: 4 in 1D, 16 in 2D, 256 in 3D, 65536 in 4D. (Proved; Verified by enumeration at dim 1..4.)

## Move two: substitution

The *fractal* of a design at a level is its tile substituted into itself that many times, substitution being the Kronecker product:

```
fractal(g, level) = g (x) g (x) ... (x) g [level copies]
```

Because the construction is a pure Kronecker power, the fill count is multiplicative in the level: `fill(level) = fill^level`. The proof is one line - the sum of the entries of a Kronecker product is the product of the sums - so a single tile fixes a closed form for every level at once, and with it the fractal dimension `log(fill) / log(base)`. (Proved; Verified for all 16 designs at dim 2 and all 256 at dim 3, at level 2 and level 3.)

Three designs make the point concretely at side 3. (Verified by `lab/rs/design-census`: each tile built cell by cell from its parity rule, then Kronecker-substituted, `20^2 = 400` at level 2.)

| design | `dim` | fill at side 3 | fill at level | dimension |
|---|---|---|---|---|
| `bang dim 2, code 7` | 2 | 8 of 9 | `8^level` of `9^level` | `log(8)/log(3) = 1.892789` |
| `bang dim 3, code 23` | 3 | 20 of 27 | `20^level` of `27^level` | `log(20)/log(3) = 2.726833` |
| `bang dim 3, code 3` | 3 | 12 of 27 | `12^level` of `27^level` | `log(12)/log(3) = 2.261860` |

The first row is the Sierpinski carpet, the second the Menger sponge. Neither was put in by hand; both fall out of a parity rule and one product.

## Move two with a different tile each level

Nothing in the Kronecker product requires the same tile twice. For an ordered word of designs `w = (c_1, ..., c_level)` the *mixed product* is `A_w = A_(c_1) (x) ... (x) A_(c_level)`, first factor outermost, and the fill law above survives untouched: `fill(A_w) = prod_i fill(c_i)`, the same one-line sum-of-entries argument. The order of the word is part of the object - `A (x) B` and `B (x) A` share dimension, side, fill and density, and differ as placed patterns - so an order swap is the clean control experiment, holding everything multiplicative fixed while changing only the geometry. Which observables notice is [connectivity](connectivity.md), "Order in the mixed product". A periodic word collapses back to this page's picture by associativity alone, so the mixed product is genuinely new only when the word is aperiodic. (Proved.)

## The name is the rule

The core name of a design is not a word, it is the rule written as a number. Index the `2^dim` corners of `{0,1}^dim` as `c_0, ..., c_(2^dim - 1)` in binary order. A design `F` has code

```
i(F) = sum of 2^k over all k with c_k in F, 0 <= i(F) < 2^(2^dim)
```

and its canonical name is the JSON object `{"kind":"bang","dim":<dim>,"code":<code>}` at base 2, gaining a `"base":<base>` key past base 2, the code in plain decimal with no leading zero; pages write the prose view of that object, `bang dim <dim>, code <code>` and `bang dim <dim>, base <base>, code <code>`, per [NAMES](../../crates/mrlymath/NAMES.md). The historical names carpet, net, tree and void survive only as aliases pointing at particular codes. In 3D, *carpet* and *net* are the single class `bang dim 3, code 23`, *tree* is `bang dim 3, code 3`, *void* is `bang dim 3, code 24`. (Verified by `lab/rs/design-census`: each name's defining corner set is built from its definition and reduced to the smallest code in its symmetry class.)

That carpet and net collapse into one class at base 2 is not a slip. It is the same fact `lab/rs/design-census` records, where the rep code is 7 in dimension 2 and 23 in dimension 3 and only the net label survives.

## The universe is finite, and small up to symmetry

Designs that differ by a symmetry of the cube draw the same shape. The symmetry group is the hyperoctahedral group `B_dim` of signed permutations, of order `2^dim * dim!`, acting on the `2^dim` corners.

| `dim` | designs `2^(2^dim)` | classes up to cube symmetry |
|---|---|---|
| 1 | 4 | 3 |
| 2 | 16 | 6 |
| 3 | 256 | 22 |
| 4 | 65536 | 402 |

(Verified three independent ways by `lab/rs/design-census`: a direct orbit walk over every code, a Burnside average `(1/|B_dim|) * sum over g of 2^c(g)` that never builds an orbit, and the class sums of the fill census, whose base-2 cube group is the same `B_dim`. All three agree at dim 1, 2, 3, 4.)

These counts are OEIS `A000616`, the number of NP-equivalence classes of Boolean functions, at offset `-1`, so that `A000616(dim)` runs `3, 6, 22, 402, 1228158, 400507806843728` over dim 1..6. The match is an identity, not a coincidence - a design up to cube symmetry *is* a Boolean function up to permuting variables and flipping their parities - and the bijection is given in [the bijection page](bijection.md). (Proved there; the terms are Verified here, the Burnside sum reproducing all of `A000616(0..6)` against the live OEIS entry.)

The Burnside form is what makes high dimensions tractable: it needs the cycle counts of the group, never the `2^(2^dim)` designs, so dim 6 is instant while listing its designs is impossible.

## Three genera

A single parity constraint takes exactly three irreducible forms, and genus is a property of the shape, so it is constant on a symmetry class.

- **Isotropic.** Fill depends only on the number of odd coordinates: a level-set `S` inside `{0,...,dim}`, filled where the popcount lies in `S`.
- **Axial.** Fill is "these named axes must be even": a pin of a set of axes.
- **Compound.** Neither - a genuine entanglement of the coordinates.

| `dim` | classes | isotropic | axial only | compound |
|---|---|---|---|---|
| 1 | 3 | 3 | 0 | 0 |
| 2 | 6 | 5 | 1 | 0 |
| 3 | 22 | 10 | 2 | 10 |

(Verified by testing each class orbit-wide rather than on its smallest member - a class counts as isotropic when *some* member is a level-set, which several 3D classes are without their canonical representative being one. The `dim + 1` pins are always present, but the pin of no axes and the pin of all `dim` axes are also level-sets, which is why the axial-only column reads `dim - 1`.)

The algebraic degree over `GF(2)` is a class invariant, and in 3D its histogram across the 22 classes is `deg -1: 1, 0: 1, 1: 3, 2: 9, 3: 8`. (Verified.) Degree does not detect a compound, though: describing compounds as the designs of degree 3 or more is false already in 3D (Refuted), where `bang dim 3, code 60` - fill iff exactly one of two named axes is odd - has degree 1 and is neither a level-set nor a pin. Genus and degree are independent invariants. (Verified.)

## The 2D universe in full

Six classes, sixteen designs. (Verified by `lab/rs/design-census`: codes, degrees, rules and orbit sizes all recomputed; the orbit sizes `1, 4, 4, 2, 4, 1` sum to 16.)

| name | deg | genus | rule | orbit | alias |
|---|---|---|---|---|---|
| `bang dim 2, code 0` | -1 | iso | `S = {}` | 1 | |
| `bang dim 2, code 1` | 2 | iso | `S = {0}` | 4 | dust / point |
| `bang dim 2, code 3` | 1 | axis | `pin(y)` | 4 | tree / line |
| `bang dim 2, code 6` | 1 | iso | `S = {1}` | 2 | star / void |
| `bang dim 2, code 7` | 2 | iso | `S = {0,1}` | 4 | carpet / net |
| `bang dim 2, code 15` | 0 | iso | `S = {0,1,2}` | 1 | |

One caution on reading any such table: the rule shown belongs to the canonical representative, the smallest code in the class. Parity flips are symmetries of the infinite tiling but not of a truncation to `side` cells, so the level-1 fill at a fixed `side` is *not* constant on a class - `bang dim 2, code 6` and its orbit-mate `bang dim 2, code 9` (the diagonal `S = {0,2}`) are the same design and fill differently at side 3. What is invariant is the tail: the degree and leading coefficient of the fill, hence the fractal dimension. The same caveat is recorded against the census in [the sequence ledger](../sequences.md).

## Structure, and what survives into high dimensions

Four structural facts, checked rather than supposed.

- A code of the form `2^k - 1` is canonical in every dimension. Every member of its class has popcount `k`, and `2^k - 1` is the smallest integer with `k` bits set. (Proved; Verified at dim 1..4.)
- Orbit sizes divide the group order `2^dim * dim!`, by orbit-stabilizer, and sum to `2^(2^dim)`. In 3D the sizes are exactly `{1, 2, 4, 6, 8, 12, 24}` and sum to 256. (Proved; Verified at dim 3.)
- The *anti* of a design is its complement, which is bitwise complement of the code, and the canonical set is closed under it. Six 3D classes are self-complementary - codes `15, 23, 27, 30, 60, 105`, among them the carpet `bang dim 3, code 23` - and 42 of the 402 classes at dim 4. (Verified at dim 2, 3, 4.)
- The isotropic classes number `3, 5, 10, 19, 36, 71, 136, 271` at dim 1..8. (Verified at dim 1..8, by enumerating the `2^(dim+1)` level-sets and quotienting by the parity flips, which are the only part of `B_dim` that moves a level-set design.) These eight terms are `(2^(dim+1) + 2^ceil((dim+1)/2))/2` for odd `dim` and one less than that for even `dim`; that the pattern continues is Proved below. The formula without the even-`dim` correction overcounts by exactly one at dim 2, 4, 6, 8, where an extra pair of level-sets is identified by a parity flip. (Refuted.)
- **The proof, in four lines.** A level-set design `F_S` is fixed by every axis permutation, so its whole orbit is `{F_S xor t}` over the `2^dim` parity flips. `F_S xor t` depends on `t` only through `u = |t|`, and is a level set only when every achievable `|c xor t|` at fixed `|c|` sits wholly in or wholly out of `S`. `u = dim` always works and gives the reversal `S -> dim - S`; `u = 1` forces `S` to be a union of parity classes, so it acts only on the pair `{evens, odds}`; every other `u` forces `S` trivial. So the count is subsets of `{0..dim}` up to reversal, minus one when `dim` is even, because reversal preserves parity there and the extra `u = 1` edge merges `evens` with `odds` - while for odd `dim` reversal already swaps them and nothing is merged.
- **Crossref, and the closed form that follows.** The base count is [OEIS A005418](https://oeis.org/A005418) at index `dim + 2`, with `a(n) = 2^(n-2) + 2^(floor(n/2)-1)`, so the whole sequence is `A005418(dim+2) - [dim even]`. Recomputed here to dim 16: `3, 5, 10, 19, 36, 71, 136, 271, 528, 1055, 2080, 4159, 8256, 16511, 32896, 65791`. The formula holds at every term. A direct request to `https://oeis.org/search?q=id:A005418&fmt=json` returns: name *"Number of (n-1)-bead black-white reversible strings; also binary grids; also row sums of Losanitsch's triangle A034851; also number of caterpillar graphs on n+2 vertices"*, formula `a(n) = 2^(n-2) + 2^(floor(n/2) - 1)`, offset 1, and the listed terms `1, 2, 3, 6, 10, 20, 36, 72, 136, 272, 528, 1056, 2080, 4160, 8256, 16512, 32896, 65792` reproduce all sixteen values above under `A005418(dim+2) - [dim even]`. (Verified at source; the sixteen recomputed terms have no lab generator beyond dim 8 and stand as Conjecture there, the closed form being Proved.)

That last line is the whole asymptotic story. The nameable classes - isotropic or axial - number at most `2^(dim+1) + dim + 1`, since there are only `2^(dim+1)` level-sets and `dim + 1` pins to begin with, while the classes in total number at least `2^(2^dim) / (2^dim * dim!)`, no orbit being larger than the group. The first bound is exponential, the second doubly exponential, so the fraction of classes that are isotropic or axial tends to `0`. Almost every design is a compound. (Proved; the ratio of the two bounds is already `6.3 * 10^-5` at dim 5 and `3.4 * 10^-13` at dim 6, Verified.)

This is the precise content of the intuition that most designs look like noise. In the dimensions we can draw, the historical four and their level-set siblings are nearly the entire nameable world. The compounds are the ocean that only opens as `dim` climbs.

## Where the numbers live

The classification census - canonical representatives, orbit sizes, fill polynomials, three notions of algebraic degree, each design validated cell by cell against an independently rendered array - is `lab/rs/design-census`, which writes `sequences.csv` and the per-dimension counts in `counts.csv`. Sequences that fall out of this construction, and the standard every one of them has to meet, are in [the sequence ledger](../sequences.md).
