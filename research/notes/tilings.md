---
title: Which designs tile
lead: Which levels of a dimension-one design tile the integers: a closed condition on the zero set of the digit transform decides T1 at every level, a horizon decides T2, the guess that the digit set must tile its residues fails at base 12, a level fails T2 first at base 18, and spectral levels and product-form limit measures agree with the tiling verdict on every case the census decides.
figure: research-tilings
slug: tilings
---

Fix a base `b >= 2` and a digit set `F` inside `{0..b-1}` with `k = |F| >= 2`. The level `n` of the dimension-one design is the finite set `A_n = {sum_{i<n} d_i b^i : d_i in F}` of `k^n` integers, and its mask is `A_n(x) = prod_{i<n} F(x^(b^i))` with `F(x) = sum_{d in F} x^d`. A finite set `A` tiles the integers when `A + C = Z` with every integer written once; it is spectral when some set of `|A|` frequencies `theta` has `A(e(theta - theta')) = 0` for every two of them, `e(t) = exp(2 pi i t)`. Every verdict on this page is read from one object, the zero set of [the digit transform](mobius.md) `hat F(t) = sum_{f in F} e(f t)` at roots of unity, and every number is printed by [lab/py/level-tiles](../lab/py/level-tiles/).

## The object

- `Z(F) = {m >= 2 : Phi_m | F(x)}`, the orders of the roots of unity where `hat F` vanishes; `hat F(a/m) = 0` with `gcd(a, m) = 1` exactly when `m` is in `Z(F)`.
- `Z_p(F) = {a >= 1 : p^a in Z(F)}` for a prime `p`, and `e_p = v_p(b)`, the exponent of `p` in the base.
- `S_A` is the set of prime powers `s` with `Phi_s | A(x)`. Coven and Meyerowitz read two conditions on it: **T1** `A(1) = prod_{s in S_A} Phi_s(1)`, and **T2** `Phi_{s_1 ... s_m} | A(x)` whenever `s_1, ..., s_m` in `S_A` are powers of distinct primes.
- Their theorems, read at source: T1 and T2 imply that `A` tiles (Theorem A); a tile satisfies T1 (Theorem B1); a tile whose size has at most two prime factors satisfies T2 (Theorem B2). Whether every tile satisfies T2 is open.
- The tiling depth of `F` at base `b` is the largest `n` such that `A_1, ..., A_n` all tile, `inf` when every level tiles.

## The index lemma

**Proved.** `Phi_m | A_n(x)` if and only if `m/gcd(m, b^i)` is in `Z(F)` for some `i < n`, and the multiplicity of `Phi_m` in `A_n` is `sum_{i<n}` of the multiplicity of `Phi_{m/gcd(m, b^i)}` in `F`. A primitive `m`-th root `zeta` makes `zeta^(b^i)` a primitive root of order `m/gcd(m, b^i)`, so `F(zeta^(b^i)) = 0` exactly when that order is in `Z(F)`; and `Phi_u(x^K)` is a product of distinct cyclotomic polynomials, so multiplicities add over the factors `F(x^(b^i))`. In Fourier terms the zero set of `hat F_n(t) = prod_{i<n} hat F(b^i t)` on `Q/Z` is the union over `i < n` of the preimages of the zero set of `hat F` under `t -> b^i t`; one step of it is Coven-Meyerowitz Lemma 1.1(7).

For a prime power the lemma reads off the level exponents: `p^a` is in `S_{A_n}` exactly when `a` is in `Z_p(F)` for `p` not dividing `b`, and when `a - e_p i` is in `Z_p(F)` for some `i < n` for `p | b`. Every census verdict below is computed from `Z(F)` through this lemma and never from a factorisation of the level. (**Verified**: PARI factors 752 masks, every digit set with `0` at bases 2 to 8, levels 1 to 3 and level 4 at bases up to 4, and the cyclotomic part equals the lemma with multiplicity on all 752; `index`.)

## The hand check

The Cantor level `{0, 2, 6, 8}`, level 2 of `{0, 2}` at base 3, has mask `(1 + x^2)(1 + x^6) = Phi_4^2 Phi_12`, so `S = {4}` and T1 reads `|A_2| = 4` against `Phi_4(1) = 2`: it does not tile. (**Verified**: PARI and the lemma agree on `Phi_4^2 Phi_12`, and no complement exists in `Z/N` for `4 | N <= 128`; `hand`.)

## The tiling levels form an initial segment

**Proved.** If `A_n` tiles then `A_m` tiles for every `m <= n`. Digits below the base give unique expansions, so `A_n = A_m + b^m A_(n-m)` with every element written once, and `A_n + C = Z` gives `A_m + (b^m A_(n-m) + C) = Z`. So the tiling levels are `1..N` for the depth `N`, and a digit set that does not tile `Z` has no tiling level at all: no non-tiling `F` has a tiling level.

## A digit set that tiles its residues tiles every level

**Proved.** If `F + E = Z/b` with every residue written once, then `A_n + E_n = Z/b^n` with `E_n = sum_{i<n} b^i E`, so every level tiles the integers with period `b^n`. The set `D = F + E` is a complete residue system mod `b`, and `D_n = sum_{i<n} b^i D` is one mod `b^n`: a residue `r` fixes `d_0` in `D` by `r mod b`, then `(r - d_0)/b` fixes `d_1`, and so on, the carry automaton of [cuts](cuts.md) run on the digit polynomial `D(x) = F(x) E(x)`, which is `1 + x + ... + x^(b-1)` mod `x^b - 1`. Each `d_i` splits uniquely as `f_i + e_i`, so the sum `A_n + E_n` is direct.

The converse is the guess, and it is false (below). What survives is a condition on `Z(F)` alone.

## T1 at every level

**Proved.** T1 holds at every level `A_n` if and only if
- T1 holds for `F`, that is `k = prod_p p^|Z_p(F)|`;
- `Z_p(F)` is empty for every prime `p` not dividing `b`;
- for every `p | b` the elements of `Z_p(F)` are pairwise incongruent mod `e_p`.

By the index lemma the exponents of `p` in `S_{A_n}` are `Z_p(F)` when `p` does not divide `b` and the union of the translates `Z_p(F) + e_p i`, `i < n`, when it does. T1 at level `n` compares `v_p(k^n) = n v_p(k)` with the size of that set. Off the base the size is constant, so `v_p(k) = |Z_p(F)| = 0`. On the base the union has at most `n |Z_p(F)|` elements and level 1 gives `v_p(k) = |Z_p(F)|`, so T1 holds for every `n` exactly when the translates are pairwise disjoint, that is when no two elements of `Z_p(F)` differ by a positive multiple of `e_p`.

**Corollaries, Proved.**
- If every level tiles then `|F|` divides `b`: T1 is necessary (Theorem B1), and the condition forces `|Z_p(F)| <= e_p`.
- The T1 depth, the largest `n` with T1 at levels `1..n`, is `0` when `F` fails T1, `1` when some prime off the base has `Z_p(F)` nonempty, and otherwise the least `(c' - c)/e_p` over pairs `c < c'` in some `Z_p(F)` with `c = c' mod e_p`, `inf` when there is none.
- When `|F|` is a prime power, T2 is vacuous at every level, so the tiling depth equals the T1 depth and every level tiles exactly when the three conditions hold.
- When the base is a prime power `p^e`, every level tiles exactly when `F` tiles `Z/b`. A factor `Phi_{p^a}` with `a > e` has degree `p^(a-1)(p-1) >= p^e > deg F`, so `Z_p(F)` lies in `1..e`, the size of `F` is a power of `p`, and the Coven-Meyerowitz complement of `F` has period `lcm(S_F)`, a divisor of `b`.

The depth can stop anywhere. `{0, 1, 4, 5}` at base 6 has mask `Phi_2 Phi_8`, `Z_2 = {1, 3}` and `e_2 = 1`, so levels 1 and 2 tile and level 3 does not; `{0, 1, 8, 9}` at base 10 has `Z_2 = {1, 4}` and stops at level 3. (**Verified**: on all 65519 digit sets with `0` at bases 2 to 16 the census asserts the T1 depth formula against the T1 depth read level by level to level 16, the tiling depth against the least of the T1 depth, the T2 depth and 8, which equals the T1 depth at every prime-power size while T2 cuts below it on 771 sets of the other sizes, and the prime-power-base law, and prints depth `2` at bases 6, 10 and 14 and depth `3` at bases 10 and 14; `census`, `witness`.)

## The guess is false

The guess: every level tiles exactly when `F` tiles `Z/b`. One direction is the residue section above; the other fails.

**Refuted**, twice.
- `F = {0, 2}` at base 6. `{0, 2}` does not tile `Z/6`, since translation by `2` has odd cycles on `Z/6`, yet `Z(F) = {4}`, `Z_2 = {2}`, `e_2 = 1`, and every level tiles by the T1 condition. The cure is to divide by the gcd: tiling is invariant under scaling (Coven-Meyerowitz Lemma 1.4), and `{0, 1}` tiles `Z/6`.
- `F = {0, 1, 8, 9}` at base 12, with gcd `1`. Its mask is `(1 + x)(1 + x^8) = Phi_2 Phi_16`, so `Z_2 = {1, 4}`, `e_2 = 2`, and `1`, `4` are incongruent mod `2`: T1 holds at every level, and `|A_n| = 4^n` is a prime power, so every level tiles. `F` itself tiles `Z` with period `16`, `{0, 1, 8, 9} + {0, 2, 4, 6} = {0..15}`. But it does not tile `Z/12`: a complement `E` of size `3` would carry `Phi_3`, `Phi_4`, `Phi_6` and `Phi_12`, since `F` vanishes at none of those orders, and T1 for `E` would read `3` against at least `Phi_3(1) Phi_4(1) = 6`. No integer multiple `vF` tiles `Z/12` either. Scaling raises both exponents, `Z_2(vF) = {1 + v_2(v), 4 + v_2(v)}` by Coven-Meyerowitz Lemma 1.4(2), and their Lemma 2.1 puts every element of `S_vF` among the prime powers dividing `12` when `vF` tiles `Z/12`; whatever happens to `1 + v_2(v)`, the power `2^(4 + v_2(v))` never divides `12`. (**Verified**: an exact-cover search over `v = 1..12`, every class mod 12, finds no complement for `vF` or for `v{0, 3, 8, 11}`; `witness`.)

**Verified.** No base below 12 has a counterexample after the gcd: over the 65519 digit sets at bases 2 to 16, 609 tile at every level and 588 tile `Z/b`; of the 21 that tile at every level without tiling `Z/b`, 19 are cured by the gcd, and the remaining two are `{0, 1, 8, 9}` and `{0, 3, 8, 11}` at base 12, both mirror-symmetric, zero sets `{2, 16}` and `{2, 6, 16}` (`census`). The levels of `{0, 1, 8, 9}` at base 12 tile with complements `{0, 2, 4, 6}` mod `16`, `{0, 2, 16, 18}` mod `64` and `{0, 2, 64, 66}` mod `256` (`witness`). The reason is visible in the T1 condition: a counterexample after the gcd needs an exponent `c` in some `Z_p(F)` above `e_p`, which the degree bound forbids at a prime-power base; a prime size `p` is cured by the gcd, since `Phi_{p^c} | F` with `|F| = p` puts `F` in one class mod `p^(c-1)`; size `4` needs `4 | b`, and 12 is the first base divisible by `4` that is not a prime power.

## T2 at every level

**Proved.** If `F` satisfies T1 and T2 and `lcm(S_F)` divides `b`, then every level satisfies T1 and T2, so every level tiles; this is the case where `F` tiles `Z/b` through its Coven-Meyerowitz complement. Each exponent `c` in `Z_p(F)` is at most `e_p`, so they are incongruent mod `e_p`. A T2 instance at level `n` takes, for distinct primes `p_j`, exponents `c_j + e_j i_j` with `c_j` in `Z_{p_j}(F)` and `i_j < n`; at `l = max i_j` the index lemma reduces it to `prod p_j^((c_j - e_j(l - i_j))_+)`, which keeps only the primes with `i_j = l` and their exponents `c_j`, a product of elements of `S_F` at distinct primes, and T2 for `F` puts it in `Z(F)`.

**Proved (the horizon).** Assume the T1 condition. Let `C_p` be the largest exponent of `p` in an element of `Z(F)`, `G` the largest `ceil(C_p/e_p)` over the `omega` primes with `Z_p(F)` nonempty. Then T2 holds at every level if and only if it holds at levels `1..1 + G(2 omega - 1)`. A T2 instance is hardest at the level `1 + max i_j`, and its witnesses are the `l <= max i_j` whose reduction lies in `Z(F)`. Every witness has `l >= i_j - G` for every `j`, since otherwise the exponent of `p_j` exceeds `C_{p_j}`. So an instance with `min i_j > G` can be lowered to `min i_j = G` with its witnesses; and if the sorted `i_j` have a gap above `2G`, every witness lies more than `G` above each lower `i_j`, the lower primes vanish at every witness, and the instance stands or falls with its upper part. The reduced instances have `max i_j <= G + 2G(omega - 1)`. (**Verified**: on the census the horizon agrees with T2 checked six levels past it on every digit set that passes the T1 condition; `census`.)

**Proved.** `F = {0, 1, 2, 6, 7, 8}` at base 18 tiles the integers and satisfies T1 at every level, yet level 2 does not tile. The mask of `F = {0, 1, 2} + {0, 6}` is `Phi_3 Phi_4 Phi_12`, so `S_F = {3, 4}`, T1 and T2 hold, and `F` tiles with period 12; `Z_2 = {2}`, `Z_3 = {1}` satisfy the T1 condition with `e_2 = 1`, `e_3 = 2`. At level 2 the exponents are `{2, 3}` for `2` and `{1, 3}` for `3`, so T2 asks for `Phi_108 = Phi_{4 * 27}`; the index lemma gives `108/gcd(108, 1) = 108` and `108/gcd(108, 18) = 6`, neither in `Z(F) = {3, 4, 12}`. `|A_2| = 36` has two prime factors, so Theorem B2 says `A_2` does not tile. So `F` tiles `Z`, T1 holds at every level, and level 2 still fails, through T2.

**Verified.** No base below 18 has such a digit set. Over the digit sets of a size with two prime factors below the base, at bases up to 30 with at most 200000 sets a cell, those that tile, pass the T1 condition, and fail T2 at a level up to 6 number `0` of size 6 at base 12, `16` of size 6 (10 up to mirror) at base 18, `0` of size 10 at base 20, `4` of size 6 at base 24 and `258` of size 6 (141 up to mirror) at base 30, every one failing at level 2; size 12 at base 24 (1352078 sets), size 14 at base 28 (20058300) and sizes 10 and 15 at base 30 exceed the cap and are skipped; at bases below 18 the only such size is `6` at base 12 (`witness`). A level-2 failure of this kind needs an exponent above `e_p` together with T2 for `F`, and the degree of `Phi_{p^c} Phi_{q^d} Phi_{p^c q^d}` must fit below `b`.

## The classification

**Proved.** For a base `b` and a digit set `F` in `{0..b-1}`:
- every level tiles only if `F` passes the T1 condition, and then `|F|` divides `b`;
- if `|F|` is a prime power, every level tiles if and only if `F` passes the T1 condition;
- if `|F|` has two prime factors, every level tiles if and only if `F` passes the T1 condition and T2 holds at levels `1..1 + G(2 omega - 1)`;
- if `|F|` has three or more prime factors, the same condition is sufficient, and its necessity is the open necessity of T2; such a size divides `b`, is below it, and so needs `b >= 60`;
- if `b` is a prime power, every level tiles if and only if `F` tiles `Z/b`.

The first two lines and the prime-power base are T1 alone; the third is Theorems A, B1 and B2 applied level by level through the index lemma and the horizon.

## Spectral levels

**Proved.** If T1 and T2 hold at every level, every level is spectral, by Łaba's Theorem 1.5(i) applied to each `A_n`; with the classification, every level is spectral whenever every level tiles and `|F|` has at most two prime factors. The spectrum is explicit: sums of `j/s` over `s` in `S_{A_n}`, `0 <= j < p` for `s` a power of `p`, as in Łaba-Wang's proof of their Theorem 1.4.

**Verified.** On the 6866 levels with `|A_n| <= 36` at bases 2 to 12 and levels 1 to 3, a level is spectral exactly when it tiles, on all 6749 levels the search decides. The search is complete for rational spectra: a spectrum through `0` has every element `theta` with `A_n(e(theta)) = 0`, so a rational one lies in `(1/L)Z` for `L = lcm Z(A_n)`, and a clique search on `Z/L` with the edges `L/gcd(d, L)` in `Z(A_n)` finds one or proves none. It is complete for all spectra when every root of `F` on the unit circle is a root of unity, since a root of `A_n` on the circle is then one too; PARI finds the other kind of root on 77 of the 4083 digit sets, and their 117 non-tiling levels with no rational spectrum stay undecided. The spectra found are re-checked numerically (`spectral`).

**Conjecture.** A level of a dimension-one design is spectral if and only if it tiles. This is Fuglede's conjecture on a structured family of integer sets; its tiling side is the classification above.

## The limit measure

The limit of the levels is the self-similar measure `mu_{b,F}`, the weak limit of the uniform measure on `b^(-n) A_n`. The guess for it: `mu_{b,F}` is spectral exactly when every level of `F` at base `b` tiles. Łaba and Wang conjectured the stronger form, that a spectral `mu` has `F` a rational multiple of a complementing set mod `b` (their Conjecture 3.1(c)); at base 12 `{0, 1, 8, 9}` is no such multiple, while its measure meets the spectral condition of Liu-Wang-Zheng Theorem 1.3, an unrefereed arXiv preprint, so the residue form fails for the measure of this witness too. Their Example 1.1, `{0, 1, 8, 9}` at base 4, quoted from Dutkay and Jorgensen, already shows a spectral measure whose digits are not distinct mod the base.

**Proved.** For consecutive digits `F = {0..N-1}`, every level tiles if and only if `N | b`: `Z(F)` is the divisors of `N` above 1, `Z_p(F) = {1..v_p(N)}`, the T1 condition reads `v_p(N) <= e_p`, and then `lcm(S_F) = N` divides `b`, so T2 holds at every level. Dai, He and Lau prove that `mu_{b,{0..N-1}}` is spectral if and only if `N | b`, so the guess holds on this family. For `{0, 2}` the T1 condition needs `b` even, and Jorgensen and Pedersen prove that at odd base `R` at most two exponentials are orthogonal in `L^2(mu)`, the base-3 Cantor measure among them, while base 4 is spectral. (**Verified**: `N | b` against the every-level verdict on all 2016 pairs `2 <= N <= b <= 64`; `measure`.)

**Verified.** For the product-form digit sets `{0..N-1} + m{0..L-1}`, T1 and T2 at every level agree with the spectral condition of Liu-Wang-Zheng Theorem 1.3, stated in an unrefereed arXiv preprint, `N | p`, `L | p` and `N | m/gcd(m, p^d)` with `d` the largest `i >= 0` such that `mL/gcd(mL, p^i)` shares a factor with `L`, on all 576422 cases with base `p <= 24`, `2 <= N, L <= 12` and `N <= m <= p^2`; no case passes the T1 condition and fails T2 with three primes, so on the box the every-level verdict is a tiling verdict (`measure`). Here digits exceed the base and a level can be a multiset; T1 and T2 force a set, since the Coven-Meyerowitz complement `B` of the proof of Theorem A gives `A_n(x) B(x) = 1 + x + ... + x^(M-1)` mod `x^M - 1`, `M = |A_n| |B|`, with non-negative coefficients (their Lemma 1.3), so every coefficient of `A_n(x)` is `0` or `1`. The zero set of a product-form mask comes from the index lemma for one power, `Z({0..N-1}) + Z({0..L-1}(x^m))`. A comparison that read only levels up to 8 would disagree on `{0, 1} + 256{0, 1}` at base 18, whose T1 depth is exactly 8: the T1 condition is what reads every level.

**Conjecture.** For an integer base `b >= 2` and a finite digit set `F` in the non-negative integers containing `0`, the measure `mu_{b,F}` is spectral if and only if every level `A_n` is a set that tiles the integers. The direction from spectral to tiling levels is a question of An and coauthors, quoted by Wu and Xiao, who settle it for equidifferent digit sets of prime size and show the converse fails for Cantor-Moran measures with varying digit sets; the self-similar converse is the half this page adds evidence for.

## The census

The census runs every digit set containing `0` with at least two digits at every base from 2 to 16, 65519 sets, and prints per base the number that tile `Z/b`, the number that tile at every level, the number that pass the T1 condition, the counterexamples to the guess before and after the gcd, the T2 horizon and the histogram of tiling depths (`census`). The totals: 588 tile `Z/b`, 609 tile at every level, 641 pass the T1 condition; the 32 that pass it and fail T2 are all of size 6 at base 12, and fail already at level 1. At a prime base only the full digit set tiles every level, since the size must divide the base. Every tiling verdict at levels up to 3 and bases up to 12, 803 of them, carries a Coven-Meyerowitz complement whose sum with the level is checked to cover every residue mod `lcm(S)` once, and 492 levels at bases up to 8 and levels up to 2 agree with an exhaustive complement search over `Z/N`, `|A| | N <= 512`, which finds a complement for every tiling verdict and none for the others; two more levels pass the search's step cap (`search`).

## Literature

- Coven and Meyerowitz, the T1 and T2 conditions, Theorems A, B1 and B2, the scaling lemma 1.4, the residue lemma 1.3 and the period lemmas 2.1 and 2.3, read at source on arXiv; the index lemma generalises their Lemma 1.1(7).
- Łaba, Theorem 1.5: T1 and T2 give a spectrum; a spectrum inside `p^(-alpha) Z` forces a prime-power size and T1.
- Łaba and Wang, Theorems 1.2 and 1.4: a compatible pair gives a spectral measure, and a complementing set mod `b` with at most two prime factors in its size gives one; Conjecture 3.1.
- Jorgensen and Pedersen, Section 6 and Theorem 6.1: base 4 spectral, odd base at most two orthogonal exponentials.
- Dai, He and Lau: `mu_{1/q, {0..N-1}}` is spectral if and only if `N | q`; Section 6 restates the Łaba-Wang conjecture.
- Dutkay, Haussermann and Lai: Hadamard triples give spectral self-affine measures in every dimension, read in the abstract.
- Liu, Wang and Zheng, an unrefereed arXiv preprint, Theorem 1.3 and Examples 1.1, 1.4, 1.5: the spectral classification of `{0..N-1} + m{0..L-1}`.
- Wu and Xiao, Section 1: the question whether a spectral Cantor-Moran measure has tiling levels, and its answer for equidifferent digit sets.

## Generators

- [lab/py/level-tiles](../lab/py/level-tiles/), one file, seven verbs: `hand`, `index`, `census`, `search`, `witness`, `spectral`, `measure`, about 90 seconds in all; its README maps every number on this page to its verb.
- The plane codes at `dim 2` and the limit measure beyond the product-form family are not decided here.
