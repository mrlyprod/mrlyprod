# fill-polynomials

- Fits the odd-side fill polynomial of every design at `D = 1..4` by exact Lagrange interpolation on `k = 1..D+1`, checks it out to `k = 10`, and counts the distinct polynomials against A129824.
- Walks the hyperoctahedral group to get the symmetry classes and counts how many classes their members split on the lower coefficients, and on the leading coefficient.
- Peels unit-linear factors `(q n + 1)` from every `D = 4` signature and reports the degree-4 remainders that still split over `Q` into two quadratics.
- Factors every fill polynomial at `D = 2..6` over `Q` and collects the discriminants of the quadratic factors, under two readings: every quadratic factor, and only a degree-2 peeled remainder.
- Rebuilds the seven census observables at `D = 3` by literal cell and face counting at both side families, fits each on `n = 1..4` with `n = 5, 6` held out, and calls a design locked when no observable normalizes to `c n^k prod(a_i n + 1)`.
- Checks the grid-rendered fill count against the closed-form sum over filled corners on all 256 designs at `D = 3`, odd sides `3..13`.
- Compares that lock against the origin-free path-or-edgeless predicate on all 256 designs.
- Domain run: all `2^(2^D)` designs at `D = 1..4`, all signatures at `D = 2..6` (1053696 of them at `D = 6`), all 256 designs at `D = 3` for the lock. Whole run is 90 seconds on 8 workers.

## RUN

```
uv run python research/lab/fill-polynomials/fills.py
```

## WITNESSES

- `method.md:283` - distinct polynomials `4, 12, 64, 700` at `D = 1..4`, equal to A129824.
- `method.md:285-286` - lower coefficients split `4 of 6` classes at `D = 2`, `20 of 22` at `D = 3`, `400 of 402` at `D = 4`; the leading coefficient splits `0` everywhere.
- `DISCOVERIES.md:431` - the seven `D = 4` signatures whose quartic remainder splits into two centered-polygonal quadratics: `(1,0,1,0,1)`, `(1,0,2,0,1)`, `(1,1,2,1,1)`, `(1,2,3,2,1)`, `(1,3,2,3,1)`, `(1,4,2,4,1)`, `(1,4,5,4,1)`.
- `DISCOVERIES.md:341` - the negative discriminants form a gapless run `-3..-4` at `D = 2`, `-3..-12` at `D = 3`, `-3..-24` at `D = 4`, of lengths `2, 6, 12`; the run reaches length `20` at `D = 5`, matching the page's prediction.
- `DISCOVERIES.md:341` - the depth `-63` is reached: at `D = 6` the peeled-remainder reading is gapless from `-3` to `-63`, length `31`, so the page's `D(D-1) = 30` at `D = 6` is one short; under the wider reading of every quadratic factor the `D = 6` run is length `80`, to `-160`.
- `DISCOVERIES.md:445` - `14` locked designs, `9` on the path clause `62, 94, 110, 118, 122, 124, 188, 218, 230` and `5` on the edgeless clause `128, 134, 146, 148, 150`.
- The path-or-edgeless predicate and the census lock rule agree on `256` of `256` designs, checked here and carried nowhere else.
- `method.md:275-279` - the two generators agree: the grid count matches the closed form on `256` of `256` designs at `D = 3`, over the six odd sides the census renders.
