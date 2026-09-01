# rho-decoupling

- Computes `N_F(L; d)`, the number of length-`L` digit strings over a digit set `F` in base `q` whose value `sum f_j q^j` is divisible by `d`, exactly, by dynamic programming on residues mod `d` with u128 counts; the value map is injective on fixed-length strings, so this counts integers below `q^L` with padded digits in `F`.
- For each family and depth the sweep over `2 <= d <= D` with `(d, q) = 1` prints the worst normalized error `d |N_F(L;d) - k^L/d| / k^L` among `d` coprime to the digit-difference gcd, the multiplicative order of `q` at the worst `d`, the per-digit rate `worst^(1/L)`, the per-factor ceiling `gamma = max_a |sum_{f in F} e(a f / d)| / k` at that `d`, and the slack against the proved bound `d (1 - 8/(k^2 d^2))^L`.
- The proved bound is asserted exactly at every census cell where its hypotheses hold: `d |N - k^L/d| * (k^2 d^2)^L <= d * k^L * (k^2 d^2 - 8)^L` in big integers, no floats in the claim path.
- Families whose digits share a factor also print the unrestricted worst error, the wall where the count does not equidistribute.
- The Type I line accumulates `sum mu(d) (N_F(L;d) - k^L/d)` and its absolute-value trivial bound over squarefree `d <= D` coprime to `q` as exact fractions, printing both relative to `k^L` and their ratio.
- The decouple line probes the fixed divisor `d = 7` at the deepest level of each family: the per-digit error rate against the per-factor ceiling, across bases `3, 4, 5, 10, 100`, which is where the large-base decoupling is visible.
- Four pinned probes read single divisors of `q^t - 1` at `q = 100` (`d = 101, 9999, 3367, 999999`, the first inside the sweep range, the rest beyond it): each line prints the rate `(|N - k^L/d|/k^L)^(1/L)` and the orbit-mean damping factor `mean = rate * d^(1/L)`, reading `0.106` at `d = q^2 - 1` against `k^(-1/2) = 0.1005` and `0.237` at `d = q^3 - 1` against `k^(-1/3) = 0.216`.
- Census families: the three two-digit sets at `q = 3`; `{0,1,2}` at `q = 4`; `{0,1,2,3}` and `{0,2,4}` at `q = 5`; one excluded digit at `q = 10` and `q = 100`; the lower half `{0..49}` and the pair `{0,1}` at `q = 100`. Depths to `L = 96`, divisor ranges to `D = 500`; floats appear only in printed readouts, truncated at forty decimal digits.

## RUN

- `CARGO_BUILD_JOBS=4 cargo run --release --manifest-path research/lab/Cargo.toml -p rho-decoupling`
- Under two seconds; prints only, writes nothing.
- `cargo test -p rho-decoupling` pins the DP against brute-force string enumeration at four bases, the residue vector total against `k^L`, the exact splitting of `N_F(L; d1 d2)` across `d1 | q`, `(d2, q) = 1` at `q = 6`, the Mobius sieve against hand values and `M(100) = 1`, and the exact bound inequality at `q = 3`, `F = {0,1}`, `L = 16`, `d <= 60`.

## WITNESSES

- mobius.md digit strings across divisors, the uniform equidistribution bound `|N_F(L;d) - k^L/d| <= k^L (1 - 8/(k^2 d^2))^L` for `(d, q) = 1` with `d` coprime to every digit difference: asserted exactly at every census cell.
- mobius.md digit strings across divisors, the worst divisors are the pinned ones: at every family's deepest level the sweep argmax `d` has `ord_d(q) <= 8`, so `d | q^t - 1` with `t <= 8`, e.g. `d = 164` at `q = 3`, `d = 143` at `q = 10`, `d = 101, 303` at `q = 100`; shallow levels can stray (`d = 199`, `ord = 99`, at `q = 10`, `L = 6`).
- mobius.md digit strings across divisors, the decoupling: at `d = 7` the per-digit error rate falls `0.49, 0.33, 0.25, 0.11, 0.017` along `q = 3, 4, 5, 10, 100` with one digit excluded, but stays `0.50` for `F = {0,1}` at `q = 100`: the gain is carried by the digit count `k`, not the base alone.
- mobius.md digit strings across divisors, the fixed-`k` wall: for `F = {0,1}` at `q = 100` the worst normalized error over `d <= 500` still reads `7.2` at `L = 96`, decaying by factor `0.9929` per digit, pinned at `d = 303 | q^2 - 1`.
