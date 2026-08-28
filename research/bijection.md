# A design is a Boolean function

The design count in dimension `D` matches OEIS A000616, the number of NP-equivalence classes of Boolean functions on `D` variables. That used to be an observation about shared terms. It is a theorem: there is an explicit bijection between designs and Boolean functions that carries cube symmetry onto NP-equivalence, so the two classifications are the same classification and the counts agree in every dimension. The term-matching survives below as a backstop, not as the argument.

One naming guard: NP on this page is Negation-Permutation - negate inputs, permute inputs, the Boolean-function equivalence A000616 classifies - and has nothing to do with the complexity class. No claim about P versus NP is made or implied anywhere on this page; [complexity.md](complexity.md) carries the same disclaimer for the measures it computes.

Every claim on this page is tagged **Proved** (re-derived here), **Verified** (recomputed by a lab study, not proved), or **Conjecture** (a number with no generator). The generators are `lab/design-census` and `lab/oeis-terms`.

## The two objects

Fix a dimension `D`. The parity cube is `{0,1}^D`; its `2^D` points are corners, indexed `c_0 .. c_(2^D - 1)` in binary order. A design is a subset `F` of the corners - the filled ones - with code `i(F) = sum of 2^k over k with c_k in F`, so there are exactly `2^(2^D)` designs. A Boolean function on `D` variables is a map `f : {0,1}^D -> {0,1}`, and there are `2^(2^D)` of those too.

**Proved.** The indicator map `Phi(F) = 1_F` is a bijection from designs to Boolean functions. A subset of a finite set and its `{0,1}`-indicator are the same datum: `Phi^-1(f) = f^-1(1)` inverts it. Read in corner order, the fill vector of a design is the truth table of its function, and the code `i(F)` is the integer whose binary digits are that truth table. The design and the function are one object under two names.

## Cube symmetry is NP-equivalence

The cube symmetry group `B_D` is the group of signed permutations: permute the `D` axes by some `pi`, then flip parity on any subset of axes, i.e. XOR a fixed vector `t`. On corners,

```
g . c = (c_pi(0) xor t_0, ..., c_pi(D-1) xor t_(D-1))
```

which over `GF(2)` is the affine map `c -> P*c xor t` with `P` a permutation matrix. The NP group acts on Boolean functions by the two moves that preserve a function's shape: negate inputs (the N), permute inputs (the P). Two functions are NP-equivalent when `f'(x) = f(P*x xor t)` for some permutation matrix `P` and some `t`. You may relabel the input variables and complement any of them; you may not touch the output.

**Proved.** These are the same group, of order `2^D * D!`, and `Phi` is equivariant for it. Same group: both consist of exactly the maps `x -> P*x xor t`, with `D!` free choices of `P` and `2^D` free choices of `t`. Same action: for a design `F` with indicator `f`, `1_(g.F)(x) = 1` iff `x` is in `g.F` iff `g^-1 . x` is in `F`, so `1_(g.F) = f o g^-1`, which is the NP substitution action on the function side. Hence `Phi(g . F) = g . Phi(F)`, and two designs share a cube-symmetry orbit exactly when their indicators share an NP class.

**Proved.** The group is NP and not NPN. Cube symmetry moves corners, so it flips inputs; it never exchanges filled for void, which is the separate operation the project calls *anti* - bitwise complement of the code. Output complementation is the extra N that upgrades NP to NPN, and it appears nowhere in the argument above, so the count lands on NP exactly. **Verified.** The distinction is not cosmetic: adjoining the output flip and recounting orbits by brute force gives `2, 4, 14, 222` at `D = 1..4`, which is `A000370(D)` for `D = 1..4`, against `3, 6, 22, 402` for NP. The offset of A000370 is 0 and the term at index 0 is `1`, so the printed `2, 4, 14, 222` are A000370(1..4). (The NPN recount has no lab generator; Conjecture as a computation, the entry's terms being what they are.)

## The count

**Proved.** The number of designs up to cube symmetry, in every dimension `D`, equals the number of NP-equivalence classes of Boolean functions on `D` variables. The equivariant bijection carries orbits onto classes one-to-one, and equal sets of orbits have equal cardinality. **Verified.** That class count is A000616: the entry's comment reads "number of NP-equivalence classes of switching functions of n or fewer variables", its offset is `-1` with a conventional `a(-1) = 1`, and dimension `D` reads off at index `D`.

Three separately written routines are run against it in `lab/design-census`: an orbit enumeration over designs as corner subsets, an orbit enumeration over truth tables using the textbook NP substitution with no reference to designs, and the Burnside average below. **Verified** for every row.

| `D` | design orbits | NP orbits | Burnside | A000616 |
|---:|---:|---:|---:|---:|
| 1 | 3 | 3 | 3 | 3 |
| 2 | 6 | 6 | 6 | 6 |
| 3 | 22 | 22 | 22 | 22 |
| 4 | 402 | 402 | 402 | 402 |
| 5 | - | - | 1228158 | 1228158 |
| 6 | - | - | 400507806843728 | 400507806843728 |
| 7 | - | - | 527471432057653004017274030725792 | 527471432057653004017274030725792 |

Brute force stops at `D = 4`, where closing orbits over `65536` codes takes a fraction of a second; `D = 5` would mean `2^32` codes, and Burnside costs nothing by comparison. The group orders check as `2^D * D! = 2, 8, 48, 384` for `D = 1..4`. A fourth confirmation: the base-2 rows of the fill census in `lab/design-census` report 6 canonical designs in dimension 2 and 22 in dimension 3, from a group built independently as one dihedral map per axis - at base 2 the residue rotation `r -> r+1` is the parity flip, so that group is this one. A fifth route, canonically labelling each orbit of the 65536 base-2 `D = 4` designs by its minimum bitmask under the 384 signed coordinate permutations, closes 402 orbits whose sizes sum to 65536 exactly.

## Burnside

**Proved.** Since `Phi` is equivariant for a finite group on a finite set, the Cauchy-Frobenius lemma applies on either side and returns the same number:

```
distinct(D) = (1/|B_D|) * sum over g in B_D of 2^c(g), |B_D| = 2^D * D!
```

where `c(g)` is the number of cycles of `g` acting on the `2^D` corners. The fixed-point count is `2^c(g)` because a design is fixed by `g` exactly when it is a union of cycles of the corner permutation `g` induces, and each cycle is independently in or out. On the function side the same statement reads: `f` is fixed iff it is constant on each input orbit. **Verified.** Evaluated over the full group element by element, this reproduces A000616 for `D = 0..7`.

Burnside is not what proves the identity - the bijection does that, and it is uniform in `D`. Burnside is what makes the shared count computable past the point where orbits can be enumerated, and it is the form the project's counting code actually runs. Because the bijection and the Burnside formula both hold in every dimension, statements about how the design count grows are statements about one well-defined object per dimension rather than extrapolations from the three cases small enough to draw.

## Where the honest line falls

The theorem is proved relative to one imported definition: that the symmetry of a design is the signed-permutation group of the cube, order `2^D * D!`, acting on corners as above. Everything downstream of that definition is derived here. A different group would be a different theorem.

The identification with A000616 is **Verified**, not proved, and cannot be otherwise: that a particular OEIS entry counts NP classes is a fact about the entry. It is checked two ways - against the entry's own stated definition, and against its terms through `D = 7` by recomputation. The [sequence ledger](sequences.md) records A000616 in its established-entries table; this page is where it is recomputed.

## Past base 2

Base `q >= 3` has designs too - a filled subset of `{0,...,q-1}^D` - and a natural symmetry group that is *not* `B_D`: one dihedral group per axis on the residues, the rotations `r -> r + b` and reflections `r -> b - r`, composed with permutations of the axes. It is the group the fill-class census builds, met above at base 2 where the rotation is the parity flip. Burnside over it counts the classes, and at `D = 2`, `q = 1..8`, the count runs

```
2, 6, 26, 805, 172112, 239123150, 1436120190288, 36028817512382026
```

That sequence is catalogued: it is OEIS A255016, the number of toroidal `n x n` binary arrays under rotation and reflection of rows and columns plus transposition (Ethier and Lee, *Counting toroidal binary arrays II*, J. Int. Seq. 18, 2015). The identification is structural, not a term-match: residues wrap, so a base-`q` design colours the discrete torus `(Z/q)^D`; a row shift is `r -> r + b`, a row reflection is `r -> b - r`, and the axis swap at `D = 2` is matrix transposition - the three moves in the entry's name. **Verified** at `q = 2`, where `lab/design-census` recovers 22 and 402 in dimensions 3 and 4 and `lab/walk-dimension` closes the 26 classes on 512 codes at `q = 3, D = 2`; **Conjecture** for the rest, which has no lab generator: the Burnside values match the live entry on all eight terms, and brute-force orbit closure with no Burnside in it agrees at `q = 4, D = 2` (805 on 65536).

Two precisions keep the bookkeeping honest.

- **This group is not the rigid hypercube group.** Quotienting the same cells by the rigid group - per-axis maps only the identity and the reversal `r -> q-1-r` - gives `2, 6, 102, 8548, 4211744` at `D = 2`, which is OEIS A054247 and row `n = 2` of A361870. The two counts agree at `q <= 2` and diverge from `q = 3` on. The agreement at base 2 is a collapse, not a coincidence: on two residues `r -> b - r` is the same map as `r -> r + b`, so the group that acts at `q = 2` has order `2^D * D!` - it *is* the `B_D` of this page. The abstract wreath order `(2*q)^D * D!` is the order of the acting group only from `q = 3`. (**Conjecture**, no lab generator: the counts, and that Burnside over the abstract group with its repeats returns the identical count at every `q`.)
- **The `D = 3` line is A398348.** The same Burnside there gives `2, 22, 111618, 6005363762644688, 7089215977519836239803174210135872`, regenerated by `lab/oeis-terms` by cycle walk and by the fixed points of the affine map's powers, with the b-file to `n = 14`; `lab/design-census` reaches the Burnside 111618 at `n = 3` as well. The line is OEIS A398348, entered from this lane with a b-file to `n = 14`, and it cross-references A255016 as the two-dimensional case. (**Verified** against the live entry.)

The counting code for the `D = 3` line is `lab/oeis-terms` and for base 2 `lab/design-census`; the `D = 2` line past `q = 3` and the rigid-group counts have no lab generator yet.
