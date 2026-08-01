# The millennium map

Seven problems, one small fractal project, and the honest distances between
them. The research notes have kept a running score of how the project's
threads sit next to the Clay Millennium Prize Problems - first as a single
number per problem, later as two. This essay is that map, redrawn with
everything that has since been re-run in public.

One ground rule before any table: a score is a judgement, not a computation.
What can be checked is the evidence a score leans on, and every number quoted
below has been recomputed in the open lab - `research/mrlymath/lab/millennium/`, `research/mrlymath/lab/porous/`,
`research/mrlymath/lab/yangmills/`, `research/mrlymath/lab/levels/`, `research/mrlymath/lab/complex_dimensions/`.

## The first survey

The first pass scored each problem once, 0 to 10, on "how real is the link".

| problem | link | the one-line reason |
|---|---|---|
| Navier-Stokes | ~4 | Stokes flow through carpet and sponge pores is a real, simulable sub-problem |
| Riemann | ~3 | coprime-node density is `1/zeta(d)`; spectra tested against GUE came back clustered - a clean negative |
| Yang-Mills | ~3 | self-similar graphs open exact spectral gaps - a vocabulary analogy to the mass gap |
| P vs NP | ~2 | the `lcm(1..n) ~ e^n` assembly wall is a teaching parable for verify-versus-solve |
| Hodge | ~1 | exact Betti numbers of the cell complexes are real but unrelated |
| Birch-Swinnerton-Dyer | ~0-1 | no elliptic curve appears anywhere in the project |
| Poincare | ~1 | solved by Perelman; diffusion methods are a methodological cousin of Ricci flow |

The scores are self-reported and nothing in them can be rerun, but the
arithmetic under the reasons can. `research/mrlymath/lab/millennium/survey.py` rebuilds the two
that are computations: `lcm(1..n)^(1/n)` walks to `e` - at `n = 3000` the lcm
holds 4330 bits against 4329 for `e^3000` - and the exact Mobius count agrees
with `1/zeta(d)` to a few parts in ten thousand at `d = 2, 3, 4`. The reasons
are sound. The scores stayed opinions.

## Two axes

A later pass split the single number in two: *link* - is the mathematical
connection real - and *handle* - could anything computable move the problem.
The split matters because the two questions turn out to have opposite answers
in the most interesting row.

| problem | link | handle |
|---|---|---|
| Riemann | 6 | 0 |
| Navier-Stokes | 4 | 2 |
| Yang-Mills | 3 | 1 |
| P vs NP | 2 | 0 |
| Hodge | 1 | 0 |
| Birch-Swinnerton-Dyer | 0 | 0 |
| Poincare | - | - |

Poincare is solved and left unscored. Only one row moved from the first
survey: Riemann, from an undifferentiated ~3 to link 6, handle 0. It is also
the only row with something checkable underneath; the other six remain
judgements and should be read as judgements.

## The Riemann amendment

Lay the same grid on the unit interval at every scale `n = 1..Q` and add the
layers up. The bright points - the nodes many scales agree on - are exactly
the Farey fractions `F_Q`, the reduced fractions with denominator at most
`Q`. That identification is proved, not an analogy: the stack's nodes are not
*like* the Farey sequence, they are the Farey sequence.

Which matters because of two theorems from 1924. Franel proved that
`sum delta_j^2 = O(Q^(-1+eps))` for every `eps > 0` - with `delta_j` the
distance of the `j`-th Farey node from perfect equidistribution - is
equivalent to the Riemann hypothesis; Landau, in the note published
immediately after, proved the same for `sum |delta_j| = O(Q^(1/2+eps))`. So
the question "how evenly are the stack's bright points spread?" is not
related to RH. At this level of precision it is RH.

The meter reads what RH predicts: `S2*Q` flattens near `0.656` as `Q` runs
from 125 to 8000 and its local exponent walks to `-1.00`, the Franel rate,
while `S1/sqrt(Q)` falls from `0.20` to `0.11`, well under Landau's threshold
of `0.5`. And here the second axis earns its keep: RH is already verified
numerically far beyond any range this meter can reach, so the table can only
ever illustrate what is known. The connection is exact, and it is untouchable
- link 6, handle 0, and both halves deserve to be said together. The full
result, tagged claim by claim, is public at [primes](https://git.mrly.net/research/mrlymath/primes);
the meter is `research/mrlymath/lab/millennium/franel.py`.

## Navier-Stokes and the ordering effect

The sharpest physics-neighbourhood result started life on shaky paper: the
research notes reported it citing a script that nobody has since been able to
find. That is no pedigree to publish on, so the effect was rebuilt from
scratch - and it is real.

The object: three carpet tiles at bases 3, 5 and 7, pore fractions `8/9`,
`21/25` and `40/49`, composed by Kronecker product into one 105 x 105 sponge,
with a choice of which base sits outermost, middle and innermost. Kronecker
products commute in count, so all six orderings fill exactly the same 6720
pore cells - porosity cannot tell them apart. Simulated Darcy/Stokes flow
can: conductance runs from `0.3399751735` (ordering 3-5-7) to `0.3439279767`
(7-5-3), a spread of 1.16 percent, strongly anti-correlated with the
outermost tile's pore fraction, Pearson `-0.831251`. The sign, stated so it
cannot be read backwards: flow is *highest* when carpet(7), the tile with the
lowest pore fraction, sits outermost. Coarse blockage costs more than the
same blockage subdivided - carpet(3) outermost puts one solid 35 x 35 square
in the middle of the sample, while carpet(7) outermost breaks the same
blocked fraction into nine 15 x 15 squares that flow can route between.

Two honest fences. First, the original `-0.83` was a measurement recorded in
the research notes with its script since lost; it ships here on the strength
of the fresh runs, not the old note. Four independent routes now agree, three of them public -
`research/mrlymath/lab/porous/flow.py`, `research/mrlymath/lab/porous/check.py` and `research/mrlymath/lab/porous/recheck.py`,
different geometry constructions and different solvers, all six conductances
identical to ten decimal places - with a fourth adversarial re-derivation
during verification. Second, two clauses of the original claim did not
survive the rebuild: conductance is *not* monotonic in the outermost pore
fraction (the orderings interleave, which is exactly why the correlation is
`-0.83` and not `-1`), and the isotropy and contact-count checks the note
offered as evidence are automatic consequences of the tiles' symmetry and
prove nothing. What carries the weight is independent solvers agreeing to
`1e-10` while the orderings differ by `4e-3`.

Scope, plainly: level 1 in each scale, one 105 x 105 grid per ordering, two
dimensions. Nothing is claimed about deeper levels, about 3D, or about the
Navier-Stokes equations themselves. This is the physics neighbourhood, which
is what link 4, handle 2 says.

## Yang-Mills and the band gap

The code-23 design - the same rule that draws the Menger sponge at base 3 -
built at base 2 has a tile graph that is the star `K_{1,3}`, and its
Laplacian opens an exact band gap. At every level tested the value 4 is an
eigenvalue exactly and exactly once, nothing lies anywhere in `[2, 4)`, and
the top of the lower band climbs to 2: `1.000000` at level 1, `1.975680` at
level 3, `1.999605` at level 5. Exact arithmetic certifies the empty interval
through level 4 and dense spectra confirm it through level 6, so the width
converges to `2 = k - 2` with `k = 4` the popcount of the tile. The defect
`2 - lo` shrinks by a factor climbing toward 8 per level - ratios 5.80, 7.09,
7.75, 7.94, reading 8.0000 by level 11 in the lab's deepest run - an
`8^(-L)` rate that is a numerical fit, not a theorem.

The score stays at link 3, handle 1, because a spectral gap in a graph
Laplacian shares a *word* with the Yang-Mills mass gap, not the gauge theory.
The public spectral work is on [complexity](https://git.mrly.net/research/mrlymath/complexity); the
gap scripts are in `research/mrlymath/lab/yangmills/`.

## Two supporting threads

Both belong to the Riemann row's clean-negative half and its structural
counterpart. The spacings story is written up on
[complexity](https://git.mrly.net/research/mrlymath/complexity); the lattice story on
[dimensions](https://git.mrly.net/research/mrlymath/dimensions).

*Level spacings are Poisson-side, not GUE.* Had the fractal spectra shown the
level repulsion the zeta zeros show, that would have been a headline. They
show the opposite: every fractal tested - carpet cell graphs in 2D, Menger
cell and slice graphs in 3D - reads clustered, at roughly three times the GOE
prediction for small spacings, with a random-graph control reading GOE and a
square-lattice control reading clustered, as they should. The mechanism is
self-similarity forcing degeneracy: the largest eigenvalue multiplicity in
the slice graph grows from 12 at level 2 to 48 at level 3, against
multiplicity 1 for the random control. Established on spectra up to 4096
nodes; `research/mrlymath/lab/levels/recheck.py`.

*Every design is lattice-class.* The box-counting function of every proper
design - anything strictly between a single point and the full cube -
oscillates log-periodically forever, at the period the theory predicts - the
measured peak sits within a tenth of a percent of `2*pi/ln 3` for the base-3
Cantor design - and composing designs only multiplies the bases: base 3 with
base 5 lands at the base-15 period, still lattice. No composition of the
existing operators can leave the class; escaping it needs unequal cell sizes
inside one subdivision level, which no parity rule on a grid can express. One
honest trim from the re-derivation: "lattice implies not Minkowski
measurable" is a theorem on the line and a conjecture in higher dimension, so
for the 2D and 3D designs that clause is conjectured, not proved.
`research/mrlymath/lab/complex_dimensions/spectrum.py` and `research/mrlymath/lab/lattice/`.

## P vs NP, and the remainder

The P vs NP thread comes from an earlier and narrower survey in the research
notes - three problems, no scores - and re-running it mostly shrank it.
Reading designs as Boolean functions, certificate complexity equals block
sensitivity for every design at `D <= 4` - all `2^(2^D)` of them, by
exhaustion - and block sensitivity exceeds sensitivity by at most 1. True,
verified, and empty as evidence: at dimension `D` the designs are *all*
Boolean functions on `D` variables, and a collapse at so few variables is
exactly what the classical bounds predict, since the known separating
constructions need far more. A fact about small functions, not about P versus
NP. `research/mrlymath/lab/complexity/collapse.py`.

Hodge, Birch-Swinnerton-Dyer and Poincare keep their near-zero rows for the
reasons in the first table: real Betti numbers with no Hodge content, no
elliptic curve anywhere, and a solved problem admired from a distance.

## What the map is for

The standing note the research notes have always carried belongs at the end
of the public version too: nothing here claims a Millennium problem,
approaches one, or expects to. Five links score 4 or less, the one that
scores 6 has a handle of 0, and the seventh problem is solved. The value is the map itself - knowing which
threads touch something real (a genuinely RH-equivalent observable, a
reproducible transport effect), which are vocabulary (a band gap that shares
a name with a mass gap), and which are nothing (an elliptic curve the project
has never met). A small project that knows its distances can walk anywhere
without falling into a famous hole.
