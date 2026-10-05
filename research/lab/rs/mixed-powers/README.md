# mixed-powers

- Computes, for a finite base set `D` of integers `>= 3` and a level `k >= 1`, whether every large integer is a sum of distinct terms of the multiset `M_k(D)` of powers `d^j`, `d` in `D`, `j >= k`, one copy per base: Erdos problem 124 at one level.
- The certificate is the surplus certificate of `research/notes/erdos.md`: the sums `P_n` of the `n` least terms are held as one bit array, each term folded in by one shift-or pass, and after each term `T = 1 + max{x <= S_n/2 : x not in P_n}` is read by a word scan down from `S_n/2`; when `T <= a_(n+1)` and the step `a_(m+1) <= S_m - 2T + 1` holds until the surplus `(sigma - 1) a_(m+1) >= C_k + 2T - 1` takes over, the level is complete and `F = T - 1` is the largest integer that is no sum. The step check runs over the terms up to `10^30` in exact integer arithmetic, `sigma` and `C_k` over the common denominator of the `d - 1`.
- The census enumerates the base sets in `[3, R]` minimal for `sigma > 1` and `gcd = 1`, every other such set containing one, and certifies each at `k = 1, 2, ...` until a cell passes the bit cap.
- The wide certificate keeps only `P_n` meet `[0, W]` as bits and the holes of `P_n` in `(W, S_n/2]` as a sorted list capped at `2^25`, exact for `S_n < 2^63`, by the update rule proved on the note, so a level closes on `W` bits where the full array needs `S_n`; at the close it sorts every non-sum by its route, below the least term, window or pair. A capped cell returns `Cap` either because the list passes `2^25` or because the candidates of one step past `S_n` do, without saying which.
- The route verb reads, per minimal base set, the least `sum(M_1(D) below N)/N` over terms `N` in `[10^E, 10^30]`, truncated to six places, the quantity that bounds two-part partitions below `sigma = 2`.

## VERBS

- `census R K BITS THREADS`: every minimal base set in `[3, R]` at `k = 1..K` with a `2^BITS`-bit array per cell; prints `F`, the count of positive non-sums up to `F`, the index `n` and term `a_n` of the certificate, `S_n`, the term where the surplus takes over, and the depth histogram. Runtime `2.9` s at `census 10 5 32 4`, `8.7` s at `census 12 5 32 4`, on `4` threads, `512` MB per thread.
- `deep R K BITS THREADS`: the census through the wide certificate at `W = 2^BITS`, each cell also printing its non-sums by route and the route of `F`, and per level the cells whose `F` comes by each route. Runtime `21.3` s at `deep 10 6 31 4`, `2.8` s at `deep 12 3 31 4`, on `4` threads.
- `wcell D K BITS`: one cell through the wide certificate. Runtime `1.8` s at `wcell 3,4,5 5 33`, `0.9` s at `wcell 3,4,6 4 33`, `3.6` to `5.4` s at `W = 2^33` and `13.1` to `14.7` s at `W = 2^34` on the `k = 5` cells of the note, `2` GB at `W = 2^34`.
- `wcontrol BITS`: every cell of the minimal sets in `[3, 10]` at `k <= 4` that the full certificate settles within `2^30` bits, against the wide certificate at `W = 2^BITS`. Runtime `6.8` s at `wcontrol 16`: `109` agree, none differ. It samples only `k <= 4`, bases up to `10` and `S_n <= 2^30`, never the cap path or the scale of the `k = 5` cells.
- `split D K X COUNT`: the non-sums in `[X, X + COUNT)` by a meet in the middle over the terms up to `X + COUNT`, independent of the bit array. Runtime `0.3` s at `split 3,4,5 5 31333640736 64`.
- `chainless BITS`: the multiset `4, 5, 6, 7, 12, 18, 31, ...` of the note, `a_(n+2) = S_n - y_n - y_(n+1)`, up to `2^BITS`: windows, the least of `2 (S_n - a_(n+1)) - a_(n+1)`, the values of `a_(n+3) - a_(n+2) - a_(n+1)`, the chain of non-sums `a_(n+1) + y_n` and every non-sum by route. Runtime under `0.1` s at `chainless 27`.
- `forced BITS`: the multiset of `chainless` with `a_(n+2) = S_n + 2` or `S_n + 3` forced at `n >= 6`, `n = 0 mod 6, 8, 10`: open windows, chained pairs, the least of `2 (S_n - a_(n+1)) - a_(n+1)`, the non-sums below the last term and those in the last `12` term intervals. Runtime under `0.1` s at `forced 26`.
- `windowed BITS`: the multiset `V = 4, 5, 6, 7, 13, 19, 47, ...` of the note, `a_(n+2) = S_n - u_n - u_(n+1)` with `u_n` set from the chain holes `x_n = a_(n+1) + u_n`, up to `2^BITS`: sortedness, the windows and their least width over the term, chained pairs, windows holding a non-sum, the least of `12 (S_n - a_(n+1)) - a_(n+1)`, the largest `a_(n+1) - 3 a_n`, the values of `a_(n+3) - a_(n+2) - a_(n+1)` at `n = 3 mod 5` from `3`, the chain of non-sums `x_n` and every non-sum by route. Runtime under `0.1` s at `windowed 27`.
- `jumps R K BITS THREADS`: every minimal base set in `[3, R]` at `k = 1..K` through the wide certificate at `W = 2^BITS`, logging `S_n` and `T_n` at every step: checks the jump law of the note on every step, and prints per cell the rises of `T`, the birth step `b` of `F`, `F - a_(b+1)`, whether `T_b - 1 > R_b`, `R_b = S_b - a_(b+1)`, the route of `F`, the peak of `T_n/a_(n+1)` after the last base enters, and per capped cell the run of rises open at the cap; then per level the counts, the largest and least peaks, the gap from the last rise to the certificate, the cells run and the violations of the jump law and the route lemma; it stops a base set at its first capped level. Runtime `20.1` s at `jumps 10 6 31 4` on `4` threads, `7.9` GB peak resident; `30.6` s and `8.8` GB at `jumps 10 6 31 2`, so fewer threads do not lower the peak.
- `trace D K BITS`: one cell step by step: `a_n`, its base, `S_n`, `a_(n+1)`, `R_n = S_n - a_(n+1)`, `T_n`, `T_n/a_(n+1)`, and `rises` on the rows `n` with `T_(n+1) > T_n`. Runtime under `0.1` s at `trace 3,4,5 3 26`.
- `cell D K BITS`: one base set, comma separated, at one level. Runtime `1.0` s at `cell 3,4,5 4 34`, `1.3` s at `cell 3,6,9,10,12 2 34`, `2` GB.
- `control`: nine cells against a plain knapsack over every term up to `4 F`, same `F` and same count of non-sums, and a `gcd = 3` set that never certifies below `2^24`. Runtime `0.9` s.
- `set D K B`: the same base set read as a set of powers, ties counted once, by a plain knapsack to `B`; prints the largest non-sum below `B`. Runtime under `0.01` s at `set 3,5,6,9 1 20000`, which reads `649` against the multiset's `F = 22`.
- `windows D K E F`: over the terms `a_(n+1)` in `[10^E, 10^F]` of `M_K(D)`, the count of windows `a_(n+2) > S_n` and of pairs `m < n` of windows where the upper window `(S_m, a_(m+2))` meets the lower window `(S_n - a_(n+1), a_(n+2) - a_(n+1))`. Runtime under `0.01` s at `windows 3,4,5 1 6 30`: `27` of `123`, no pair.
- `graham T P Q BITS`: the set `floor(T (P/Q)^n)`, `n >= 1`, up to `2^BITS`, exact in integers; the count of windows and of windows holding a non-sum, and the largest non-sum up to `2^(BITS-1)`. Runtime under `0.1` s at `graham 2 5 3 26`: `31` windows, all holding a non-sum, largest `23559582`.
- `refute BITS`: the set `2 F_m - 1`, `m >= 2`, up to `2^BITS`, which refutes the surplus lemma L of the note: the least of `2 (S_n - a_(n+1)) - a_(n+1)`, the windows and those holding a non-sum, and the non-sums below `10^6`. Runtime under `0.1` s at `refute 27`: `36` windows, all holding a non-sum.
- `ternary N HI THREADS`: `g_3(n)` of Erdos problem 817 for `n = 1..N`, exhaustive: for each largest element in turn, a search from the top down over sets whose `3^n` ternary sums are distinct, one bit array of sums, pruned by `g_3(r)` of the run before and by `sum a_i^2 >= (9^n - 1)/8`. Runtime `5.9` s at `ternary 6 200 8`.
- `probe N TOP THREADS`: the same search at one largest element. Runtime `36` s at `probe 7 380 8`.
- `band LO HI W THREADS`: the least largest element in `[LO, HI]` of an admissible 7-set whose six largest elements lie within `W` of it, exhaustive inside that band. Runtime `2.8` s at `band 419 475 70 8`, which finds `{302, 409, 447, 459, 465, 466, 474}`, and `114` s at `band 419 473 100 8`, which finds none.
- `offsets LO HI`: the offset families `{0, 1, 3, 8, 22, 60}`, `{0, 2, 6, 9, 23, 61}`, `{0, 2, 5, 7, 21, 60}` below a largest element plus one free offset. Runtime `1.3` s.
- `route R E`: the least ratio per minimal base set in `[3, R]` over terms in `[10^E, 10^30]`. Runtime under `0.01` s.

## RUN

```
bash scripts/cargo.sh cargo run --release -p mixed-powers -- census 10 5 32 4
bash scripts/cargo.sh cargo run --release -p mixed-powers -- census 12 5 32 4
bash scripts/cargo.sh cargo run --release -p mixed-powers -- cell 3,4,5 4 34
bash scripts/cargo.sh cargo run --release -p mixed-powers -- cell 3,6,9,10,12 2 34
bash scripts/cargo.sh cargo run --release -p mixed-powers -- control
bash scripts/cargo.sh cargo run --release -p mixed-powers -- set 3,5,6,9 1 20000
bash scripts/cargo.sh cargo run --release -p mixed-powers -- route 10 12
bash scripts/cargo.sh cargo run --release -p mixed-powers -- deep 10 6 31 4
bash scripts/cargo.sh cargo run --release -p mixed-powers -- deep 12 3 31 4
bash scripts/cargo.sh cargo run --release -p mixed-powers -- wcell 3,4,8,9 5 33
bash scripts/cargo.sh cargo run --release -p mixed-powers -- wcell 3,5,7,9 5 33
bash scripts/cargo.sh cargo run --release -p mixed-powers -- wcell 3,4,9,10 5 33
bash scripts/cargo.sh cargo run --release -p mixed-powers -- wcell 3,5,6,9 5 34
bash scripts/cargo.sh cargo run --release -p mixed-powers -- wcell 3,6,8,9,10 5 34
bash scripts/cargo.sh cargo run --release -p mixed-powers -- wcell 3,4,6 4 33
bash scripts/cargo.sh cargo run --release -p mixed-powers -- wcell 3,4,6 5 34
bash scripts/cargo.sh cargo run --release -p mixed-powers -- wcell 3,5,8,9 5 34
bash scripts/cargo.sh cargo run --release -p mixed-powers -- wcell 3,4,5 6 34
bash scripts/cargo.sh cargo run --release -p mixed-powers -- wcell 3,6,8,9,12 3 33
bash scripts/cargo.sh cargo run --release -p mixed-powers -- wcell 3,6,9,10,12 3 33
bash scripts/cargo.sh cargo run --release -p mixed-powers -- wcontrol 16
bash scripts/cargo.sh cargo run --release -p mixed-powers -- split 3,4,5 5 31333640736 64
bash scripts/cargo.sh cargo run --release -p mixed-powers -- chainless 27
bash scripts/cargo.sh cargo run --release -p mixed-powers -- forced 26
bash scripts/cargo.sh cargo run --release -p mixed-powers -- windowed 27
bash scripts/cargo.sh cargo run --release -p mixed-powers -- jumps 10 6 31 4
bash scripts/cargo.sh cargo run --release -p mixed-powers -- trace 3,4,5 3 26
bash scripts/cargo.sh cargo run --release -p mixed-powers -- windows 3,4,5 1 6 30
bash scripts/cargo.sh cargo run --release -p mixed-powers -- graham 2 5 3 26
bash scripts/cargo.sh cargo run --release -p mixed-powers -- refute 27
bash scripts/cargo.sh cargo run --release -p mixed-powers -- ternary 6 200 8
bash scripts/cargo.sh cargo run --release -p mixed-powers -- band 419 475 70 8
bash scripts/cargo.sh cargo run --release -p mixed-powers -- band 419 473 100 8
```

- `bash scripts/cargo.sh cargo test --release -p mixed-powers`, `11` tests, under `1` s after the build.

## READS

- `30` minimal base sets in `[3, 10]`, all certified at `k = 1, 2, 3`, `21` at `k = 4`, `5` at `k = 5`; `103` in `[3, 12]`, all at `k = 1, 2` with `{3, 6, 9, 10, 12}` at `k = 2` from the cell verb, `F = 1473914231`.
- `F({3, 4, 5}, k)` at `k = 1..4`: `79, 77613, 4330731, 1075364603`, with `11, 1128, 45704, 1785062` positive non-sums up to `F`; `F({3, 4, 6}, k)` at `k = 1..3`: `986, 242113, 58941162`.
- The one base set in `[3, 12]` with `sigma = 1` and `gcd = 1` is `{3, 4, 7}`, outside the certificate.
- Through the wide certificate: all `30` minimal sets in `[3, 10]` at `k = 4`, `28` at `k = 5`, `{3, 5, 6, 7}` and `{4, 5, 6, 7, 9}` at `k = 6`; `F({3, 4, 5}, 5) = 31333640736` with `100335626` non-sums; `F({3, 4, 6}, 4) = 14059625243`; `101` of the `103` in `[3, 12]` at `k = 3`.
- From `k = 4` on every certified cell makes `F` by the pair route.
- `V` up to `2^27`: `37` terms, `7` windows at `n = 5, 10, ..., 35`, none chained, all holding a non-sum, least width `0.255` of the term; `33` chain values, all non-sums.
- The jump law holds on every step of the `173` cells `jumps` reaches at `W = 2^31`; `T_n/a_(n+1)` after the last base enters peaks at `1.9430` and passes `1.1727` in every certified cell with `k >= 2`; all `28` capped cells sit in one open run of rises.
- Route ratios over `[10^12, 10^30]` for the `30` minimal base sets in `[3, 10]`: between `1.105953` and `1.537469`.

## WITNESSES

- `research/claims/erdos.md`, every row citing this study:
- `route 10 12`: the two-part obstruction at twelve digits.
- `census 10 5 32 4`: cofinite at `k = 1, 2, 3` for every base set in `[3, 10]`.
- `census 12 5 32 4` and `cell 3,6,9,10,12 2 34`: cofinite at `k = 1, 2` for every base set in `[3, 12]`.
- `census 10 5 32 4`, `cell 3,4,5 4 34` and `control`: the largest non-sums of `{3, 4, 5}` and `{3, 4, 6}` and the knapsack agreement.
- `census 10 5 32 4` and `cell 3,4,5 4 34`: the Conjecture row on `{3, 4, 5}` at every level.
- `cell 3,5,6,9 1 20` and `set 3,5,6,9 1 20000`: sets and multisets differ on tie cells.
- `graham 2 5 3 26`: the seed of the incompleteness of `floor(2 (5/3)^n)`.
- `windows`: the power multisets run no window chain.
- `census 10 5 32 4`, `cell 3,4,5 4 34` and `cell 3,4,5 5 34`: the certificate for `{3, 4, 5}` at `k = 5` passes `2^34` bits.
- `refute 27`: the Refuted row on the surplus lemma L.
- `chainless 27` and its crate test: the Refuted row on the chain-excluded lemma.
- `forced 26`: the six forced-window variants of the note, none with a non-sum in its last `12` term intervals.
- `windowed 27` and its crate test: the Refuted row on the windowed lemma.
- `jumps 10 6 31 4`: the jump law on every step of the `173` cells it reaches, the route of `F` by its birth step, the peaks of `T_n/a_(n+1)` and the open runs at the cut.
- `deep 10 6 31 4`, the `wcell` runs and `wcontrol 16`: cofinite at `k = 1..4` for every base set in `[3, 10]`, `28` of `30` minimal sets at `k = 5`, and the route of every `F`.
- `deep 10 6 31 4` and `split 3,4,5 5 31333640736 64`: `F({3, 4, 5}, 5)`.
- `deep 12 3 31 4`: `101` of `103` minimal sets in `[3, 12]` at `k = 3`.
- `ternary 6 200 8`: `g_3(n)` for Erdos problem 817 at `n = 1..6`.
- `band 419 475 70 8` and the crate test on the set: `g_3(7) <= 474`.
- `band 419 473 100 8`: no set beating `474` with its six largest elements within `100` of the top.
- The certificate, exactness, monotone, wide certificate, two-part, window, two-route, one-inequality, surplus-does-not-heal, chain, birth, jump and route-of-`F` rows are proofs on `research/notes/erdos.md`.
