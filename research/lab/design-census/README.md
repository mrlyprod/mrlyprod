# Design Census

- The three design censuses in this tree, rerun in full.
- Coprime: every design with `k >= 2` at base 2 in dimensions 2 and 3 and at base 3 in dimension 2, the Menger sponge and two base-6 samples; the measured coprime density at the deepest level inside a budget of 200000 points against the predicted `B(F) / zeta(D) * prod (1 - p^-D)^-1`, the exact base-local identity at `n = 4`, the spanning index, the split of the spanning lines by `log_q(k)` against one, and how many lines widen their gap to the prediction at some level.
- Fill: one row per cube-group orbit at bases 2 and 3 in dimensions 2 and 3, base 3 dimension 3 by the four named designs only; level-1 fill and void at sides 1..12, the fill quasi-polynomial, orbit size, popcount and three algebraic degrees, every row rendered cell by cell and checked against the closed form and the level law.
- Fill classes: the parity-rule generator against the popcount-profile closed form over every base-2 design at `D = 2, 3`, the distinct fill sequences at `D = 1..4` against `Prod (1 + C(D,k))`, the 16 published terms of A129824, and the per-dimension counts table with A000616 by class sums to `D = 6`.
- The base-2 cube group counted three ways at `D = 1..4`: orbit walk, Burnside average, class sums.

## RUN

- `CARGO_BUILD_JOBS=4 cargo run --release --manifest-path research/lab/Cargo.toml -p design-census`
- Half a minute on 8 cores; writes `sequences.csv` and `counts.csv` beside this file.

## WITNESSES

- coprime.md:29 the exact identity at `n = 4` on all 763 lines, zero failures.
- coprime.md:35 code 34376528265 at base 6 with `k = 8`, `k_2 = 3`, `k_3 = 2`, `k_6 = 1`, bracket `1/2`.
- coprime.md:101-110 the family table 11/5/3, 247/149/27, 502/365/175, 1/1/1, 2/2/2, total 763 designs, 522 spanning, zero flagged; the sponge at `n = 4` measuring 0.7719 against 0.8207; `k = 3` reaching `n = 11`.
- coprime.md:111-113 219 distinct term-vectors among the 502 base-3 designs and 175 spanning; 490 spanning lines above dimension one, 32 at exactly one, four more at index 3, none below; 426 lines whose gap to the prediction widens at some level.
- core.md:47-51 fill at `n = 3` of 8 of 9, 20 of 27 and 12 of 27, with `20^2 = 400` at level 2.
- core.md:81-86 and core.md:156-163 carpet and net share one orbit at base 2, rep code 7 in dimension 2 and 23 in dimension 3 so only the net label survives, and the six 2D classes with orbit sizes 1, 4, 4, 2, 4, 1 and degrees -1, 2, 1, 1, 2, 0.
- core.md:96-106 base-2 orbit counts 3, 6, 22, 402 agreeing three ways at `D = 1..4`.
- core.md:243-248 the census this study writes: representatives, orbit sizes, fill polynomials, three degrees, per-dimension counts.
- method.md:109-112 58 designs across bases 2 and 3 and dimensions 2 and 3, rendered cell by cell, orbit counts against Burnside, a 59-line csv.
- method.md:113-117 763 designs measured against the predicted density, 522 spanning, 241 degenerate, none flagged.
- method.md:322-330 `mrly_023` filling `4*k^3 - 3*k^2` and reading 20 at `n = 3`, `mrly_015` and `mrly_027` sharing one polynomial, 21 distinct polynomials over 22 classes and 64 over 256 designs.
- bijection.md:88-99 group orders 2, 8, 48, 384, orbit counts 22, 402, 1228158, 400507806843728, and 6 canonical designs in dimension 2, 22 in dimension 3; bijection.md:184 the base-3 `D = 3` Burnside 111618.
- sequences.md:11 and sequences.md:105-108 fill classes counted by A129824 with zero profile collisions, A000616 `3, 6, 22, 402, 1228158, 400507806843728`, all 16 published A129824 terms, 144 and 2304 generator checks with zero mismatches, and the A129824 column to `D = 8`.
