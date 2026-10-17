# block-exact

- The exact grid optimum `alpha_m` of the discrete block problem of [progressions](../../../notes/progressions.md), section The door, at odd prime `m`: the largest peelable subset of `Z_m^2`, found by enumerating every peelable set of a target size or more up to affine symmetry.
- Odd prime `m` only. At odd `m` the classes are single points and a set admits the potential exactly when it peels: remove repeatedly a live point that is the midpoint of no live pair `x != z`, until empty. The affine group `AGL(2, m)`, of order `m^2 (m^2 - 1)(m^2 - m)`, preserves midpoints, and its two-point stabiliser fixes a line pointwise and is transitive off it, which the canonicity test uses. Even `m` (classes of four points) and composite `m` (no transitive point stabiliser) are not run.
- No dependency: one file, `std` only, `8` threads.

## THE METHOD

- A point `(a, b)` is the index `a m + b`; a set is a `u128` mask. The canonical set of an orbit is the one whose sorted index list is lexicographically least; it contains `0` and `1 = (0, 1)`.
- Orderly generation. The canonical set minus its largest index `t_k` is canonical: if `h T' < T'` with the first difference at `p` (so `p` in `h T'`, not in `T'`), then at `p < t_k` the set `h T` beats `T` at `p` or at `h(t_k)`, and `p >= t_k` forces `h T' = T'`. Peelability is hereditary, since removing points never creates a midpoint. So the depth-first search that adds a larger index and keeps a child only when it peels and is canonical visits every orbit of peelable sets exactly once.
- `canonical`: for every ordered pair `(t, t')` of the set, the line `t + c (t' - t)` is read as an `m`-bit pattern `F` and compared with the set's pattern on the line `{(0, c)}`. `F` lexicographically smaller: some affine map beats the set. `F` larger: no map through this pair does. `F` equal: the `m^2 - m` matrices `A` with `A (t' - t) = (0, 1)` are each applied to the off-line points and compared; a map that fixes the set is counted, which gives the automorphism group and the orbit size.
- The bound. At a node `S` the candidate list `C` holds the indices above `max S` whose addition keeps `S` peelable; by heredity every peelable descendant `T` lies in `S u C`. A line `L` meets a peelable set in a peelable subset of `Z_m`, since the midpoint of a pair on `L` lies on `L`. So `card T <= sum_L maxpeel[S n L][(S u C) n L]` over the `m` lines of a parallel class, the least over the `m + 1` classes, where `maxpeel[f][a]` is the largest peelable `P` of `Z_m` with `f <= P <= a`, a `2^m x 2^m` table. A node whose bound falls below the target is cut.
- The target is a lower bound: the best of `200` greedy builds in random order, or `--target` if larger. If nothing of the target size is found the target drops by one and the run repeats, so the enumeration of every set of the final target or more points is complete, and its largest member is `alpha_m m^2`.
- The certificate of a maximiser: the peel rounds, `g = 3^(R - 1 - r)` on the point removed in round `r` of `R`, checked on every triple `x + z = 2y`, `x != z`, in exact integers.
- The frontier: the tree is cut at the first depth with `64` canonical nodes or more, and the subtrees are shared by `8` threads; the counts do not depend on the schedule.

## THE CONTROLS

- `cargo test`: `peelable` against a reflection-based peel on `2700` random sets at `m = 3, 5, 7`; the one-dimensional table gives `(m + 1)/2` at `m = 3, 5, 7, 11`; at `m = 3` the orderly census equals the brute force over all `512` sets in orbits, in sets and in sets containing the origin (`57`); the line bound is never below the size of a peelable `10`-set at `m = 5` on `200` random sets.
- `census 3 5`: `5` and `168` orbits, `172` and `1346226` sets, `57` and `417581` containing the origin, the last two the counts of the Python depth-first enumeration in [block-search](../../py/block-search/).
- `optimum 3 5`: `4/9` and `10/25`, with `1` and `9` maximisers up to symmetry.

## RUN

- `cargo run --release --manifest-path research/progressions/lab/rs/block-exact/Cargo.toml -- optimum 3 5 7` from the repo root, `9` seconds, nearly all of it `m = 7`; the greedy build reaches `19` at `m = 7`, so `--target 19` changes nothing.
- `-- census 3 5 7 --seconds 240`: every peelable set up to symmetry and in all, `52` seconds, nearly all of it `m = 7`.
- `-- optimum 11 --target 41 --seconds 120`: aborts at the cap after `3555426` nodes, the greedy build at `37`; prints no value.
- `cargo test --release --manifest-path research/progressions/lab/rs/block-exact/Cargo.toml`, `4` tests, under a second after the build.
- Prints only, writes nothing; every check asserts.

## WITNESSES

- `m = 7`: every peelable set of `19` or more points enumerated up to affine symmetry in `317238` nodes and `315875` canonicity tests, `2.3` seconds on `8` threads: `89` maximisers of `19` points up to symmetry, `8742384` sets in all (`88` with trivial automorphism group, one with a group of order `2`), none of `20` or more. So `alpha_7 = 19/49 = 0.38775..`, `C(alpha_7) = 2 sqrt(log_2(49/19)) = 2.33818..`, `sqrt 19 = 4.3588..`.
- The lex-min maximiser, rows `b = 6..0`, columns `a = 0..6`: `...#.../..#..../....#../####.../##.##../####.../###.#..`, the points `(0,0) (0,1) (0,2) (0,3) (1,0) (1,1) (1,2) (1,3) (2,0) (2,1) (2,3) (2,5) (3,1) (3,2) (3,3) (3,6) (4,0) (4,2) (4,4)`, peeled in `11` rounds, `g = 3^(10 - r)`.
- The symmetric maximiser, automorphism group of order `2`: `...##../..#..../....#../##..##./###..../##...##/####...`, the points `(0,0) (0,1) (0,2) (0,3) (1,0) (1,1) (1,2) (1,3) (2,0) (2,2) (2,5) (3,0) (3,6) (4,3) (4,4) (4,6) (5,1) (5,3) (6,1)`, peeled in `8` rounds, `g = 3^(7 - r) <= 2187`, the shallowest peel among the `89`: the peel depths are `8, 9, 10, 11, 12, 13` rounds for `15, 25, 29, 13, 6, 1` maximisers.
- `census 7`: `6481246` orbits, `639135164466` sets, `183343240059` containing the origin, `51.9` seconds; orbits by size `0..19`: `1 1 1 3 8 31 177 937 4600 19774 73265 229593 592716 1195221 1736088 1614316 820713 181849 11863 89`. A Monte Carlo over `20000` random `k`-sets per size, run before the census, estimated `6.39e11` sets.
- `m = 3`: `alpha_3 = 4/9`, the one maximiser up to symmetry `.../##./##.` with `54` images, every point removed in round `0`. `m = 5`: `alpha_5 = 10/25`, `9` maximisers up to symmetry, `88000` sets, the lex-min `...../..#../##.#./###../###..` peeled in `3` rounds.
