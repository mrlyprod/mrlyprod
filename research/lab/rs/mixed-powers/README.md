# mixed-powers

- Computes, for a finite base set `D` of integers `>= 3` and a level `k >= 1`, whether every large integer is a sum of distinct terms of the multiset `M_k(D)` of powers `d^j`, `d` in `D`, `j >= k`, one copy per base: Erdos problem 124 at one level.
- The certificate is the surplus certificate of `research/notes/erdos.md`: the sums `P_n` of the `n` least terms are held as one bit array, each term folded in by one shift-or pass, and after each term `T = 1 + max{x <= S_n/2 : x not in P_n}` is read by a word scan down from `S_n/2`; when `T <= a_(n+1)` and the step `a_(m+1) <= S_m - 2T + 1` holds until the surplus `(sigma - 1) a_(m+1) >= C_k + 2T - 1` takes over, the level is complete and `F = T - 1` is the largest integer that is no sum. The step check runs over the terms up to `10^30` in exact integer arithmetic, `sigma` and `C_k` over the common denominator of the `d - 1`.
- The census enumerates the base sets in `[3, R]` minimal for `sigma > 1` and `gcd = 1`, every other such set containing one, and certifies each at `k = 1, 2, ...` until a cell passes the bit cap.
- The route verb reads, per minimal base set, the least `sum(M_1(D) below N)/N` over terms `N` in `[10^E, 10^30]`, truncated to six places, the quantity that bounds two-part partitions below `sigma = 2`.

## VERBS

- `census R K BITS THREADS`: every minimal base set in `[3, R]` at `k = 1..K` with a `2^BITS`-bit array per cell; prints `F`, the count of positive non-sums up to `F`, the index `n` and term `a_n` of the certificate, `S_n`, the term where the surplus takes over, and the depth histogram. Runtime `2.9` s at `census 10 5 32 4`, `8.7` s at `census 12 5 32 4`, on `4` threads, `512` MB per thread.
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
bash scripts/cargo.sh cargo run --release -p mixed-powers -- windows 3,4,5 1 6 30
bash scripts/cargo.sh cargo run --release -p mixed-powers -- graham 2 5 3 26
bash scripts/cargo.sh cargo run --release -p mixed-powers -- refute 27
bash scripts/cargo.sh cargo run --release -p mixed-powers -- ternary 6 200 8
bash scripts/cargo.sh cargo run --release -p mixed-powers -- band 419 475 70 8
bash scripts/cargo.sh cargo run --release -p mixed-powers -- band 419 473 100 8
```

- `bash scripts/cargo.sh cargo test --release -p mixed-powers`, `7` tests, under `1` s after the build.

## READS

- `30` minimal base sets in `[3, 10]`, all certified at `k = 1, 2, 3`, `21` at `k = 4`, `5` at `k = 5`; `103` in `[3, 12]`, all at `k = 1, 2` with `{3, 6, 9, 10, 12}` at `k = 2` from the cell verb, `F = 1473914231`.
- `F({3, 4, 5}, k)` at `k = 1..4`: `79, 77613, 4330731, 1075364603`, with `11, 1128, 45704, 1785062` positive non-sums up to `F`; `F({3, 4, 6}, k)` at `k = 1..3`: `986, 242113, 58941162`.
- The one base set in `[3, 12]` with `sigma = 1` and `gcd = 1` is `{3, 4, 7}`, outside the certificate.
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
- `ternary 6 200 8`: `g_3(n)` for Erdos problem 817 at `n = 1..6`.
- `band 419 475 70 8` and the crate test on the set: `g_3(7) <= 474`.
- `band 419 473 100 8`: no set beating `474` with its six largest elements within `100` of the top.
- The certificate, exactness, monotone, two-part, window and surplus-does-not-heal rows are proofs on `research/notes/erdos.md`.
