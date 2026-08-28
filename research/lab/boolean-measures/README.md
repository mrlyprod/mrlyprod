# Boolean Measures

- Computes seven Boolean complexity measures, `s`, `bs`, `C`, `dt`, `deg`, `dnf`, `cnf`, on every hyperoctahedral class of designs at `D = 3` (22 classes) and `D = 4` (402 classes), from the truth-table definitions.
- Diffs the result cell by cell against `measures_d3.csv` and `measures_d4.csv`, which travel as data; fits the fill polynomial from rendered odd-side grids and compares it with the closed form.
- Groups the catalog by genus, `GF(2)` degree, popcount and fill polynomial, and reports which keys determine which measures, with the smallest witness pair at `D = 4`.
- Checks the pin family, the inequality web, total influence in exact rationals at `D = 1..4`, and the `(s, bs, C)` profile of every design at `D = 4` plus seeded samples at `D = 5`.

## RUN

- `uv run python research/lab/boolean-measures/measures.py`
- Domain: every design at `D = 1..4`; at `D = 5` a seeded sample of 50000 uniform designs, plus 20000 uniform and 20000 thinned designs; about three seconds.

## WITNESSES

- complexity.md:61 286 cells at `D = 3` and 5226 at `D = 4`, zero mismatches.
- complexity.md:66-87 the `D = 3` table of 22 classes, printed row by row.
- complexity.md:90-91 genus `iso 10, axis 2, comp 10`; `GF(2)` degree histogram `-1: 1, 0: 1, 1: 3, 2: 9, 3: 8`.
- complexity.md:94-96 the three classes `mrly_007`, `mrly_031`, `mrly_023` with `bs = 2`, `dt = 3`.
- complexity.md:104-118 the pin family, six measures equal `r`, `dnf = 1`, `cnf = r`, at `D = 3` and `D = 4`.
- complexity.md:141-150 fill polynomial 21 of 22 at `D = 3`, one collision `4k^3 - 4k^2 + k`; at `D = 4` 183 of 402, 94 shared, 92 groups, 81 split, 279 measure-splits.
- complexity.md:154-164 the witness rows `mrly_00027` and `mrly_00281`, six of seven measures split, their ANFs, one of 14 size-two groups splitting `bs`.
- complexity.md:194-209 `C = bs` on all 424 classes and on 50000 samples at `D = 5`; `mrly_07128` the sole `s != bs` class, orbit 24; 216 samples at `s = 3, bs = 4`; `deg = s^2` at `mrly_00855` and `mrly_01911`.
- complexity.md:248 `P[s(f) = D]` is `1/2, 5/8, 69/128, 18253/32768`.
- complexity.md:269-278 no design with one bichromatic edge at `D = 3`, realised counts `0, 3, 4, 5, 6, 7, 8, 9, 12`; influence mean `1/2, 1, 3/2, 2`, variance `1/4, 1/4, 3/16, 1/8`.
- README.md:50 the `D = 4` witness bullet.
