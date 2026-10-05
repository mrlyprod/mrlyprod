# measure-spectra

- Decides, for a base `b` and a digit set `F` whose every level tiles, which route makes the limit measure `mu_{b,F}` spectral: a Hadamard triple after dividing by the gcd, the stage form of An-Lai (the `k`-stage product form of their Definition 2.3), or, at `pq` digits, the one-stage product form of their Definition 2.1, and prints explicit spectra with their orthogonality and the Jorgensen-Pedersen sum `Q`.
- The every-level verdict, the zero sets and the gcd normalisation come from [level-tiles](../level-tiles/), imported by path.
- A Hadamard triple `(b, F, L)` is found by a clique search in `Z/b` on the differences `d` with `b/gcd(b, d)` in the zero set of `F`; the stage form cuts each digit into base `b_P` windows, `b_P` the part of `b` on the primes of `|F|`, scales window `j` by `(b/b_P)^(K-j)`, and checks conditions (i) and (ii) of the definition exactly on every combination of window sets.
- Exact integer and rational arithmetic for every verdict; floats only in `Q`, a sanity check whose convergence can be slow (a near-cycle of the transfer operator with weight close to 1 slows it, as at `{0, 2, 21, 23}` base 24, rate about `0.963` a level).

## RUN

```
uv run python research/lab/py/measure-spectra/measure_spectra.py
uv run python research/lab/py/measure-spectra/measure_spectra.py hunt spectra
```

- No argument runs every verb, about 320 seconds on one core.
- `census` (80 s): every digit set with `0` at bases 2..16 of a size dividing the base, and at bases 17..30 every size `4`, `8`, `9` or with two primes dividing the base, cells of at most 130000 sets; the every-level designs split into Hadamard after the gcd, stage form, and neither; prints the skipped cells.
- `four` (18 s): every four-digit set `{0, a, b, c}` in `{0..b-1}` at bases 4..36, every-level verdict against the condition of An-He-Lai Theorem 1.7.
- `spectra` (3 s): explicit spectra, exact orthogonality against the zero set, and `1 - min Q` over 32 points in `[-1, 1]` at growing depth `n`, with two controls that are not spectra.
- `pq` (172 s; 3.5, 13, 38, 104 and 13 s): every digit set of size 6 with `0` at bases 30, 36, 42, 48, and at base 66 those with every digit below 36, prefiltered by T1 with floating evaluations at roots of unity and decided exactly by the every-level verdict; each design without a Hadamard triple after the gcd is checked for the shape of the two-prime proof (groups mod `p^(c-1) q`, the class condition mod `q^(f a)`) and for the product-form triple `(b, A ⊕ B_j, L_1 ⊕ L_2)`.
- `hunt` (1 s): every gcd-1 tree digit set of size `p^r` with an exponent above `e_p`, `p = 2` at bases 12..48 and `p = 3` at bases 36, 45, 72, asserted to tile at every level and checked for the stage form.
- `converse` (46 s; 17, 9, 2 and 1 s for the prime cells, 1.5, 13 and 1.6 s for size 6 at bases 30 and 42 and size 8 at base 24, under 1 s for the rest): the covering bound against the T1 condition, read from the prime-power orders, on every gcd-1 set with `0` of size 6 at bases 30 and 42 and size 8 at base 24; the covering bound, the least `prod_{q in Q} q^(rho_q)` over sets `Q` of primes of `b` meeting every order with primes in `b`, `rho_q` the number of classes mod `e_q` of `v_q(m)`, on every set of the prime cells and on every four-digit set with `0` and gcd `1` at bases 4..36 against the An-He-Lai condition; the five-integer clique of `{0, 1, 251, 375, 501}` at base 510, its orders found over the `b`-smooth `m` with `phi(m) <= max F`; and: every gcd-1 digit set with `0` in `{0..b-1}` of prime size `p`, `p = 5` at bases 5..42, `7` at 7..26, `11` at 11..20, `13` at 13..20; the orders `m` of `Z(F)` whose primes divide `b`, found exactly by reducing the residue counts mod `Phi_m` with an integer matrix; the every-level verdict `p` in that set; an upper bound for a set of integers incongruent mod `b` with orthogonal differences, a clique search on `Z/b` where `d` is an edge when its pattern mod each `q^(e_q)` is that of some order at its level; the count of non-prime-power orders; and two single checks, the mask of `{0, 1, 3, 5, 6}` and four such integers for `{0, 1, 9, 10}` at base 12.

## WITNESSES

- `notes/tilings.md` "The measure in the Hadamard window": 607 of the 609 every-level designs at bases 2..16 (`census`).
- "The level shift": the spectra of `{0, 1, 8, 9}` at base 12 and `{0, 1, 12, 13, 24, 25}` at base 30, and the two controls (`spectra`).
- "The measure at prime-power size": 314 of 314 tree sets and 40 of 40 census designs in stage form (`hunt`, `census`).
- "The measure at four digits or fewer": 58905 of 58905 (`four`).
- "The measure at `pq` digits": 3221, 8867, 17360, 37559 every-level designs and the 69 without a Hadamard triple, all in the product form; at base 66 below 36, 8087 and 31 (`pq`).
- "The gap": 6898 every-level designs, 6844 Hadamard after the gcd, 54 stage form, 0 neither; 14 of the stage forms at base 30 size 6 (`census`).
- "The converse at prime size": the census of 803630, 654288, 167960 and 77520 sets with bounds at most 3, the empty cyclotomic part of `{0, 1, 3, 5, 6}` and the four integers `0, 2, 8, 10` for `{0, 1, 9, 10}` at base 12 (`converse`); the five integers `0, 17, 1734, 1751, 176868` for `{0, 1, 251, 375, 501}` at base 510 with orders `6, 30, 150, 750`; the covering bound below `p` on 795499, 653504, 167959 and 77519 non-tiling sets and on no tiling one, and below 4 on all 48897 non-tiling four-digit sets and none of the 2241 tiling ones; at size 6 base 30, 8684 pass the T1 condition and 107942 fail, at size 6 base 42, 49566 and 682990, at size 8 base 24, 2188 and 242638, the bound below the size exactly on the failing ones (`converse`).
