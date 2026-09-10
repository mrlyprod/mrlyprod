# Integers

Which integers this work writes, which it never writes, and which it writes many times - taken over the whole registry rather than over one favourite sequence. The [sequences](sequences.md) ledger asks whether a given sequence is known; this page asks the opposite question, what the union of every sequence the registry holds covers, and answers it by census.

The generator is `lab/integer-census`, one pass over `mrlylab::ledger::keys`, which prints its definition before any table and writes `rows.csv`, `multiset.csv` and a manifest into a directory given on the command line. Every number below is a line of that run. The registry it walks is the one [sequences](sequences.md) is rendered from and the [sequences demo](../demos/sequences/) searches live; the closed forms it replays are the fill law and the exposure recurrence of the [sequence-census paper](https://github.com/carlomitchener/carlomitchener/tree/main/research/sequence-census).

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
- **The two readings of multiplicity differ, and the honest one is smaller. Verified.** There are 347308 `(row, integer)` incidences against 360703 `(row, index, integer)` incidences, so 13395 times a row writes the same integer twice and the row reading refuses to count it twice. 29144 rendered terms are at or below zero - Euler characteristics, the voids of a solid design - and are excluded and reported, never folded in.

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
- **The bias is divisibility and smoothness, not congruence. Verified.** On the tail `10000..100000` the written count by residue mod 12 runs `1175, 440, 145, 194, 715, 176, 420, 224, 531, 358, 229, 116`, a ratio of `10.13` between residue 0 and residue 11, and mod 6 it runs `1595, 664, 676, 552, 944, 292`, a ratio of `5.46`. Sorted by greatest prime factor the written share on the same band falls `0.5798`, `0.1406`, `0.0506`, `0.0313`, `0.0117` across the bands `1..10`, `10..100`, `100..1000`, `1000..10000`, `10000..100000`. Of the 9592 primes, 750 are written and only 158 of the 8363 above `10000`; the first missed prime is 269, the first missed integer.
- **Every cube, fourth, fifth and sixth power is written; the squares are not. Verified.** Cubes `46/46`, fourth powers `17/17`, fifth `10/10`, sixth `6/6`, all of `1..100000`. Squares run `176/316`: every square to `98^2 = 9604` is written and `99^2 = 9801` is not, and the largest written square is `97969 = 313^2`, written by exactly one row, `mrly_bang_d4_28662.voids.side`, whose closed form `4k^4 - 8k^3 + 8k^2 - 4k + 1` is the square of the centered square numbers.
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

Every search below is exhaustive on both sides: every window of the census sequence is indexed and every record of a local copy of [the OEIS stripped dump](https://oeis.org/stripped.gz) is walked against that index, so no sampling of offsets is involved. The dump read holds 398817 records. Under the standing caveat of [sequences](sequences.md) a dump is a snapshot, so every absence here is **Conjecture** and needs a live re-read before it is repeated.

- **The miss set is new to the OEIS only in its dense head. Conjecture.** No record carries any 4-term window of the miss set at offsets `0..416`; the first hit is at offset 417, in [A049537](https://oeis.org/A049537). Above that the miss set does hit, and the hits are near-interval records rather than identifications: 852 hits at `k = 4`, 130 at `k = 10`, 37 at `k = 15`, 15 at `k = 20`, the 20-term witnesses being [A112820](https://oeis.org/A112820) and [A118471](https://oeis.org/A118471), each a sequence that runs a block of consecutive integers through a region the census misses wholesale. The head is the informative part and is clean: the string `,269,362,422,443,` appears nowhere in the dump.
- **The write-once set is absent at every offset. Conjecture.** The 2897 integers written by exactly one row have no hit at any offset of any record at `k = 4, 10, 15, 20` - a cleaner absence than the miss set's, because the once set is thin where the miss set is an interval complement.
- **The champion set meets two records for exactly twelve terms. Verified.** Ascending, the twenty champions open `2, 3, 4, 6, 7, 8, 9, 12, 14, 15, 16, 18`, which is a window of [A100290](https://oeis.org/A100290) and of [A336231](https://oeis.org/A336231) and of no other record. All three part at the thirteenth: A100290 gives 21, A336231 gives 19, the census gives 20. Both records are binary-digit conditions, which is the right neighbourhood - the registry's designs are corner subsets of a parity cube - and neither is the champion set.
- **The written-per-decade run meets one record and parts at the next term. Verified.** The written counts by decade are `9, 90, 859, 5452, 4722` with 100000 itself, summing to 11133. The prefix `9, 90, 859` sits inside [A209631](https://oeis.org/A209631) alone, an exponential-transform array, which continues 6689 where the census gives 5452.
- **No recognizable family is systematically missed. Verified.** 173 records hold at least ten distinct integers of `1..100000` and lie wholly inside the miss set, the longest being [A361796](https://oeis.org/A361796) at 41 terms. At a miss density of `0.88867` a 41-term run of misses has probability about `10^-2.1`, which 398817 records make ordinary: the census excludes nothing a catalogue would recognise, it just runs out of depth.

## What is left

- Whether any integer of `1..100000` is written by no row at any depth. The 96-term reading is a lower bound and already moves at least 765 of the misses across, 269 among them; the honest frontier needs a cap the dimension-2 side grid can pay for, and that tier costs `cap^3`. **Conjecture.**
- Whether the miss set has any arithmetic characterisation at all. No modulus to 64 separates it, no growth order does, and it is closed under nothing; the only theorem on offer is the finiteness bound above. **Conjecture.**
- Whether the 953 tail families are 953 rules. The families are de-duplicated by written set and not by generating rule, so two rules with equal truncated value sets merge and 953 is a lower bound on the number of rules, never an upper one. **Conjecture** that no bounded union of named families reaches the written tail.
- Whether the multiplicity function, `a(n)` the number of rows writing `n`, is worth an entry. It is absent from the dump, but it is a function of the registry's own shape - the tier mix, the cap, the ceiling - and not of `n` alone, so it is a reading of this instrument and not a sequence of the integers. **Conjecture** that no reparametrisation of it is submittable.

## THE FIELD LADDER

A design is a corner set `C` of the parity cube `{0,1}^D`, its signature is `s_j = #{c in C : weight(c) = j}` and its weight enumerator is `W(t) = sum_j s_j t^j`. The odd-side fill is `P(n) = sum_(c in C) (n+1)^(D - weight(c)) n^weight(c) = (n+1)^D W(n/(n+1))`, the polynomial [sequences](sequences.md) counts with. This section reads that polynomial as a product of norm forms, one per irreducible factor of `W` over `Q`, and censuses the number fields those forms carry. The generator is `lab/field-ladder`, five verbs `norm`, `ladder`, `fields`, `swap`, `hunter`; the box is the origin-filled box `s_0 = 1`, `0 <= s_j <= C(D, j)`, which is `6, 32, 350, 8712, 526848` signatures at `D = 2..6` carrying `2^(2^D - 1)` oriented designs.

- **The fill is a product of norm forms, one per irreducible factor of the weight enumerator. Proved.** Write `W = cont(W) prod_i g_i^(e_i)` over `Z` with each `g_i` irreducible and primitive, `cont(W)` the content, and `m = deg W`. Substituting `t = n/(n+1)` and clearing `(n+1)^D` gives `P(n) = (n+1)^(D-m) cont(W) prod_i g_i*(n)^(e_i)` with `g*(n) = (n+1)^(deg g) g(n/(n+1))`, and `g*(n) = lc(g) prod_theta ((1 - theta) n - theta)` over the roots `theta` of `g`, the norm form of `Q(theta)` evaluated at `n(1 - theta) - theta`. The constant is the content and never the leading coefficient: at `D = 2` the signature `(1,2,0)` has `W = 1 + 2t`, `cont(W) = 1`, `lc(W) = 2` and fill `(n+1)(3n+1)`. On the origin-filled box `cont(W) = 1` always, since `W(0) = s_0 = 1`. The substitution is the Mobius map of matrix `[[1,0],[1,1]]` in `SL_2(Z)`, so `Q(theta/(1 - theta)) = Q(theta)`, and `disc(g*) = disc(g)` whenever `g(1) != 0`, which is `deg g* = deg g`, automatic for irreducible `g` of degree at least 2. The identity and its resultant form `P(n) = (n+1)^(D-m) Res_t(W(t), n - t(n+1))` are exact on all `16`, `256` and `65536` designs at `D = 2, 3, 4` over `12`, `64` and `700` signatures; the discriminant equality is checked over the `6`, `32` and `350` origin-filled signatures of those dimensions, `0` mismatches on the `2`, `25` and `343` factor slots of degree at least 2 (`norm`).
- **The bare form `P = s_D prod_theta ((1 - theta) n - theta)` needs the lift `(n+1)^(D - deg W)` on exactly half the designs. Proved.** `deg W < D` iff `s_D = 0` iff the all-odd corner is empty, and `C -> C xor {all-odd}` is a fixed-point-free involution of the designs, so the count is `2^(2^D - 1)`, which is `8` of `16`, `128` of `256` and `32768` of `65536` at `D = 2, 3, 4` (`norm`).
- **With the origin filled every rational root of `W` is `-1/k` and every linear factor of `P` is `(a n + 1)`. Proved.** `W` has nonnegative coefficients and `W(0) = s_0 = 1`, so `W(x) >= 1` for `x >= 0` and no factor of `W` has a positive real root; `W(0) = 1` makes `W` primitive, so `cont(W) = 1` and `prod_i g_i(0)^(e_i) = 1`, and each `g_i(0)` is `1` or `-1`, and `g_i(0) = -1` with positive leading coefficient forces a positive real root, so `g_i(0) = 1`. A linear factor is then `1 + k t` with `k >= 1` and `(1 + k t)* = (k+1) n + 1`. This is why the divisor tribe of [sequences](sequences.md) is the all-rational floor of the ladder and why its factors are `(a n + 1)` and never `(a n + b)` with `b > 1`. With the origin empty the law fails: signature `(0,2,1)` at `D = 2` has `W = t^2 + 2t` and `P(n) = n(3n + 2)` (`norm`).
- **The pure quadratic layer realizes exactly the imaginary quadratic fields of discriminant at least `-2D(D-1)`, and no others. Proved.** The pure signature `(1, b, c)` has `W = 1 + b t + c t^2` with `0 <= b <= C(D,1) = D` and `0 <= c <= C(D,2)`, fill `P(n) = (n+1)^(D-2)((1 + b + c) n^2 + (b + 2) n + 1)` and discriminant `b^2 - 4c` on both sides. Every `d = 0` or `1 mod 4` with `-4C(D,2) <= d < 0` occurs, by `b = 0`, `c = -d/4` and by `b = 1`, `c = (1-d)/4`, and no smaller value occurs since `b^2 - 4c >= -4C(D,2)`; those are `D(D-1)` values, the count the discriminant staircase row of [DISCOVERIES](DISCOVERIES.md) states. Passing to fields divides out the conductor, and the fields realized are exactly those of fundamental discriminant `d_K` with `abs(d_K) <= 4C(D,2) = 2D(D-1)`, which is `2, 5, 10, 14, 21` fields at `D = 2..6` and not `D(D-1) = 2, 6, 12, 20, 30` (`fields`).
- **The whole census adds no further imaginary quadratic field at `D <= 6`. Verified.** Over every signature of the box the imaginary quadratic field discriminants are exactly the fundamental discriminants of the window above, so the run is gapless and the first gap is the first fundamental discriminant past `2D(D-1)`, namely `7, 15, 31, 43, 67` at `D = 2..6` (`fields`).
- **The `D(D-1)` count law counts orders, and under the field reading the same layer counts `2, 5, 10, 14, 21`. Proved.** The two counts are the same statement read twice: `D(D-1)` counts the values `b^2 - 4c` that are `0` or `1 mod 4` in `[-4C(D,2), -1]`, which are discriminants of quadratic orders and not all of them fundamental, `-12 = -3 * 2^2` being the first that is not; dividing out the conductor leaves `2, 5, 10, 14, 21` fields at `D = 2..6`. The two boxes must not be read against each other either: this section sweeps the `526848` origin-filled signatures at `D = 6` and reaches `-60`, while the discriminant staircase row sweeps the `1053696` signatures with `s_0` free (`lab/fill-polynomials`) and reaches `-63`, `-160` and `-899`; the deepest, `-899`, is carried by the origin-empty signature `(0,0,15,1,15,0,0)`, whose `W = t^2 (15t^2 + t + 15)` has no constant term and so falls outside the box of this section (`fields`).
- **The real quadratic side is where the census beats the pure layer. Verified.** At `D = 6` the pure layer gives `5, 8, 12, 13, 17, 21, 24, 28` and the census adds `29` and `33`, both from signatures whose weight enumerator has degree above 2, before its first gap at `37` (`fields`).

| `D` | signatures | `Q` | quadratic | cubic | quartic | quintic | sextic |
|---|---|---|---|---|---|---|---|
| 2 | 6 | 4 | 2 | | | | |
| 3 | 32 | 7 | 13 | 12 | | | |
| 4 | 350 | 12 | 62 | 130 | 146 | | |
| 5 | 8712 | 19 | 266 | 955 | 3522 | 3950 | |
| 6 | 526848 | 30 | 1173 | 7305 | 45292 | 222437 | 250611 |

- **The ladder is a census by the top degree of the irreducible factors of `W`. Verified.** The table counts signatures, exhaustively at `D = 2..6`, the `D = 6` row over `526848` signatures carrying `2^63` oriented designs. Counted by oriented design rather than by signature the `D = 4` row reads `504` rational, `6884` quadratic, `13241` cubic and `12139` quartic of `32768` (`ladder`).

| degree, signature | `D = 3` | `D = 4` | `D = 5` | `D = 6` |
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

- **Each run is an initial segment of the table of smallest field discriminants, and the table above is where it stops. Verified.** Each cell is the smallest field discriminant of that degree and signature the box misses; a `past` cell means the run covers the whole table read from the LMFDB, 100 entries for the four large classes and 20 for the rest, and the gap is beyond it; `empty` means no factor of that class occurs at all. The runs are printed by the generator: at `D = 6` the cubic `(1,1)` run `23, 31, 44, 59, ..., 815` and the quintic `(1,2)` run `1609, 1649, 1777, ..., 12752` are 100 long and the quintic `(3,1)` run `4511, ..., 13883` is 20 long (`fields`).
- **No field signature is excluded by the sign condition. Proved.** No irreducible factor of `W` has a positive real root, so every real root of every factor is negative, and this excludes no field: for a field `K` with generator `gamma`, the element `theta = -1/(gamma + N)` with `N` above every real conjugate of `gamma` generates `K`, has all real conjugates negative, and has `1/theta` an algebraic integer, so its primitive minimal polynomial has positive leading coefficient, constant term 1 and no positive real root, which are exactly the two conditions a factor of `W` satisfies.
- **The totally real classes are the sparse side of the ladder. Verified.** What the census shows is a delay, not an exclusion: signature `(3,0)` first occurs at `D = 5` with the single field `49`, `(4,0)` at `D = 6` with the single field `725`, and `(5,0)` does not occur at `D <= 6`, where the smallest totally real quintic field is `14641` (`fields`).

| degree | `D = 3` | `D = 4` | `D = 5` | `D = 6` | first miss at `D = 6` |
|---|---|---|---|---|---|
| 2 | 7 | 12 | 23 | 35 | `37` at `(2,0)` |
| 3 | 31 | 44 | 76 | 307 | `316` at `(3,0)` |
| 4 | none | 189 | 697 | 1107 | `1125` at `(4,0)` |
| 5 | none | none | 5753 | at least 12752 | past the table |

- **The box `0 <= s_j <= C(D, j)` reaches every number field of degree `d` up to a bound `B(d, D)`. Verified.** `B` is the largest bound with every field of degree `d` and absolute discriminant at most `B` reached. The merge over signatures is by field and not by absolute value: `8` is the discriminant of two fields, `-8` and `+8`, and at `D = 3` only `-8` is reached, which is why `B(2,3)` is `7` and not `11`; a discriminant the table lists twice in one signature, `576`, `1008`, `1040` and `1088` below `B(4,6)`, is credited only when the box carries two non-isomorphic factors at it, and each of those four does. `none` means the smallest field of that degree is already missed, and `at least 12752` means the run passes the last table entry. The box height is `max_j C(D, j)`, which is `3, 6, 10, 20` at `D = 3..6`, and the bounds grow far faster than the height (`hunter`).
The box is not chosen for a search, it is forced by the geometry: the corner counts of the parity cube give exactly `0 <= s_j <= C(D, j)` with `s_0 = 1`. That a bounded-height search reaches every field of small discriminant is the classical mechanism behind the tables this section runs against: [Hunter 1957](REFS.md) puts a generator of a quintic field of discriminant `D` at `abs(sum rho_i) <= 2` and `5 (sum abs(rho_i)^2)^4 <= 8 abs(D)`, [Pohst 1982](REFS.md) turns a bound of that kind into the computation of the minimum discriminants of sixth degree fields, and the complete lists themselves are the database of [Jones and Roberts 2014](REFS.md), whose minimal quintic root discriminants for `S_5` with two and with one complex place are `1609^(1/5)` and `(13 * 347)^(1/5)`, the `1609` and `4511` opening the runs above. What is new here is the box, the fractal reading and the fill and void involution, not the search.

- **Every number field appears at some finite `D`, and its discriminants arrive in order. Conjecture.** The evidence is the table above, `B(2, 6) = 35`, `B(3, 6) = 307`, `B(4, 6) = 1107` and `B(5, 6)` at least `12752` (`hunter`), each a gapless initial run against the LMFDB tables; the two proved necessary conditions on a factor, constant term 1 and no positive real root, are met by a generator of every field, and only the shape of the box stands between the census and completeness.

| `D` | `P+ V+` | `P+ V-` | `P- V+` | `P- V-` | designs |
|---|---|---|---|---|---|
| 3 | 17 | 4 | 67 | 40 | 128 |
| 4 | 413 | 91 | 4994 | 27270 | 32768 |

- **The complement on the parity cube does not swap the two tribes, and the news is the count. Refuted.** With `V(n) = (2n+1)^D - P(n) = n Q(n)`, `P+` means `P` splits into linear factors over `Q` and `V+` means `Q` does. The swap clause forbids the `P+ V+` cell alone, and that cell holds `17` of `128` origin-filled oriented designs at `D = 3` and `413` of `32768` at `D = 4`; the smallest witness is the `D = 3` design on corners `000` and `001`, fill `(n+1)^2 (2n+1)` and void core `(2n+1)(3n+2)`. That a witness exists was already known, the self-dual design being named as an exception where the clause is stated; what is new is that the exception is `0.13` of the designs at `D = 3`. The `P- V-` cell, `40` and `27270` designs, is not forbidden by the clause and is counted here only to show the census is dominated by it, `0.83` at `D = 4` (`swap`).
- **Each field discriminant is computed and guarded, and the runs count discriminants while the bounds count fields. Verified.** The field discriminant of a factor is `nfdisc` of its reversed monic model and the field signature is `polsturm` of the factor, both from PARI; every value is then guarded against the polynomial discriminant, which must be a square multiple of it with the square root the index, and against `0` or `1 mod 4`. All `256179` distinct irreducible factors of degree 2 to 5 over `D = 2..6` pass, so no factor is unresolved and no run rests on an unchecked value. Two fields can share a discriminant, so a run counts discriminants; the bounds above are lifted to fields by `nfisisom` (`fields`, `hunter`).
- **Nine of the fields the box carries are checked on their own source page. Verified.** Each is an irreducible factor of a weight enumerator in the box, read through its reversed monic model: `x^2 + x + 1` is [LMFDB 2.0.3.1](REFS.md) at `-3`, `x^2 + 3x + 1` is [LMFDB 2.2.5.1](REFS.md) at `5`, `x^3 + x^2 + 2x + 1` is [LMFDB 3.1.23.1](REFS.md) at `-23`, `x^3 + 5x^2 + 6x + 1` is [LMFDB 3.3.49.1](REFS.md) at `49`, `x^4 + 2x^2 + 3x + 1` is [LMFDB 4.0.117.1](REFS.md) at `117`, `x^4 + 7x^3 + 13x^2 + 7x + 1` is [LMFDB 4.4.725.1](REFS.md) at `725`, `x^5 + 2x^4 + x^3 + 4x^2 + 4x + 1` is [LMFDB 5.3.4511.1](REFS.md) at `-4511`, `x^4 + x^3 + 12x^2 + 19x + 11` is [LMFDB 4.0.1225.1](REFS.md) at `1225`, and `x^5 + 2x^3 + 8x^2 + 4x + 1` is [LMFDB 5.1.4429.1](REFS.md) at `4429`. The label of a field is `degree.r1.abs(disc).index`, so `4.0.1225.1` is the totally imaginary quartic of discriminant `1225 = 5^2 7^2` and is a different field from the totally real `1125` the table above misses at `(4,0)`. Every one agrees with `nfdisc` of the model (`fields`).
- **Factor multiplicity is carried, not divided out. Verified.** A repeated factor `g^e` contributes `e` copies of its field to the ladder and one discriminant to the census, and the census of distinct factors is by polynomial, so a field with several generators inside the box is counted once per polynomial in the factor counts and once per field in the runs (`ladder`, `fields`).

## The rest of the tree

- [README](README.md) is the front door: the parity cube, the Kronecker product, and the index of every page.
- [sequences](sequences.md) is the ledger this page is the complement of: which sequences are known, against which integers are reached.
- [method](method.md) - how a claim here is produced and checked, worked through on the odd-side fill polynomial.
- [DISCOVERIES](DISCOVERIES.md) - where every line above is tagged with its witness and its refutation attempt.
- [REFS](REFS.md) - every sequence id above resolved to a canonical URL.
