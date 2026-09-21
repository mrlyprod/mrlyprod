---
title: Integers
lead: The census the other way round: which integers the whole registry writes, which it never writes, and which it writes thousands of times.
figure: research-integers
slug: integers
---

Which integers this work writes, which it never writes, and which it writes many times - taken over the whole registry rather than over one favourite sequence. The [sequences](../sequences.md) ledger asks whether a given sequence is known; this page asks the opposite question, what the union of every sequence the registry holds covers, and answers it by census.

The generator is `lab/rs/integer-census`, one pass over `mrlylab::ledger::keys`, which prints its definition before any table and writes `rows.csv`, `multiset.csv` and a manifest into a directory given on the command line. Every number below is a line of that run. The registry it walks is the one [sequences](../sequences.md) is rendered from and the [sequences demo](../../site/demos/sequences/) searches live; the closed forms it replays are the fill law and the exposure recurrence of the [sequence-census paper](https://github.com/carlomitchener/carlomitchener/tree/main/research/sequence-census). The [integers demo](../../site/demos/integers/) reads that union integer by integer: which of the first thousand the designs write, how many rows write each, and which the pinned window misses. The [plot demo](../../site/demos/plot/) draws a row of the same ledger rather than listing it, with the smallest linear recurrence its terms satisfy, its characteristic polynomial and its growth beside it.

**Proved** means a proof is given here; **Verified** means recomputed by the study; **Conjecture** means neither; **Refuted** means the study kills it.

## The definition

The census is only as good as its window, so the window is pinned and printed, never assumed.

- A registry row is one `(design, measure, axis)` key of `mrlylab::ledger::keys` over the four cost tiers.
- A row's rendered window is its first `min(48, B)` terms, `B` the leading terms whose footprint fits `100000` cells, under the ledger's own budget of `100000` cells a term.
- A term's footprint is 1 cell for a closed measure, `number^dimension + level * span` for a convolved measure, `number^(dimension * level)` for a grid measure.
- A row whose rendered terms are strictly increasing stops at the first term above `100000`; the count of rows truncated this way is printed, never assumed to lose nothing.
- Row `R` writes `n` iff `n` is a term of `R` inside `R`'s rendered window and `1 <= n <= 100000`.
- Multiplicity counts rows, not `(row, index)` pairs: a row writing `n` at several indices counts once.
- An integer `n` appears iff some row writes it, and is missed iff no row writes it.

## The census

- **The registry is 18066 rows and every tier count is derived twice. Verified.** 7692 closed, 5044 convolved, 2665 side grid, 2665 level grid, each matched against an independent count over `SPACES`, `ledger::designs` and `Measure::applies`; no row goes unread. Truncation is declared and counted: 5529 rows stop at the ceiling, 6802 at the 48-term cap, 5735 at a cell budget, and 390 write no integer in the window at all.
- **The two readings of multiplicity differ, and the honest one is smaller. Verified.** There are 347308 `(row, integer)` incidences against 360703 `(row, index, integer)` incidences, so 13395 times a row writes the same integer twice and the row reading refuses to count it twice. 29144 rendered terms are at or below zero - [Euler characteristics](/wiki/euler-characteristic/), the voids of a solid design - and are excluded and reported, never folded in.

| window | never | once | multiple | written | share written |
|---|---|---|---|---|---|
| `1..=1000` | 41 | 31 | 928 | 959 | 0.9590 |
| `1..=10000` | 3589 | 765 | 5646 | 6411 | 0.6411 |
| `1..=100000` | 88867 | 2897 | 8236 | 11133 | 0.1113 |

| decade | width | missed | miss density |
|---|---|---|---|
| `1..9` | 9 | 0 | 0.000000 |
| `10..99` | 90 | 0 | 0.000000 |
| `100..999` | 900 | 41 | 0.045556 |
| `1000..9999` | 9000 | 3548 | 0.394222 |
| `10000..99999` | 90000 | 85278 | 0.947533 |

- **The written set is finite, so the miss density tends to 1. Proved.** A row renders at most 48 terms, so whatever the ceiling, the registry writes at most `48 * 18066 = 867168` integers. The registry is a fixed finite object and the integers are not: past `867168` the census is almost all miss, and no growth of the ceiling changes that. This is the one statement on the page that survives any change of window.

## The miss set

- **Every integer to 268 is written and the first miss is a prime. Verified.** 269 is missed, and `1..268` is the longest written run in the window; the longest missed run is 447 wide, on `95265..95711`, with 95264 and 95712 both written. The ceiling itself, 100000, is written by 103 rows. The first thirty misses are `269, 362, 422, 443, 446, 487, 502, 538, 607, 611, 618, 626, 643, 653, 659, 668, 677, 691, 698, 701, 709, 723, 758, 773, 787, 797, 803, 835, 857, 878`.
- **The miss set is not a union of residue classes. Refuted.** All 2079 classes mod `2..64` hold a written integer on `10000..100000`, exhaustively; no modulus in that range separates written from missed.
- **The bias is divisibility and smoothness, not congruence. Verified.** On the tail `10000..100000` the written count by residue mod 12 runs `1175, 440, 145, 194, 715, 176, 420, 224, 531, 358, 229, 116`, a ratio of `10.13` between residue 0 and residue 11, and mod 6 it runs `1595, 664, 676, 552, 944, 292`, a ratio of `5.46`. Sorted by greatest prime factor the written share on the same band falls `0.5798`, `0.1406`, `0.0506`, `0.0313`, `0.0117` across the bands `1..10`, `10..100`, `100..1000`, `1000..10000`, `10000..100000`. Of the 9592 [primes](/wiki/prime-numbers/), 750 are written and only 158 of the 8363 above `10000`; the first missed prime is 269, the first missed integer.
- **Every cube, fourth, fifth and sixth power is written; the squares are not. Verified.** Cubes `46/46`, fourth powers `17/17`, fifth `10/10`, sixth `6/6`, all of `1..100000`. Squares run `176/316`: every square to `98^2 = 9604` is written and `99^2 = 9801` is not, and the largest written square is `97969 = 313^2`, written by exactly one row, `sequence dim 4, code 28662, voids, side`, whose closed form `4k^4 - 8k^3 + 8k^2 - 4k + 1` is the square of the centered square numbers.
- **The square frontier is the cap, not arithmetic. Verified.** Row multiplicity at `96^2, 97^2, 98^2, 99^2, 100^2` is `321, 19, 480, 0, 123`: the dense square families are exhausted, not excluded. Deepening the window to 96 terms, in the section below, writes at least 228 of the 316 squares and moves the first missed square from `9801` to `38809 = 197^2`. Oddness excludes nothing: `97969` is odd and written, and `9801` is missed for want of depth.

## The depth of the window

The whole miss set is a statement about the rendered window, and the study measures how much of one rather than asserting it is harmless.

| rendered window | written | missed | first miss |
|---|---|---|---|
| 8 terms | 5263 | 94737 | 269 |
| 32 terms | 8749 | 91251 | 269 |
| 48 terms | 11133 | 88867 | 269 |

- **More than half the written set arrives past the head. Verified.** 5870 of the 11133 written integers appear only past term 8 and 2384 only past term 32, so a census read off the ledger's own 8-term heads sees less than half of what 48 terms see, and its miss set starts `269, 281, 302, 311` rather than `269, 362, 422, 443`.
- **A row's written column is rebuilt from its head and the stop rule alone. Verified.** 3608 rows have a head whose finite differences terminate at order 6 or less, by degree `207, 1104, 569, 518, 1202, 0, 8`. Extending the head by Newton forward differences and applying the pinned stop rule reproduces the row's `written` column exactly for 1306 of 1306 ceiling-stopped rows and 1325 of 1333 cap-stopped rows; the 8 failures are exactly the rows whose head degree reads 6, which eight terms cannot certify. The 969 budget-stopped rows carry no rendered length in their head and are not testable this way, which is said rather than hidden.
- **Deepening the cap moves every window-relative number except the longest missed run. Verified.** Extending only the 1325 cap-stopped rows the rebuild reproduces, out to 96 terms, gives a strict lower bound on the 96-term census: at least 11898 integers written, the first miss moved from 269 to 362, the longest written run at least 361, at least 228 of 316 squares. 269 becomes written; the run `95265..95711` does not move.
- **That a missed integer is written by no row at any depth is Conjecture.** 6802 rows are cut by the cap and their deeper terms are not rendered here; the 96-term reading is a lower bound, not a census, and the true frontier of the written set is not known at any depth.

## The champions

| rank | integer | rows | rank | integer | rows |
|---|---|---|---|---|---|
| 1 | 16 | 2858 | 6 | 64 | 2176 |
| 2 | 9 | 2811 | 7 | 3 | 1951 |
| 3 | 4 | 2559 | 8 | 6 | 1883 |
| 4 | 12 | 2303 | 9 | 8 | 1790 |
| 5 | 36 | 2270 | 10 | 33 | 1777 |

- **The whole top of the census is small and mostly a power. Verified.** All twenty champions lie below 65 - ascending, `2, 3, 4, 6, 7, 8, 9, 12, 14, 15, 16, 18, 20, 21, 24, 25, 33, 36, 49, 64` - and they carry 39007 of the 347308 incidences, a share of `0.1123`. The 366 perfect powers of `1..100000` carry 58906 incidences, a share of `0.1696` against a density of `0.003660`: `46.34` times their weight.
- **The champions are not the divisor-rich integers. Refuted.** On `1..1000` the mean row count is `193.42` over all integers, `995.26` over the squares and `920.58` over the perfect powers, but only `170.60` over the 413 integers with at least eight divisors - below the overall mean. Being a small perfect power is what a champion is; being highly divisible is not, and reads slightly against it.
- **Multiplicity is not driven by each row's first term. Refuted.** Dropping every row's first rendered term removes 17036 of the 347308 incidences, `4.9%`, and changes nothing that matters: the written set stays 11133, the never counts stay `41`, `3589`, `88867` in all three windows, and the leaders stay `36` at 2212, `64` at 2112, `16` at 2000, `9` at 1999 - the same integers in a different order.
- **The multiplicity spectrum is neither geometric nor a power law. Refuted.** With `S(m)` the count of integers written by at least `m` rows, `S(1) = 11133` and `S(2) = 8236` give a ratio `0.7398`, which predicts `S(64) = 6.312e-5` against the observed 977 - wrong by seven orders. The spectrum takes 410 distinct values with a maximum of 2858.
- **The effect is a property of a measure column, not of a design. Verified.** `euler.side` writes 1 in 695 of its 859 rows, `peak.side` writes 12 in 809 of its 1261, `heights.side` writes both 9 and 33 in 765 of its 1261. A champion is an integer that one reading of the geometry returns for most designs at once.
- **The one champion that is neither small-smooth nor a power is an offset. Verified.** 33 ranks tenth and is `3 * 11`. The eight integers below 100 that `heights.side` writes most often are `9, 17, 25, 33, 41, 49, 57, 65`, every one of them `1 mod 8`: the column runs arithmetic progressions whose common difference is a power of two, and `33 - 1 = 2^5`. The arithmetic of the champion is the arithmetic of the step, not of the integer.
- **The closed tier carries the census and the grid tiers carry its tail. Verified.** Of the 11133 written integers the closed tier covers 7628 with 3983 exclusive, the side grid 6203 with 2603, the level grid 1826 with 541, the convolved tier 792 with 130. Above `30000` there are 2174 written integers and the closed tier covers 1853 of them.
- **The tail is not a few dominant families. Refuted.** Restricted to `30000..100000` the rows' written sets collapse to 953 distinct families, and 875 of those families own a tail integer no other family writes - between them 2005 of the 2174 tail integers. So every cover of the written tail needs at least 875 families, and the tail is a wide superposition rather than a handful of dominant sequences.

## Against the OEIS

Every search below is exhaustive on both sides: every window of the census sequence is indexed and every record of a local copy of [the OEIS stripped dump](https://oeis.org/stripped.gz) is walked against that index, so no sampling of offsets is involved. The dump read holds 398817 records. Under the standing caveat of [sequences](../sequences.md) a dump is a snapshot, so every absence here is **Conjecture** and needs a live re-read before it is repeated.

- **The miss set is new to the OEIS only in its dense head. Conjecture.** No record carries any 4-term window of the miss set at offsets `0..416`; the first hit is at offset 417, in [A049537](https://oeis.org/A049537). Above that the miss set does hit, and the hits are near-interval records rather than identifications: 852 hits at window length `k = 4`, 130 at `k = 10`, 37 at `k = 15`, 15 at `k = 20`, the 20-term witnesses being [A112820](https://oeis.org/A112820) and [A118471](https://oeis.org/A118471), each a sequence that runs a block of consecutive integers through a region the census misses wholesale. The head is the informative part and is clean: the string `,269,362,422,443,` appears nowhere in the dump.
- **The write-once set is absent at every offset. Conjecture.** The 2897 integers written by exactly one row have no hit at any offset of any record at `k = 4, 10, 15, 20` - a cleaner absence than the miss set's, because the once set is thin where the miss set is an interval complement.
- **The champion set meets two records for exactly twelve terms. Verified.** Ascending, the twenty champions open `2, 3, 4, 6, 7, 8, 9, 12, 14, 15, 16, 18`, which is a window of [A100290](https://oeis.org/A100290) and of [A336231](https://oeis.org/A336231) and of no other record. All three part at the thirteenth: A100290 gives 21, A336231 gives 19, the census gives 20. Both records are binary-digit conditions, which is the right neighbourhood - the registry's designs are corner subsets of a [parity](/wiki/parity/) cube - and neither is the champion set.
- **The written-per-decade run meets one record and parts at the next term. Verified.** The written counts by decade are `9, 90, 859, 5452, 4722` with 100000 itself, summing to 11133. The prefix `9, 90, 859` sits inside [A209631](https://oeis.org/A209631) alone, an exponential-transform array, which continues 6689 where the census gives 5452.
- **No recognizable family is systematically missed. Verified.** 173 records hold at least ten distinct integers of `1..100000` and lie wholly inside the miss set, the longest being [A361796](https://oeis.org/A361796) at 41 terms. At a miss density of `0.88867` a 41-term run of misses has probability about `10^-2.1`, which 398817 records make ordinary: the census excludes nothing a catalogue would recognise, it just runs out of depth. The [tour demo](../../site/demos/tour/) runs the other way in a dozen cards, each drawing a design beside the sequence it counts and the OEIS record that holds the terms.

## What is left

- Whether any integer of `1..100000` is written by no row at any depth. The 96-term reading is a lower bound and already moves at least 765 of the misses across, 269 among them; the honest frontier needs a cap the dimension-2 side grid can pay for, and that tier costs `cap^3`. **Conjecture.**
- Whether the miss set has any arithmetic characterisation at all. No modulus to 64 separates it, no growth order does, and it is closed under nothing; the only theorem on offer is the finiteness bound above. **Conjecture.**
- Whether the 953 tail families are 953 rules. The families are de-duplicated by written set and not by generating rule, so two rules with equal truncated value sets merge and 953 is a lower bound on the number of rules, never an upper one. **Conjecture** that no bounded union of named families reaches the written tail.
- Whether the multiplicity function, `a(n)` the number of rows writing `n`, is worth an entry. It is absent from the dump, but it is a function of the registry's own shape - the tier mix, the cap, the ceiling - and not of `n` alone, so it is a reading of this instrument and not a sequence of the integers. **Conjecture** that no reparametrisation of it is submittable.

## THE FIELD LADDER

A design is a corner set `C` of the parity cube `{0,1}^dim`, its signature is `s_j = #{c in C : weight(c) = j}` and its weight enumerator is `W(t) = sum_j s_j t^j`. The fill at odd side `2n + 1` is `P(n) = sum_(c in C) (n+1)^(dim - weight(c)) n^weight(c) = (n+1)^dim W(n/(n+1))`, the polynomial [sequences](../sequences.md) counts with. This section reads that polynomial as a product of norm forms, one per irreducible factor of `W` over `Q`, and censuses the number fields those forms carry. The generator is `lab/py/field-ladder`, eight verbs `norm`, `ladder`, `sign`, `fields`, `swap`, `hunter`, `beyond`, `polya`; the box is the origin-filled box `s_0 = 1`, `0 <= s_j <= C(dim, j)`, which is `6, 32, 350, 8712, 526848` signatures at dim 2..6 carrying `2^(2^dim - 1)` oriented designs.

- **The fill is a product of norm forms, one per irreducible factor of the weight enumerator. Proved.** Write `W = cont(W) prod_i g_i^(e_i)` over `Z` with each `g_i` irreducible and primitive, `cont(W)` the content, and `m = deg W`. Substituting `t = n/(n+1)` and clearing `(n+1)^dim` gives `P(n) = (n+1)^(dim-m) cont(W) prod_i g_i*(n)^(e_i)` with `g*(n) = (n+1)^(deg g) g(n/(n+1))`, and `g*(n) = lc(g) prod_theta ((1 - theta) n - theta)` over the roots `theta` of `g`, the norm form of `Q(theta)` evaluated at `n(1 - theta) - theta`. The constant is the content and never the leading coefficient: at dim 2 the signature `(1,2,0)` has `W = 1 + 2t`, `cont(W) = 1`, `lc(W) = 2` and fill `(n+1)(3n+1)`. On the origin-filled box `cont(W) = 1` always, since `W(0) = s_0 = 1`. The substitution is the Mobius map of matrix `[[1,0],[1,1]]` in `SL_2(Z)`, so `Q(theta/(1 - theta)) = Q(theta)`, and `disc(g*) = disc(g)` whenever `g(1) != 0`, which is `deg g* = deg g`, automatic for irreducible `g` of degree at least 2. The identity and its resultant form `P(n) = (n+1)^(dim-m) Res_t(W(t), n - t(n+1))` are exact on all `16`, `256` and `65536` designs at dim 2, 3, 4 over `12`, `64` and `700` signatures; the discriminant equality is checked over the `6`, `32` and `350` origin-filled signatures of those dimensions, `0` mismatches on the `2`, `25` and `343` factor slots of degree at least 2 (`norm`).
- **The bare form `P = s_dim prod_theta ((1 - theta) n - theta)` needs the lift `(n+1)^(dim - deg W)` on exactly half the designs. Proved.** `deg W < dim` iff `s_dim = 0` iff the all-odd corner is empty, and `C -> C xor {all-odd}` is a fixed-point-free involution of the designs, so the count is `2^(2^dim - 1)`, which is `8` of `16`, `128` of `256` and `32768` of `65536` at dim 2, 3, 4 (`norm`).
- **With the origin filled every rational root of `W` is `-1/k` and every linear factor of `P` is `(a n + 1)`. Proved.** `W` has nonnegative coefficients and `W(0) = s_0 = 1`, so `W(x) >= 1` for `x >= 0` and no factor of `W` has a positive real root; `W(0) = 1` makes `W` primitive, so `cont(W) = 1` and `prod_i g_i(0)^(e_i) = 1`, and each `g_i(0)` is `1` or `-1`, and `g_i(0) = -1` with positive leading coefficient forces a positive real root, so `g_i(0) = 1`. A linear factor is then `1 + k t` with `k >= 1` and `(1 + k t)* = (k+1) n + 1`. This is why the divisor tribe of [sequences](../sequences.md) is the all-rational floor of the ladder and why its factors are `(a n + 1)` and never `(a n + b)` with `b > 1`. With the origin empty the law fails: signature `(0,2,1)` at dim 2 has `W = t^2 + 2t` and `P(n) = n(3n + 2)` (`norm`).
- **The sponge rule fills a divisor form in every dimension, with minimal avatar `4, 24, 240, 3360` at `dim 1` to `4`. Proved.** Keeping the cells with at most one odd coordinate is the signature `(1, dim, 0, ..., 0)`, so `W = 1 + dim t` and the fill is `(n+1)^dim + dim n (n+1)^(dim-1) = (n+1)^(dim-1)((dim+1) n + 1)`, a product of `dim` linear factors `(a n + 1)` of exponent pattern `(dim+1, 1, ..., 1)`; the avatar map of the [divisor avatars paper](https://github.com/carlomitchener/carlomitchener/tree/main/research/divisor-avatars) then reads it as the [divisor function](/wiki/divisor-function/) `d(x^n)` for `x = 2^(dim+1) 3 * 5 * ... * p_dim`, the tower `4, 24, 240, 3360`, whose first three terms are [A005408](https://oeis.org/A005408), [A000567](https://oeis.org/A000567) and [A103532](https://oeis.org/A103532). Its `n = 1` column is `(dim+2) 2^(dim-1)`, [A001792](https://oeis.org/A001792), the level-1 cell count of the [Menger sponge](/wiki/menger-sponge/) in every dimension (`ladder`).
- **The rational floor is named integer by integer, one minimal avatar per signature. Verified.** The `Q` column of the ladder below counts the signatures whose `W` splits into linear factors over `Q`, `4, 7, 12` at `dim 2, 3, 4`, and the avatar map is a bijection from them to the exponent patterns of `prod_i (a_i n + 1)`, so each names one smallest integer: the seven at `dim 3` are `30, 60, 120, 180, 240, 360, 900` and the twelve at `dim 4` are `210, 420, 840, 1260, 1680, 2520, 3360, 5040, 6300, 7560, 12600, 44100`. The floor is closed under products, since a Kronecker product of designs multiplies fills and concatenates exponent patterns (`ladder`).
- **Adding one corner to the sponge leaves the rational floor for `Q(sqrt 5)`. Verified.** At `dim 3` the signature `(1,3,1,0)`, the sponge with one weight-2 corner added, has `W = 1 + 3 t + t^2` of discriminant `5` and fill `(n+1)(5 n^2 + 5 n + 1)`, the norm form of the real quadratic field of discriminant `5`, which is the first value of the real side of the pure layer (`norm`, `fields`).
- **The pure quadratic layer realizes exactly the imaginary quadratic fields of discriminant at least `-2 dim (dim - 1)`, and no others. Proved.** The pure signature `(1, b, c)` has `W = 1 + b t + c t^2` with `0 <= b <= C(dim,1) = dim` and `0 <= c <= C(dim,2)`, fill `P(n) = (n+1)^(dim-2)((1 + b + c) n^2 + (b + 2) n + 1)` and discriminant `b^2 - 4c` on both sides. Every `d = 0` or `1 mod 4` with `-4C(dim,2) <= d < 0` occurs, by `b = 0`, `c = -d/4` and by `b = 1`, `c = (1-d)/4`, and no smaller value occurs since `b^2 - 4c >= -4C(dim,2)`; those are `dim(dim-1)` values, the count the discriminant staircase row of [DISCOVERIES](/research/discoveries/) states. Passing to fields divides out the conductor, and the fields realized are exactly those of fundamental discriminant `d_K` with `abs(d_K) <= 4C(dim,2) = 2 dim (dim - 1)`, which is `2, 5, 10, 14, 21` fields at dim 2..6 and not `dim(dim-1) = 2, 6, 12, 20, 30` (`fields`).
- **The whole census adds no further imaginary quadratic field at dim at most 6. Verified.** Over every signature of the box the imaginary quadratic field discriminants are exactly the fundamental discriminants of the window above, so the run is gapless and the first gap is the first fundamental discriminant past `2 dim (dim - 1)`, namely `7, 15, 31, 43, 67` at dim 2..6 (`fields`).
- **The `dim(dim-1)` count law counts orders, and under the field reading the same layer counts `2, 5, 10, 14, 21`. Proved.** The two counts are the same statement read twice: `dim(dim-1)` counts the values `b^2 - 4c` that are `0` or `1 mod 4` in `[-4C(dim,2), -1]`, which are discriminants of quadratic orders and not all of them fundamental, `-12 = -3 * 2^2` being the first that is not; dividing out the conductor leaves `2, 5, 10, 14, 21` fields at dim 2..6. The two boxes must not be read against each other either: this section sweeps the `526848` origin-filled signatures at dim 6 and its field discriminants stop at `-59`, while the discriminant staircase row sweeps the `1053696` signatures with `s_0` free (`lab/py/fill-polynomials`) and its order discriminants reach `-63`, `-160` and `-899`; the deepest, `-899`, is carried by the origin-empty signature `(0,0,15,1,15,0,0)`, whose `W = t^2 (15t^2 + t + 15)` has no constant term and so falls outside the box of this section, and `-63` is inside it, carried by `(1,0,15,16,0,0,0)` with `W = (1 + t)(1 - t + 16 t^2)`, an order of the field `-7` (`fields`, `sign`).
- **The real quadratic side is where the census beats the pure layer. Verified.** At dim 6 the pure layer gives `5, 8, 12, 13, 17, 21, 24, 28` and the census adds `29` and `33`, both from signatures whose weight enumerator has degree above 2, before its first gap at `37` (`fields`).

| `dim` | signatures | `Q` | quadratic | cubic | quartic | quintic | sextic |
|---|---|---|---|---|---|---|---|
| 2 | 6 | 4 | 2 | | | | |
| 3 | 32 | 7 | 13 | 12 | | | |
| 4 | 350 | 12 | 62 | 130 | 146 | | |
| 5 | 8712 | 19 | 266 | 955 | 3522 | 3950 | |
| 6 | 526848 | 30 | 1173 | 7305 | 45292 | 222437 | 250611 |

- **The ladder is a census by the top degree of the irreducible factors of `W`. Verified.** The table counts signatures, exhaustively at dim 2..6, the dim 6 row over `526848` signatures carrying `2^63` oriented designs; the next table counts the same box by oriented design, a signature carrying `prod_j C(C(dim, j), s_j)` of them (`ladder`).

| `dim` | designs | `Q` | imaginary | real | mixed | cubic | quartic | quintic | sextic |
|---|---|---|---|---|---|---|---|---|---|
| 2 | 8 | 5 | 3 | | | | | | |
| 3 | 128 | 21 | 60 | 3 | | 44 | | | |
| 4 | 32768 | 504 | 6518 | 105 | 261 | 13241 | 12139 | | |
| 5 | 2147483648 | 1209703 | 92090824 | 85372 | 10793445 | 213022933 | 956166567 | 874114804 | |
| 6 | 9223372036854775808 | 60292748226072 | 37730659810317711 | 282117813525 | 4034425906484660 | 221284670882930634 | 644853816469624496 | 4271134780921329524 | 4044273107998049186 |

- **Counted by oriented design the quadratic class splits by field sign, and the real side is the thin one at every `dim`. Verified.** The table counts the `2^(2^dim - 1)` origin-filled oriented designs by the top degree of the irreducible factors of `W`, the quadratic column split three ways by the sign of the discriminant of its quadratic factors: imaginary when every quadratic factor has negative discriminant, real when every one has positive discriminant, mixed when both signs occur, so the three cells partition the quadratic column and `6518 + 105 + 261 = 6884` at `dim 4`. By signature the split reads `2, 0, 0` at `dim 2`, `12, 1, 0` at `dim 3`, `55, 4, 3` at `dim 4`, `223, 12, 31` at `dim 5` and `939, 30, 204` at `dim 6`. At `dim 5` the top two degrees carry a share of `0.85` of the designs while the rational floor carries less than one in a thousand (`ladder`).

| `d_K` | first `dim` | `dim 3` | `dim 4` | `dim 5` | `dim 6` | pure at `dim 6` | designs at `dim 6` |
|---|---|---|---|---|---|---|---|
| -3 | 2 | 5 | 17 | 120 | 2038 | 12 | 161635271209613747 |
| -4 | 2 | 3 | 17 | 104 | 1835 | 11 | 85702305526654908 |
| -7 | 3 | 1 | 6 | 34 | 430 | 6 | 22979667824049915 |
| -8 | 3 | 2 | 6 | 31 | 370 | 7 | 20505118680031582 |
| -11 | 3 | 1 | 4 | 19 | 230 | 6 | 9550722596669410 |
| -15 | 4 |  | 4 | 13 | 109 | 4 | 4321318090757755 |
| -19 | 4 |  | 1 | 11 | 69 | 3 | 1189654713609618 |
| -20 | 4 |  | 2 | 9 | 74 | 4 | 883929279919685 |
| -23 | 4 |  | 1 | 4 | 46 | 3 | 227539849733070 |
| -24 | 4 |  | 1 | 5 | 43 | 4 | 198666153227341 |
| -31 | 5 |  |  | 4 | 16 | 3 | 56108718293970 |
| -35 | 5 |  |  | 3 | 15 | 3 | 22335524206586 |
| -39 | 5 |  |  | 2 | 12 | 2 | 4965179083098 |
| -40 | 5 |  |  | 2 | 13 | 3 | 4581266056446 |
| -43 | 6 |  |  |  | 8 | 2 | 616822912020 |
| -47 | 6 |  |  |  | 8 | 2 | 43084458830 |
| -51 | 6 |  |  |  | 7 | 2 | 1502769500 |
| -52 | 6 |  |  |  | 6 | 2 | 781446255 |
| -55 | 6 |  |  |  | 5 | 1 | 22095090 |
| -56 | 6 |  |  |  | 5 | 2 | 6976845 |
| -59 | 6 |  |  |  | 3 | 1 | 271326 |

| `d_K` | first `dim` | `dim 3` | `dim 4` | `dim 5` | `dim 6` | pure at `dim 6` | designs at `dim 6` |
|---|---|---|---|---|---|---|---|
| 5 | 3 | 1 | 4 | 27 | 399 | 3 | 14327618707869470 |
| 8 | 4 |  | 1 | 7 | 84 | 3 | 1776358253208990 |
| 12 | 4 |  | 2 | 10 | 147 | 2 | 1710320428947910 |
| 13 | 5 |  |  | 2 | 27 | 1 | 257933527274115 |
| 17 | 5 |  |  | 3 | 30 | 1 | 237912468491580 |
| 21 | 5 |  |  | 4 | 53 | 1 | 107843480191074 |
| 24 | 6 |  |  |  | 12 | 1 | 14634227500075 |
| 28 | 6 |  |  |  | 10 | 1 | 8424036675705 |
| 29 | 6 |  |  |  | 2 | 0 | 504456555 |
| 33 | 6 |  |  |  | 1 | 0 | 518700 |

- **Field by field, an imaginary quadratic field first appears exactly where the pure layer puts it, at every `dim` at most 6. Verified.** A signature carries the field `K` when some irreducible quadratic factor `1 + b t + c t^2` of its `W` has field discriminant `d_K`, the fundamental part of `b^2 - 4c` by exact integer arithmetic, agreeing with PARI `quaddisc` on all `43` distinct values of `b^2 - 4c` over `dim 2..6`. The tables count the signatures carrying each field at each `dim`, the pure column those with `deg W = 2`; the first `dim` of each of the `21` imaginary fields is the least `dim` with `2 dim (dim - 1) >= abs(d_K)`, the pure law. The fields per `dim` number `2, 5, 10, 14, 21` imaginary and `0, 1, 3, 6, 10` real, and at `dim 4` the three cells of the quadratic class sum to the `6884` of the ladder (`sign`).
- **By oriented design the count per imaginary field falls with `abs(d_K)`; by signature it does not. Verified.** The designs column is strictly decreasing down the imaginary table at `dim 4, 5, 6` and weakly at `dim 3`, with `-3` and `-4`, the two imaginary quadratic fields with more than two units, at the top at every `dim`. By signature `-20` outranks `-19` at `dim 6` (`74` against `69`), `-40` outranks `-39` at `dim 6` (`13` against `12`) and `-24` outranks `-23` at `dim 5` (`5` against `4`): the signature count follows the number of representations `d_K f^2 = b^2 - 4c` inside the box, which favours `d_K = 0 mod 4`, so it is not a function of `abs(d_K)` alone. On the real side the designs column falls at `dim 5` and `dim 6` and not at `dim 4`, where `12` carries `21` designs against `15` for `8` (`sign`).
- **The pure law of the first `dim` fails at `dim 7`, and the failure is a family. Refuted.** `(1 - t + 22 t^2)(1 + t) = 1 + 21 t^2 + 22 t^3` is the signature `(1,0,21,22,0,0,0,0)` of the box at `dim 7`, its factor has discriminant `-87 = -3 * 29`, fundamental, and no field below `-59` occurs at `dim 6`, so the field `-87` first appears at `dim 7` where the pure law says `dim 8`. The family `(1 - t + c t^2)(1 + t) = 1 + (c - 1) t^2 + c t^3` with `c = C(dim, 2) + 1` is in the box iff `C(dim, 2) + 1 <= C(dim, 3)`, which is `dim >= 6`, and carries the order discriminant `-(2 dim (dim - 1) + 3)`, three past the pure window; it is a field discriminant whenever `2 dim (dim - 1) + 3` is squarefree, which over `dim 6..15` is `-87, -115, -183, -223, -267, -367` at `dim 7, 8, 10, 11, 12, 14` and an order at `dim 6, 9, 13, 15`, where `63 = 7 * 3^2` and `147 = 3 * 7^2`. The family is Proved; the whole census at `dim 7` is below (`beyond`).
- **Every quadratic factor of a box signature at `dim` has `c <= 1/(2^(1/dim) - 1)^2`, so the imaginary side is boxed between the pure window and about four times it. Proved.** A root `theta` of `W` has `1 = abs(sum_(j >= 1) s_j theta^j) <= (1 + abs(theta))^dim - 1`, so `abs(theta) >= R = 2^(1/dim) - 1`; the two roots of `1 + b t + c t^2` have product `1/c`, so `c <= 1/R^2` and `abs(d_K) <= 4c - b^2 <= 4c`; on the real side the root of smaller modulus is `2/(b + sqrt(d))`, so `b + sqrt(d) <= 2/R`. The bound reads `c <= 14, 27, 45, 66, 92, 122, 156, 194` at `dim 3..10` against the pure `C(dim, 2) = 3, 6, 10, 15, 21, 28, 36, 45`, and `1/R^2` grows like `(dim / log 2)^2`; the first `dim` of an imaginary field lies between the least `dim` with `4 / R^2 >= abs(d_K)` and the pure `dim`, and the census sits at the pure end (`beyond`).
- **The census at `dim 7` and the cut at `dim 8..10` find one mechanism on each side, and every field found outside the pure layer sits one `dim` before its pure `dim`. Verified.** `beyond` walks every `W = (1 + b t + c t^2) h` with `(b, c)` inside the root bound and `h` in `Z[t]` of degree at most `H`, pruned by the partial sums at the roots of the factor, `abs(sum_(i <= j) W_i theta^i) <= sum_(i > j) C(dim, i) abs(theta)^i`, a pruning an unpruned control at `dim 6` and at `dim 7`, `H = 3` reproduces to the last `W` (`6179` and `27809`). At `H = dim - 2` the walk is exhaustive: at `dim 3..6` it returns exactly the `6, 13, 20, 31` fields of the full census, and at `dim 7` it is the full quadratic census, `28` imaginary and `14` real fields over `211529` signatures, the real ones every real fundamental discriminant to `44`. At `dim 8..10` it is a cut, `H = 4, 2, 2`, a lower bound reading `37 + 21`, `49 + 26`, `59 + 34`. Outside the pure layer it finds `-87` at `dim 7`, `-115, -116` at `dim 8`, `-148, -151, -152` at `dim 9` and `-183, -184, -187` at `dim 10` on the imaginary side, and `40, 44`, `53, 57, 61, 65, 69`, `76, 88, 92` and `85, 89, 93, 97, 101, 105, 109, 113` on the real side; every real one is `(dim + 1)^2 - 4c`, carried by `(1 + (dim + 1) t + c t^2)(1 - t + c' t^2)` with `c + c' = dim + 1`, whose `s_1 = dim` and `s_2 = 0`, which is how `29 = (1 + 7t + 5t^2)(1 - t + 2t^2)` and `33 = (1 + 7t + 4t^2)(1 - t + 3t^2)` arrive at `dim 6`, and the pure `dim` of every field listed is one more than the `dim` it is found at (`beyond`, `sign`).
- **A quadratic field first appears at its pure `dim` or one before it, never earlier. Conjecture.** The pure `dim` of a field is the least `dim` at which a pure signature carries it. The evidence is exhaustive at `dim` at most 7: `21` imaginary fields at their pure `dim` and `10` real fields at `dim 6`, of which `29` and `33` are one before, then `-87`, `40` and `44` one before at `dim 7`; the cut at `dim 8..10` adds `24` more fields outside the pure layer, all one before, `29` in all. The quadratic fields per `dim` number `2 + 0`, `5 + 1`, `10 + 3`, `14 + 6`, `21 + 10`, `28 + 14`, imaginary plus real at `dim 2..7`, exactly (`sign`, `beyond`).

| degree, signature | dim 3 | dim 4 | dim 5 | dim 6 |
|---|---|---|---|---|
| 2, `(0,1)` | 15 | 31 | 43 | 67 |
| 2, `(2,0)` | 8 | 13 | 24 | 37 |
| 3, `(1,1)` | 44 | 244 | 652 | past 815 |
| 3, `(3,0)` | empty | empty | 81 | 316 |
| 4, `(0,2)` | empty | 225 | 981 | past 2156 |
| 4, `(2,1)` | empty | 400 | 1423 | 3275 |
| 4, `(4,0)` | empty | empty | empty | 1125 |
| 5, `(1,2)` | empty | empty | 7684 | past 12752 |
| 5, `(3,1)` | empty | empty | 5783 | past 13883 |
| 5, `(5,0)` | empty | empty | empty | empty |

- **Each run is an initial segment of the table of smallest field discriminants, and the table above is where it stops. Verified.** Each cell is the smallest field discriminant of that degree and signature the box misses; a `past` cell means the run covers the whole table read from the LMFDB, 100 entries for the four large classes and 20 for the rest, and the gap is beyond it; `empty` means no factor of that class occurs at all. The runs are printed by the generator: at dim 6 the cubic `(1,1)` run `23, 31, 44, 59, ..., 815` and the quintic `(1,2)` run `1609, 1649, 1777, ..., 12752` are 100 long and the quintic `(3,1)` run `4511, ..., 13883` is 20 long (`fields`).
- **No field signature is excluded by the sign condition. Proved.** No irreducible factor of `W` has a positive real root, so every real root of every factor is negative, and this excludes no field: for a field `K` with generator `gamma`, the element `theta = -1/(gamma + N)` with `N` above every real conjugate of `gamma` generates `K`, has all real conjugates negative, and has `1/theta` an algebraic integer, so its primitive minimal polynomial has positive leading coefficient, constant term 1 and no positive real root, which are exactly the two conditions a factor of `W` satisfies.
- **The totally real classes are the sparse side of the ladder. Verified.** What the census shows is a delay, not an exclusion: signature `(3,0)` first occurs at dim 5 with the single field `49`, `(4,0)` at dim 6 with the single field `725`, and `(5,0)` does not occur at dim at most 6, where the smallest totally real quintic field is `14641` (`fields`).

| degree | dim 3 | dim 4 | dim 5 | dim 6 | first miss at dim 6 |
|---|---|---|---|---|---|
| 2 | 7 | 12 | 23 | 35 | `37` at `(2,0)` |
| 3 | 31 | 44 | 76 | 307 | `316` at `(3,0)` |
| 4 | none | 189 | 697 | 1107 | `1125` at `(4,0)` |
| 5 | none | none | 5753 | at least 12752 | past the table |

- **The box `0 <= s_j <= C(dim, j)` reaches every number field of degree `d` up to a bound `B(d, dim)`. Verified.** `B` is the largest bound with every field of degree `d` and absolute discriminant at most `B` reached. The merge over signatures is by field and not by absolute value: `8` is the discriminant of two fields, `-8` and `+8`, and at dim 3 only `-8` is reached, which is why `B(2,3)` is `7` and not `11`; a discriminant the table lists twice in one signature, `576`, `1008`, `1040` and `1088` below `B(4,6)`, is credited only when the box carries two non-isomorphic factors at it, and each of those four does. `none` means the smallest field of that degree is already missed, and `at least 12752` means the run passes the last table entry. The box height is `max_j C(dim, j)`, which is `3, 6, 10, 20` at dim 3..6, and the bounds grow far faster than the height (`hunter`).

The box is not chosen for a search, it is forced by the geometry: the corner counts of the parity cube give exactly `0 <= s_j <= C(dim, j)` with `s_0 = 1`. That a bounded-height search reaches every field of small discriminant is the classical mechanism behind the tables this section runs against: [Hunter 1957](../REFS.md) puts a generator of a quintic field of discriminant `D` at `abs(sum rho_i) <= 2` and `5 (sum abs(rho_i)^2)^4 <= 8 abs(D)`, [Pohst 1982](../REFS.md) turns a bound of that kind into the computation of the minimum discriminants of sixth degree fields, and the complete lists themselves are the database of [Jones and Roberts 2014](../REFS.md), whose minimal quintic root discriminants for `S_5` with two and with one complex place are `1609^(1/5)` and `(13 * 347)^(1/5)`, the `1609` and `4511` opening the runs above. What is new here is the box, the fractal reading and the fill and void involution, not the search.

- **Every number field appears at some finite `dim`, in the pure layer of its own degree. Proved.** Take an algebraic integer `gamma` generating `K`, of degree `d`, and an integer `N` above every real conjugate; the sign lemma's `theta = -1/(gamma + N)` has primitive minimal polynomial `g(t) = prod_i (1 + (gamma_i + N) t)`, whose coefficient of `t^k` is the elementary symmetric function `e_k(gamma + N)`, an integer, and `e_k(gamma + N) = sum_j C(d - j, k - j) N^(k - j) e_j(gamma)` is a polynomial in `N` with leading term `C(d, k) N^k`, positive for `N` large. Then `W = g` is the pure signature `(1, e_1, ..., e_d, 0, ..., 0)` of the box at the least `dim` with `e_k <= C(dim, k)` for every `k`, and the census question is only how early. Polya's theorem is the sharper tool for one generator with negative coefficients: a polynomial positive on `[0, oo)`, which every `g` with `g(0) = 1`, positive leading coefficient and no nonnegative real root is, has `(1 + t)^m g` with nonnegative coefficients for `m` large. The theorem is [Polya 1928](../REFS.md), unread in the original and read in the form statement of the [positive polynomial article](../REFS.md) and in the univariate restatement of [Tan 2018](../REFS.md), which carries the exponent of [Powers and Reznick 2001](../REFS.md), `m > (d^2 - d) L(g) / (2 lambda(g)) - d` with `L = max_j abs(g_j) / C(d, j)` and `lambda = inf_(t >= 0) g(t) / (1 + t)^d`; then `W = (1 + t)^m g` sits in the box at `dim = m + d + e` for any `e` with `g_k <= C(d + e, k)` at every `k`, since `W_j = sum_k g_k C(m, j - k) <= sum_k C(d + e, k) C(m, j - k) = C(m + d + e, j)` by Vandermonde, and the least `dim` over `m` and over eight translates `N` is what the verb prints. The `polya` verb runs both on the fields the box misses at `dim 6`, from the polynomial of each LMFDB label with its discriminant recomputed by `nfdisc`: the totally real `37`, `316`, `1125`, `14641` have exponent `m = 0` at the least translate `N`, since a product of `1 + alpha_i t` with every `alpha_i > 0` has positive coefficients, and land by `dim 9, 10, 9, 11`; `-87` has `m = 1` and lands at `dim 7` by `(1 + t)(1 - t + 22 t^2)`, one below its pure `dim 8`. Each is an upper bound from one generator, and the pure layer already reaches `37` at `dim 7` (`polya`, `beyond`).
- **The discriminants of each degree and signature arrive in order. Conjecture.** The evidence is the table above, `B(2, 6) = 35`, `B(3, 6) = 307`, `B(4, 6) = 1107` and `B(5, 6)` at least `12752` (`hunter`), each a gapless initial run against the LMFDB tables, and the quadratic runs at `dim 7..10`, exhaustive at `dim 7` and a cut past it, gapless to `87, 116, 152, 187` on the imaginary side and `44, 69, 77, 101` on the real side (`beyond`).

| `dim` | `P+ V+` | `P+ V-` | `P- V+` | `P- V-` | designs |
|---|---|---|---|---|---|
| 3 | 17 | 4 | 67 | 40 | 128 |
| 4 | 413 | 91 | 4994 | 27270 | 32768 |

- **The complement on the parity cube does not swap the two tribes, and the news is the count. Refuted.** With `V(n) = (2n+1)^dim - P(n) = n Q(n)`, `P+` means `P` splits into linear factors over `Q` and `V+` means `Q` does. The swap clause forbids the `P+ V+` cell alone, and that cell holds `17` of `128` origin-filled oriented designs at dim 3 and `413` of `32768` at dim 4; the smallest witness is the dim 3 design on corners `000` and `001`, fill `(n+1)^2 (2n+1)` and void core `(2n+1)(3n+2)`. That a witness exists was already known, the self-dual design being named as an exception where the clause is stated; what is new is that the exception is `0.13` of the designs at dim 3. The `P- V-` cell, `40` and `27270` designs, is not forbidden by the clause and is counted here only to show the census is dominated by it, `0.83` at dim 4 (`swap`).
- **Each field discriminant is computed and guarded, and the runs count discriminants while the bounds count fields. Verified.** The field discriminant of a factor is `nfdisc` of its reversed monic model and the field signature is `polsturm` of the factor, both from PARI; every value is then guarded against the polynomial discriminant, which must be a square multiple of it with the square root the index, and against `0` or `1 mod 4`. All `256179` distinct irreducible factors of degree 2 to 5 over dim 2..6 pass, so no factor is unresolved and no run rests on an unchecked value. Two fields can share a discriminant, so a run counts discriminants; the bounds above are lifted to fields by `nfisisom` (`fields`, `hunter`).
- **Nine of the fields the box carries are checked on their own source page. Verified.** Each is an irreducible factor of a weight enumerator in the box, read through its reversed monic model: `x^2 + x + 1` is [LMFDB 2.0.3.1](../REFS.md) at `-3`, `x^2 + 3x + 1` is [LMFDB 2.2.5.1](../REFS.md) at `5`, `x^3 + x^2 + 2x + 1` is [LMFDB 3.1.23.1](../REFS.md) at `-23`, `x^3 + 5x^2 + 6x + 1` is [LMFDB 3.3.49.1](../REFS.md) at `49`, `x^4 + 2x^2 + 3x + 1` is [LMFDB 4.0.117.1](../REFS.md) at `117`, `x^4 + 7x^3 + 13x^2 + 7x + 1` is [LMFDB 4.4.725.1](../REFS.md) at `725`, `x^5 + 2x^4 + x^3 + 4x^2 + 4x + 1` is [LMFDB 5.3.4511.1](../REFS.md) at `-4511`, `x^4 + x^3 + 12x^2 + 19x + 11` is [LMFDB 4.0.1225.1](../REFS.md) at `1225`, and `x^5 + 2x^3 + 8x^2 + 4x + 1` is [LMFDB 5.1.4429.1](../REFS.md) at `4429`. The label of a field is `degree.r1.abs(disc).index`, so `4.0.1225.1` is the totally imaginary quartic of discriminant `1225 = 5^2 7^2` and is a different field from the totally real `1125` the table above misses at `(4,0)`. Every one agrees with `nfdisc` of the model (`fields`).
- **Factor multiplicity is carried, not divided out. Verified.** A repeated factor `g^e` contributes `e` copies of its field to the ladder and one discriminant to the census, and the census of distinct factors is by polynomial, so a field with several generators inside the box is counted once per polynomial in the factor counts and once per field in the runs (`ladder`, `fields`).

## The rest of the tree

- [README](../README.md) is the front door: the parity cube, [the Kronecker product](/wiki/kronecker-product/), and the index of every page.
- [sequences](../sequences.md) is the ledger this page is the complement of: which sequences are known, against which integers are reached.
- [method](method.md) - how a claim here is produced and checked, worked through on the odd-side fill polynomial.
- [DISCOVERIES](/research/discoveries/) - where every line above is tagged with its witness and its refutation attempt.
- [REFS](../REFS.md) - every sequence id above resolved to a canonical URL.
