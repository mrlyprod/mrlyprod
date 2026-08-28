# Method

The design space is finite. In dimension `D` there are `2^(2^D)` designs and nothing else, so a claim about designs is a claim about a finite list, and the honest way to settle it is to walk the list. That one fact sets the method used on every page here: enumerate rather than sample, produce every number twice, pin every formula to something literally drawn, publish the code, and label each claim with what was actually established rather than with how sure it feels.

Every claim below carries a tag, on the convention the other pages use. **Proved** means a proof is given or restated here; **Verified** means recomputed from scratch by a lab study, not proved; **Conjecture** means neither.

## Exhaust, do not sample

A design is a subset of the `2^D` corners of the parity cube, so dimension `D` holds `2^(2^D)` of them: 4, 16, 256, 65536 at `D = 1..4`. Designs related by a symmetry of the cube draw the same shape, and quotienting by the hyperoctahedral group `B_D` of signed permutations leaves 3, 6, 22, 402 classes. (Verified three independent ways in [the core](core.md) by `lab/design-census`, and again by the hyperoctahedral walk of `lab/fill-polynomials`, which reproduces all four.)

That is small enough to be brutal with. Where another project would test a formula on the named examples, the sweep here runs every design in the dimension: all 256 in 3D, all 65536 in 4D. A statement quantified over designs is then a finite check rather than an induction, and a counterexample cannot hide in the part of the space nobody drew - a real risk in this family, where the fraction of classes that are nameable as a level-set or an axis pin tends to zero, so almost every design is a compound nobody has drawn. (Proved in [the core](core.md).)

Enumeration does run out. Past `D = 4` the design count outgrows any list, and the counting moves to the Burnside average, which needs only the cycle counts of the group and never builds an orbit. That is the one place the method changes shape: from *checking every object* to *proving a formula and evaluating it*. See [the bijection page](bijection.md), where the class count is proved equal to the number of NP-equivalence classes of Boolean functions, uniformly in `D`. That this count is the OEIS entry A000616 is Verified there, not proved, and cannot be otherwise.

## Every closed form is pinned to a render

The standing rule for a counting formula is that it is never checked only against another formula. It is checked against an array built cell by cell from the parity rule and summed, with no arithmetic in common.

This is not a slogan; it is the shape of the test suites. The fill-class census compares its closed form against an independently rendered array for every design it censuses, at every side `n = 1..12`, and raises on any disagreement. The second generator behind the worked example below builds the `n^D` grid and counts. In the shipped code the same pattern holds: `crates/mrlymath/src/formulas` carries the fill engine and its hexagonal projection formulas, and their tests compare each closed form against a rendered cell - `fill_matches_rendered_sum` against a built array, `pro_and_cut_match_census` against an actually projected one. (Verified: `cargo test -p mrlymath` passes, those two among its 270 unit tests.)

A formula proposes; the render disposes. Everything downstream - dimensions, polynomials, densities - inherits its credibility from that comparison.

## What a sequence has to survive

[The sequence ledger](sequences.md) states the standard in full and records how each entry met it. In short, a sequence ships only with two independent generators that share no code and no method, a b-file diffed against both over the widest range they can reach, an independent re-verification by a second reader who reruns from the published directory and invents their own checks, and a novelty pass against a local OEIS dump at several windows, shifts and transforms plus the live entry where it matters.

The standard earns its cost by catching things. Re-verification of one entry catches a wrong cross-reference for `27^n` - A001024, which is `15^n` - and replaces it with A009971, powers of 27. (Verified: both entries read from the live OEIS, A001024 reading "Powers of 15" and A009971 "Powers of 27".) Another entry passes on its terms, b-file, closed form and novelty, and still ships marked defective, because its draft's own program block seeds the grid one level too high and so skips a term; the data was right and the generator printed was not. Neither error is visible to a reader who only checks that the numbers look plausible.

## One engine, then a census

The organizing move is to write one script that reads a design's invariants off its definition, then sweep it over the whole space, rather than to derive a formula per named family. The named designs stop being special cases and become rows.

The fill law is what makes this possible. With `E = ceil(n/2)` and `O = floor(n/2)` the number of even and odd residues available to one coordinate, a design `F` fills

```
fill(F, n) = sum over c in F of E^(D - w(c)) * O^(w(c))
```

cells of the `n^D` grid, `w(c)` being the number of odd coordinates of the corner `c`. (Proved: a cell is filled exactly when its parity vector is a filled corner, and for a fixed corner each coordinate independently has `E` or `O` admissible values, so the corner contributes that product; distinct corners contribute disjointly. The level-`L` fill is this raised to the `L`, proved in [the core](core.md) from the Kronecker product.) The law takes a design as data, so one implementation covers the entire space - it is the engine in `crates/mrlymath/src/formulas/counting.rs` and the engine behind every census in `lab/`.

Three censuses run on that principle, and their honesty is uneven in a way worth stating plainly. `lab/design-census` sweeps 58 designs across bases 2 and 3 and dimensions 2 and 3, validating each cell by cell and checking its orbit counts against a Burnside average, and writes a 59-line csv. (Verified.) The same study measures 763 designs against a predicted coprime density and flags none. (Verified: 763 design lines, of which 522 are spanning and so inside the density claim and 241 are degenerate and excluded by construction; every per-case summary reports zero flagged.) But that `OK` verdict is numerical agreement inside a flat tolerance at a shallow level, not a proof - the deepest level is capped, and the solid half of that census is the exact finite-level identity, not the limit. `lab/fill-polynomials` is the sweep behind the worked example below.

## The three tags

The tags are a claim about evidence, not about confidence.

- **Proved.** A proof is given or restated on the page, and the reader can follow it without running anything. The design count, the fill law, the bijection onto Boolean functions up to NP-equivalence.
- **Verified.** Recomputed from scratch, usually more than once and usually including a route that shares no method with the first. Some claims can never be more than this: that a particular OEIS entry counts what we say it counts is a fact about the entry, checkable against its stated definition and its terms, not provable from our side.
- **Conjecture.** Neither - a fitted pattern or a limit with a proved mechanism and an unclosed step. The coprime densities are conjectures with proved Euler factors. The two multiplicity laws of the triangle's Laplacian on [the complexity page](complexity.md) fit eight levels, and they are tagged as such even though they look inevitable.

A claim that cannot be re-established does not ship; what cannot be reproduced is dropped rather than softened.

## False friends

Each of these looks like evidence and is not.

- **A `1/zeta(2)` in a density is not a zeta connection.** Every coprimality density carries it, by Mobius inversion over squares. RH is about `zeta` in the critical strip, not about its special values, and `6/pi^2` is not a bridge - it appears in [farey](farey.md), [pi](pi.md) and [bases](bases.md) without any of them containing RH content. This is the likeliest trap on the tree.
- **Fill polynomials are not Jensen polynomials.** Griffin, Ono, Rolen and Zagier is about real-rootedness of polynomials built from Taylor coefficients of `xi`; this tree's come from parity counting. The resemblance is the whole of the connection.
- **Catalan, `phi`, `pi` and `L(2, chi_-3)` are eigenvalues, not omens.** A substitution rule with characteristic polynomial `x^2 - x - 1` produces `phi`, and the tree's own mass laws say so. Constants appearing is the expected behaviour of any sufficiently rich combinatorial system.
- **Exhaustive verification is not proof, and one story settles it.** The strong Mertens conjecture `|M(n)| < sqrt(n)` was believed, verified far, and is FALSE - Odlyzko and te Riele, 1985. Put that in front of anyone proposing to promote a pattern on the strength of exhaustive verification, including this tree's own Conjecture tags.

## Before spending a pass

Five questions, answered in writing before the work starts.

1. Is the object already standard under a different name?
2. Is the proposed theorem stronger than a computation, and can its quantifiers be written in one sentence?
3. What known theorem would make it immediate, and which hypothesis fails?
4. What would falsify it at the smallest nontrivial level?
5. If it is true, who besides this project would cite it, and for what use?

If the answer to the last is only "it resembles RH", keep it as an exhibit. If it is "it gives a reusable digit-restricted sieve or automaton lemma", it belongs in the proof queue.

## What counts as an anomaly

The mixed-product universe is infinite, so brute enumeration alone will manufacture false stories. Every claimed anomaly carries all six of these or it is not one.

1. A formal schedule and canonical factor names.
2. A baseline that fixes every trivial multiplicative invariant.
3. Two independent implementations, one direct-tensor and one coordinate or digit based.
4. An invariance audit under harmless reorderings and symmetries.
5. A stated search universe and a negative-control family.
6. A proof target that would explain the observed difference.

A changed fill, a changed dimension or a familiar constant is not an anomaly. It is an anomaly only when a fixed-control comparison violates a justified structural expectation.

## What a negative result has to name

A negative result is final only when every candidate is counted or a proof covers the space. Testing exactly one candidate state for the connected-component count of a mixed Kronecker word - the four-corner partition of the running product - finds it exact at length 2 and wrong on 20 of 216 words at length 3, and "the component count is not a finite-state function of the code sequence" does not follow: a rank-4 linear representation exists ([connectivity](connectivity.md)), so that claim is **Refuted**. What the failed candidate actually bounds is the naive geometric state, which does grow like `2^(L-1)` - a true theorem, wearing a false hat. The rule this leaves: name the class of descriptions a negative result rules out, or it rules out nothing.

## Worked example: the odd-side fill polynomial

One theorem, taken through the whole procedure.

**Theorem (Proved).** Fix a dimension `D` and a design `F` with popcount `p = |F|`. At odd side `n = 2k-1`, the fill is an integer polynomial in `k` of degree at most `D` whose coefficient of `k^D` is `p` - so the degree is exactly `D` for every non-empty design, and only the empty one falls short. *Proof.* At `n = 2k-1` there are `E = k` even residues and `O = k-1` odd ones, so the fill law reads `fill(F, 2k-1) = sum over c in F of k^(D-w(c)) * (k-1)^w(c)`. Each summand is a product of `D` linear integer factors, hence a monic integer polynomial of degree `D` in `k`; a sum of `p` of them is an integer polynomial whose `k^D` coefficient is `p`, and it is identically zero exactly when `p = 0`.

**Corollary (Proved).** Popcount is invariant under cube symmetry, so the leading coefficient - and with it the fractal dimension - is a class invariant. The lower coefficients are not. Parity flips are symmetries of the infinite tiling but not of the truncation to `n` cells, so members of one class can fill differently at a fixed side; the polynomial in the table below belongs to the canonical representative, the smallest code in the class.

**Corollary (Proved).** The caveat has an exact witness in the table's own Menger row. `mrly_023` is self-complementary - complementing its corner set is a cube symmetry, which is why carpet and net name one class in [the core](core.md) - but complementing does not commute with truncating. The complement member `mrly_232` fills exactly the cells the canonical member leaves void, so its polynomial is the sponge's own void count,

```
(2*k - 1)^3 - (4*k^3 - 3*k^2) = 4*k^3 - 9*k^2 + 6*k - 1
```

as exact polynomials, and vice versa: the canonical polynomial is `mrly_232`'s void count. Same leading coefficient 4, the class popcount; different tail. The orbit's eight members carry four distinct polynomials - `4*k^3 - 3*k^2`, `4*k^3 - 5*k^2 + 2*k`, `4*k^3 - 7*k^2 + 4*k - 1`, `4*k^3 - 9*k^2 + 6*k - 1` - one class filling four ways at a fixed odd side. (Proved by expansion; Verified by cell-by-cell counts at `k = 1..9`, each pair summing to `(2k-1)^3`, and by an orbit walk over the 48 signed permutations, `lab/hexagonal-slice-census`.) The complement's fill sequence `0, 7, 44, 135, 304, ...` is `(k-1)^2 * (4*k - 1)`, OEIS A395241 - one truncation of the sponge's own class, not a new design.

**Corollary (Proved).** The coefficient vector fixes the fill at every side and every level - the ledger's sense of two designs drawing the same fractal. The `D + 1` polynomials `k^(D-w) * (k-1)^w` are linearly independent - setting `k = 0` kills every term but `w = D`, then dividing by `k` and repeating kills the rest in turn - so the polynomial determines how many filled corners carry each Hamming weight, which is the popcount profile that fixes the fill at every side and every level. That is the same lemma the fill-class identity rests on in [the ledger](sequences.md), and it makes the number of distinct polynomials in dimension `D` equal to `Prod_{w=0..D} (1 + C(D,w))`, which is the closed form of A129824.

**Verified** (`lab/fill-polynomials`). Two generators sharing no code and no method: one sums the closed form over filled corners and interpolates with exact rational arithmetic, the other builds the `n^D` grid cell by cell, tests each cell's parity vector, and fits by finite differences. Both run over every design - all 4, 16 and 256 at `D = 1, 2, 3` - and agree term for term at `k = 1..8`, the grid count matching the closed form on 256 of 256 designs at `D = 3` over the six odd sides the census renders. The closed-form sweep extends to all 65536 designs at `D = 4`; in every dimension the polynomial fitted on `k = 1..D+1` predicts the true fill out to `k = 10`, the coefficients come out integral, and the leading coefficient equals the popcount with no exceptions. Distinct polynomials number 4, 12, 64, 700 at `D = 1..4`, which is A129824 at index `D` (read live: offset 0, terms `2, 4, 12, 64, 700, ...`). The lower coefficients split 4 of 6 classes at `D = 2`, 20 of 22 at `D = 3` and 400 of 402 at `D = 4`, independently reproducing a caveat the fill-class census records; the leading coefficient splits no class anywhere. A third route agrees: `lab/design-census`, written separately, carries the level-1 fill at `n = 1..12` for each 3D class representative, and its six odd columns match both generators on all 22 rows.

The full 3D table, one row per class, ordered by popcount. (Verified as above.)

| design | popcount | fill at `n = 2k-1` |
|---|---:|---|
| `mrly_000` | 0 | `0` |
| `mrly_001` | 1 | `k^3` |
| `mrly_003` | 2 | `2*k^3 - k^2` |
| `mrly_006` | 2 | `2*k^3 - 2*k^2` |
| `mrly_024` | 2 | `2*k^3 - 3*k^2 + k` |
| `mrly_007` | 3 | `3*k^3 - 2*k^2` |
| `mrly_022` | 3 | `3*k^3 - 3*k^2` |
| `mrly_025` | 3 | `3*k^3 - 3*k^2 + k` |
| `mrly_015` | 4 | `4*k^3 - 4*k^2 + k` |
| `mrly_023` | 4 | `4*k^3 - 3*k^2` |
| `mrly_027` | 4 | `4*k^3 - 4*k^2 + k` |
| `mrly_030` | 4 | `4*k^3 - 5*k^2 + k` |
| `mrly_060` | 4 | `4*k^3 - 6*k^2 + 2*k` |
| `mrly_105` | 4 | `4*k^3 - 6*k^2 + 3*k` |
| `mrly_031` | 5 | `5*k^3 - 5*k^2 + k` |
| `mrly_061` | 5 | `5*k^3 - 6*k^2 + 2*k` |
| `mrly_107` | 5 | `5*k^3 - 7*k^2 + 3*k` |
| `mrly_063` | 6 | `6*k^3 - 7*k^2 + 2*k` |
| `mrly_111` | 6 | `6*k^3 - 8*k^2 + 3*k` |
| `mrly_126` | 6 | `6*k^3 - 9*k^2 + 3*k` |
| `mrly_127` | 7 | `7*k^3 - 9*k^2 + 3*k` |
| `mrly_255` | 8 | `8*k^3 - 12*k^2 + 6*k - 1` |

The two ends are classical and forced. `mrly_001` has one filled corner and fills `k^3`, the cubes; `mrly_255` is the solid cube of side `2k-1` and fills `(2k-1)^3`, the odd cubes. (Proved by the theorem, both endpoints; Verified against the live OEIS - A000578, `a(n) = n^3`, and A016755, `a(n) = (2n+1)^3`.) The Menger sponge is one interior row, `mrly_023`, filling `4*k^3 - 3*k^2`; at `n = 3`, which is `k = 2`, that reads 20, and the celebrated dimension `log(20)/log(3)` is one evaluation of an ordinary row. Two rows are identical: `mrly_015` and `mrly_027` are distinct symmetry classes - different shapes, related by no cube symmetry - that carry the same polynomial and so fill identically at every side and level. The two classifications cut across each other rather than refining one another - a polynomial can be shared by two classes, and fill is not constant within one - so the 22 classes carry 21 distinct polynomials while the 256 designs carry 64. (Verified, `lab/design-census`.)

The proof is valid in every dimension and the sweep covers every design; the `D = 3` statement checked to `k = 6` on the 22 class representatives is the special case.

**Not claimed.** The other twenty polynomials have not been through the novelty procedure - no dump grep, no shifted windows, no live search - so nothing is asserted about whether they already sit in the OEIS under other names. They are the output of a sweep, not a claim of new sequences, and they are printed here in that spirit.

## Where the scripts live

Each result on these pages has a study under `lab/` holding its generators and a README saying what it computes, how to run it, and which page lines it witnesses - including the checks that do not resolve in the project's favour. The rule: prose without the code is a claim, not a result.
