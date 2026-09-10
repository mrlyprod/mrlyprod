# field-ladder

- Reads the odd-side fill of a base-2 design as a product of norm forms and censuses the number fields those forms carry.
- Objects: a design is a corner set `C` of `{0,1}^D`, its signature is `s_j = #{c in C : weight(c) = j}`, its weight enumerator is `W(t) = sum_j s_j t^j`, its fill is `P(n) = sum_(c in C) (n+1)^(D-weight(c)) n^weight(c) = (n+1)^D W(n/(n+1))`.
- The box is the origin-filled box `s_0 = 1`, `0 <= s_j <= C(D, j)`: `6, 32, 350, 8712, 526848` signatures at `D = 2..6`, carrying `2^(2^D - 1)` oriented designs.
- `norm` checks `P(n) = (n+1)^(D - deg W) cont(W) prod_i g_i*(n)^(e_i)` over `W = cont(W) prod_i g_i^(e_i)`, with `cont` the content and `g*(n) = (n+1)^(deg g) g(n/(n+1))`, on every design at `D = 2..4`; it checks the resultant form `P(n) = (n+1)^(D - deg W) Res_t(W(t), n - t(n+1))` on the same designs, `disc(g) = disc(g*)` over the origin-filled signatures on every irreducible factor of degree at least 2, where `g(1) != 0` holds automatically, and the linear-factor law that every linear factor of an origin-filled fill is `(a n + 1)`. The constant is the content and never the leading coefficient: `W = 1 + 2t` at `D = 2` has `lc(W) = 2` and fill `(n+1)(3n+1)`.
- `ladder` factors `W` over `Q` for every signature of the box and counts signatures and oriented designs by the top degree of the irreducible factors, exhaustively at `D = 2..6`.
- `fields` takes every distinct irreducible factor of degree 2 to 5, batches them into one PARI script, and reads the field discriminant from `nfdisc` of the reversed monic model `x^d g(1/x)`, the field signature from `polsturm` and the polynomial discriminant from `poldisc`. Every value is guarded: it is accepted only if it is `0` or `1 mod 4` and divides the polynomial discriminant with a square quotient, the square root of that quotient being the index `[O_K : Z[1/theta]]`. It prints per dimension, degree and field signature the run against the table of smallest field discriminants and the first gap. A value failing the guard would be carried as an unresolved factor and could only widen a gap, never close one; `0` of `256179` fail.
- `swap` counts oriented origin-filled designs at `D = 3, 4` by whether the fill splits into linear factors over `Q` and whether the void core does, `V(n) = (2n+1)^D - P(n) = n Q(n)`.
- `hunter` prints, per degree and dimension, the largest `B` with every field of that degree and absolute discriminant at most `B` reached by the box, and the first miss with its field signature. The merge over signatures is signature aware, so one absolute value carried by two signatures is two fields; a value the table lists twice in one signature is two fields and is credited only when the box carries two non-isomorphic factors at it, decided by PARI `nfisisom` over at most 24 carriers. The bound over discriminants rather than fields is printed beside it, and the merged table is valid only below the smallest per-signature table maximum, which is printed.
- Tables: the quadratic tables are generated here from the definition of a fundamental discriminant; the cubic, quartic and quintic tables are the smallest field discriminants by signature read from the [LMFDB](https://www.lmfdb.org/NumberField/), 100 entries each for degree 3 and 4 signatures `(1,1)`, `(3,0)`, `(0,2)`, `(2,1)` and degree 5 signature `(1,2)`, 20 entries for the others.
- PARI is required for `fields` and `hunter`: `gp` must be on `PATH`. The run is one `gp` batch per 20000 factors, and the guard above is what stands between a machine answer and a printed one.
- The census counts discriminants, not fields: two fields can share a discriminant, the first repeat inside a printed run being `576` twice in degree 4 signature `(0,2)`. `hunter` lifts its bound to fields with `nfisisom`; a run in `fields` is a run of discriminants.
- Domain: all `2^(2^D)` designs at `D = 2..4` for `norm`; all `526848` signatures at `D = 6` for `ladder`; all `256179` distinct irreducible factors of degree 2 to 5 over `D = 2..6` for `fields` and `hunter`.

## RUN

```
uv run python research/lab/field-ladder/ladder.py norm ladder fields swap hunter
```

- Verbs may be given in any combination and run in order in one process, which is how `fields` and `hunter` share one field census.
- Measured on 8 workers: `norm` 1 s, `swap` 1 s, `ladder` 25 s, `fields` and `hunter` together 35 s. The whole study is inside a wall-clock cap of 10 minutes, and no verb may cross it.

## WITNESSES

- [integers](../../integers.md), THE FIELD LADDER - the norm-form law, the lift on the half of designs with `s_D = 0`, and the linear-factor law: `norm`.
- [integers](../../integers.md), THE FIELD LADDER - the ladder by degree at `D = 2..6`: `ladder`.
- [integers](../../integers.md), THE FIELD LADDER - the quadratic layer, the imaginary run and its first gaps `7, 15, 31, 43, 67`, the real run and the two extras `29, 33` at `D = 6`: `fields`.
- [integers](../../integers.md), THE FIELD LADDER - the field-discriminant runs by degree and signature, the Hunter bounds `B` and the field-level merge behind `B(2,3) = 7`: `fields` and `hunter`.
- [integers](../../integers.md), THE FIELD LADDER - the fill and void table `17, 4, 67, 40` at `D = 3` and `413, 91, 4994, 27270` at `D = 4`: `swap`.
- [sequences](../../sequences.md), THE ODD-SIDE FILLS - the divisor tribe is the all-rational floor of the ladder: `norm` and `ladder`.
