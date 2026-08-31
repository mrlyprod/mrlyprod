# Magic words

A design's fractal is one rule substituted into itself by the Kronecker product. A magic word lets the rule change with the scale: an ordered list of letters, one per level, folded with the first letter outermost, `A_w = A_(c_1) (x) ... (x) A_(c_L)`. The constructor is `mrlymath::bang::magic` in [the crates](../crates/mrlymath), and this page fixes the grammar of the word, sorts the families that collapse back into the ordinary self-similar theory from the ones that do not, and says plainly which of the survivors are already published outside this tree.

The short answer is that most of the space collapses. A constant word is a fractal under another name, a periodic word is one composite tile, and over a finite alphabet every word whose letter frequencies exist has a scale dimension equal to a frequency average. What survives is not dimension at all: it is the observables that see the order of the letters, and the arithmetic of the composite codes the catalog cannot otherwise name.

**Proved** means a proof is given here; **Verified** means recomputed by a crate test or a lab study; **Conjecture** means neither. The staircase numbers below are printed by `lab/slice-ladder-controls`, and the base-15 composition by `lab/complex-dimensions`; every quantity on this page that has no generator is written as a question, never as a number.

## The grammar

A letter is a design, a residue base and a rendered side, written `design[.q<base>](<side>)`, as in `carpet(3)`, `net(5)`, `c495.q3(9)`. The design part is an alias or `c<code>`: a code is a cell bitmask over the `base^D` grid, which at base 2 is the corner bitmask of [the core page](core.md), and the aliases name tile-source codes, in the plane `carpet 7, net 14, htree 3, vtree 5, void 9` with the rest listed in the [crate name tables](../crates/mrlymath/NAMES.md). An alias names a code, not a symmetry class: carpet and net collapse into one class at base 2 ([core](core.md)) and stay distinct letters here because the two codes render distinct tiles inside a word. A code at base `q > 2` whose digit set is not a parity rule is a perfectly good tile but not a mrly design, and inherits nothing from the design census ([dimensions](dimensions.md)).

A word is a comma list of letters, first letter outermost, written `carpet(3), net(5), void(7)`, with the call form `magic(carpet(3), net(5), void(7))` when a verb is wanted. The numbers-only form `magic(3,5)` is sugar for the all-carpet word and is the spelling [dimensions](dimensions.md) already uses; the letter `carpet_3` there is `carpet(3)` here. Dimension is a property of the word, since the constructor rejects mixed dimension, and is prefixed once as `d3:` when it is not the plane. Base is a property of the letter, and mixed-base words are legal. Hex is a view rather than a dimension, suffixed `| iso`, `| pro` or `| cut` on a solid word, because a hexagon here is always the diagonal slice of a cube.

A letter is native when its rendered side equals its residue base, and `.q2` is never written, exactly as `q2` never appears in a design name. Only a fully native word has filled points of mixed-radix digit form, and a letter rendered at a side unrelated to its base is a tile product that inherits no digit theorem for free ([dimensions](dimensions.md)). Printing both numbers is what keeps that distinction visible in the notation itself.

The compact token for links, file names and URLs is `mrly_word_d<D>[_<view>]_<letter>_<letter>[_i]` with letter `c<code>[q<base>]n<side>[r<rot>][a]`, mirroring the tile grammar of the [crate name tables](../crates/mrlymath/NAMES.md) with the base field inserted and the trailing `_i` still the invert of the composite. The tile grammar itself is plane-only and caps codes at the corner range, so a solid or base-q word has no canonical crate name; the open questions carry the missing word name kind.

Order is part of the object and the notation is ordered. Side, fill, density and the main-diagonal count are functions of the letter multiset alone (**Proved**, [DISCOVERIES](DISCOVERIES.md)), and contact counts are exactly multiplicative (**Verified**, same ledger), so a sorted letter list is a legitimate index; components, Euler characteristic, holes, boundary and the anti-diagonal profile are order-sensitive ([connectivity](connectivity.md)), so a sorted list is never the word. **The hyperoctahedral group acts diagonally through the Kronecker product. Proved.** For `g` in `B_D`, `g . (A (x) B) = (g . A) (x) (g . B)`, because reflecting a mixed-radix coordinate reflects every digit at once, `(nm - 1) - x = (n - 1 - i) m + (m - 1 - j)`; so a word canonicalises under one shared `g` applied to all letters, never letter by letter in independent orbits.

## What a periodic word is

**A periodic word is not a word, it is a letter. Proved.** By associativity of the Kronecker product, a word repeated `L` times is the ordinary self-similar theory of its one-period composite tile, at side the product of the sides and fill the product of the fills; the statement is on [dimensions](dimensions.md) and in [the ledger](DISCOVERIES.md). When every letter is native the composite is one residue rule at base the product of the bases, in general not itself a parity design: the alternating base-3 and base-5 word is exactly one base-15 tile on the residue set `{0, 4, 10, 14}`, checked both as integer arithmetic and as geometry (**Verified**, `lab/complex-dimensions`), and [dimensions](dimensions.md) states plainly that this composite is not a parity design.

This is the law the rest of the page obeys. Any claim of novelty for a periodic schedule is a claim about a self-similar tile in disguise, and the correct control for any schedule experiment is the frequency-average value defined below. In the outside vocabulary a periodic word is a cycle in a graph-directed construction, where the collapse is the standard observation ([Mauldin and Williams 1988](REFS.md)).

The composite is where periodic words stay interesting, and it is a catalogue question rather than a dimension question. The composite of a two-letter word lives at a side equal to the product of the sides, and the code universe the crates can name is capped far below that, so most composites are objects the catalog can build and cannot name. Which codes are Kronecker products at all, whether a code can factor in two genuinely different shapes, and what the irreducible codes are, is open and unmeasured here.

## What is known elsewhere

Words are not a new object outside this tree, and the honest position is that the tree owns the alphabet, not the construction. Stated once: a magic word is a non-autonomous iterated function system in the sense of [Rempe-Gillen and Urbanski 2016](REFS.md), restricted to a parity-rule alphabet, and in the plane its members are generalised Sierpinski carpets in the sense of [Cristea and Steinsky 2010](REFS.md).

- A word at similarity maps is a Moran construction ([Moran 1946](REFS.md)), and the dimension formulas for varying-ratio constructions ([Feng, Wen and Wu 1997](REFS.md)), specialised to a common base, give the scale-dimension formula. It is cited, not claimed, and the same literature treats the lower and upper limits separately because they can differ.
- Connectedness of level-varying plane carpets has published necessary and sufficient conditions ([Cristea and Steinsky 2010](REFS.md)), so the order-sensitivity results on [connectivity](connectivity.md) are positioned against that literature rather than stated cold.
- Mixed labyrinth fractals already own the word mixed, with box-counting dimension and arc structure of level-varying patterns studied there ([Cristea and Steinsky 2017](REFS.md)).
- A randomised schedule is a V-variable fractal at `V = 1` ([Barnsley, Hutchinson and Stenflo 2008](REFS.md)), whose dimension theory runs through products of random matrices.
- Several ratios inside one level, which no move on this tree can express, is the multiscale substitution programme ([Smilansky and Solomon 2021](REFS.md)).
- A word is an S-adic directive sequence in symbolic dynamics ([Berthe and Delecroix 2014](REFS.md)), where the question of which aperiodic words behave well is a mature subject. These identifications are read from abstracts and surveys, and this page uses them as aliases and prior art, never as an argument.
- False friend: inhomogeneous self-similar sets are a fixed condensation set unioned in at every step ([Fraser 2012](REFS.md)) and have nothing to do with level-varying rules.

What is genuinely this tree's is narrower and firmer. The alphabet is finite, enumerated and classified by symmetry, `2^(2^D)` designs in [A000616](REFS.md) classes ([core](core.md)), where the outside literature takes each level's rule as an arbitrary given subset. The composition law is the Kronecker product, which turns block reduction into one line of associativity rather than a graph-directed computation. The arithmetic side, filled points as digit strings in a non-stationary radix, is essentially absent from a metric-geometric literature. And order as a control experiment, holding side, fill, density and dimension fixed while only the letter order moves, is an experimental design the varying-ratio literature has no reason to run.

## What collapses

- **A constant word is a fractal. Proved.** Repeating one letter is repeated self-Kronecker, which is what the level parameter already means, and the recipe layer refuses to draw such a word as magic at all.
- **A periodic word is one composite tile. Proved.** As above.
- **Over a finite alphabet, the scale dimension is a frequency functional. Proved.** The log side and log fill of a length-`L` prefix are sums of per-letter values drawn from a finite set, so if each letter's frequency `f_c` exists both averages converge, the denominator average is at least `log 2`, and `d_L = sum log k_i / sum log n_i` converges to `(sum_c f_c log k_c) / (sum_c f_c log n_c)`. Thue-Morse, period-doubling and Fibonacci words are uniquely ergodic ([Berthe and Delecroix 2014](REFS.md)), so their letter frequencies exist, so their scale dimension exists and is the frequency average: the famous aperiodic words cannot witness non-stationary behaviour in dimension. What remains of the existence question on [dimensions](dimensions.md) is words over a finite alphabet with no letter frequencies, and every word over an unbounded alphabet, where frequencies can all exist while the dimension still oscillates and where the staircase itself lives.
- **Every value in `[0, log 8 / log 3]` is a scale dimension. Proved.** Over the two base-2 letters `carpet(3)` and `c8(3)`, a word where `carpet(3)` has frequency `f` has scale dimension `f log 8 / log 3`; constant and periodic words realise every rational `f` with the endpoints, and Sturmian words, whose letter frequencies exist at every irrational slope ([Berthe and Delecroix 2014](REFS.md)), realise every irrational `f`. The open question is not which values occur but which trajectories `L -> d_L` do.
- **One even letter removes the hex reading. Proved.** Slice geometry is defined at odd side only ([slices](slices.md)), and a product with an even factor is even, so a single even letter anywhere makes the composite side even and puts the whole hex and slice apparatus out of scope. A constraint on the lane, not a research question.
- **Reversal comparisons are empty on palindromes. Proved.** A palindrome equals its own reversal by definition; no further property is claimed for them.

## What order does

Order sensitivity is where the lane's content actually is, and this page owns no number of it. [Connectivity](connectivity.md) records which observables separate two orderings of the same letters, with the minimal witness, and states that no lab study regenerates the counts of that section; they are cited there and not restated here.

The mechanism is understood even where the counts are not. Components, Euler characteristic, boundary and holes are linear-representation functions of the word, a fixed vector times a product of integer matrices, one matrix per letter (**Conjecture**, as [connectivity](connectivity.md) tags it while no generator exists). Most matrix pairs fail to commute, so the word acts as a genuine matrix cocycle, and that is the only structure on this tree an aperiodic schedule can still reach once the dimension collapses to a frequency average.

**The component cocycle's growth rate along a uniquely ergodic word differs from the frequency-average prediction. Conjecture.** A difference is the tree's first genuinely non-stationary result; equality across every letter pair is an order-blindness theorem instead, provided the class it holds for is named. For products over an ergodic measure the growth rate exists at almost every word ([Furstenberg and Kesten 1960](REFS.md)); existence along one named word is part of the question, not a given, since for subadditive quantities unique ergodicity alone does not force pointwise convergence.

## The staircase

The staircase word stacks prefixes, `magic(3)`, then `magic(3,5)`, then `magic(3,5,7)` ([dimensions](dimensions.md)), so the letter in position `j` occurs `n - j + 1` times in the first `n` blocks, side and fill are the matching products of letter powers, and `dim_n = sum_j (n-j+1) log f_j / sum_j (n-j+1) log q_j` (**Proved**, read off the occurrence count). It is aperiodic and not eventually periodic, so block reduction does not apply to it; it is the cheapest concrete non-stationary schedule on the tree.

**The odd-carpet fill law. Proved.** The carpet code rendered at odd side `q` fills the cells whose coordinates are not both odd: with `E = (q+1)/2` even positions and `O = (q-1)/2` odd positions per axis, the fill is `E^2 + 2EO = (3q^2 + 2q - 1)/4 = q^2 - ((q-1)/2)^2`, the octagonal number `3k^2 - 2k` at `q = 2k - 1` already identified for this rule on [sequences](sequences.md). This discharges the fill assumption `lab/slice-ladder-controls` states before printing, so its five staircase dimensions `1.892789261, 1.892315261, 1.893034267, 1.894190425, 1.895495742` stand (**Verified**, `lab/slice-ladder-controls`, under the fill law proved here). The sequence is not monotone: it dips at the second term, because the base-5 carpet is less dense than the base-3 carpet, before climbing.

**The staircase dimension tends to the ambient dimension. Proved.** The per-letter dimension `log f_j / log q_j` tends to `2` as `q_j` grows, and `dim_n` is the weighted average of the per-letter dimensions under the weights `(n-j+1) log q_j`, where the first `J` letters carry weight `O(n)` of a total that grows like `n^2 log n`, so the average inherits the tail limit. The staircase is dimensionally boring in the limit, full dimension and zero measure, and its only content is the rate of approach, which is unmeasured here.

## Not claimed

- No word on this tree is nonlattice. Every move chooses cells of a fixed grid, so every level has one ratio; nonlattice needs incommensurable ratios inside one level, which the multiscale substitution literature has and this construction does not.
- Nothing here is adjacent to the Riemann hypothesis. The inverse spectral problem is vacuous for lattice designs, and no schedule of lattice designs restores it.
- The scale-dimension formula is not a discovery of this tree, and neither is the fact that the limit can fail to exist. Both are cited above.
- No count of order sensitivity, no component growth rate, and no factorisation census is asserted on this page, because no generator on this tree computes them.
- One anti-diagonal profile coefficient is never quoted alone: the total is order-blind and the profile is not ([DISCOVERIES](DISCOVERIES.md)), so the whole profile is the observable or nothing is.

## OPEN QUESTIONS

- Which words make the scale dimension exist, now reduced to finite alphabets without letter frequencies and to unbounded alphabets; a word-combinatorics question, not a dimension-theory one.
- Which trajectories `L -> d_L` are realisable by a schedule over a finite alphabet: monotone, bounded oscillation, prescribed rate.
- Does the component cocycle along a uniquely ergodic word differ from the frequency-average prediction, and if never, for which class of letters.
- Which codes are Kronecker products, whether a code can factor in two shape-distinct ways, and what the irreducible codes are; the irreducibles would be the true alphabet of the whole programme.
- When do two letters render one tile at one side, as the base-2 carpet and a base-3 code do at side 3 before diverging at side 9; a side-indexed collision the canonical form must name.
- Does applying a different symmetry to each letter move any order-sensitive observable, given the representation already separates codes the square's symmetry group identifies ([connectivity](connectivity.md)).
- What a mixed-base word does to the base-specific arithmetic of [bases](bases.md), whose structures exist at some bases and provably not at others, since a word may cross bases from letter to letter.
- What the word name kind and its parser should be, since a word has no canonical name outside the plane at base 2 and cannot be quoted in a sequence link without one.
- Is the rate at which the staircase approaches the ambient dimension a clean expression in the letters.

## Where the numbers live

The five staircase dimensions come from `lab/slice-ladder-controls`, which states its fill assumption before printing; the assumption is proved above. The base-15 composition comes from `lab/complex-dimensions`. The design and class counts come from the `mrlymath` census on [the core page](core.md). The order-sensitivity record is [connectivity](connectivity.md) and [the ledger](DISCOVERIES.md). Everything else on this page is a proof restated in place or a question.

> The word is ordered, the dimension is not, and everything interesting lives in the gap.
