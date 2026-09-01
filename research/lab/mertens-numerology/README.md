# mertens-numerology

- Computes the kernel-sum constants attached to a base-`q` digit set that omits `m` of the `q` digits, and the exponent bookkeeping they force.
- The digit symbol is `g_F(t) = sum_{d in F} e(d t)` and the Dirichlet kernel is `D_q(t) = sum_{d=0}^{q-1} e(d t)`; the one-step constant of the shifted-grid `l^1` recursion is `B_q(F) = sup_t sum_{r mod q} |g_F((t+r)/q)|`.
- Splitting `|g_F| <= |D_q| + |g_E|` over the excluded set `E` and using Parseval on the `q` shifted points gives the elementary upper bound `B_q(F) <= q PB_q(m)` with `PB_q(m) = sqrt(m) + Phi_q/q`.
- The kernel constant is `Phi_q = (4/pi) q + (2q/pi) H(ceil((q-2)/2)) + (1 - 2/pi)(q - 2) + 0.727` with the harmonic upper bound `H(n) = ln n + gamma + 1/(2n)`; it is `~ (2/pi) q ln q`, so `PB_q(m)/q -> 0` but only logarithmically.
- Prints per base `q`: the digit-mass exponent `alpha_q = log(q-m)/log q`, the exponent bookkeeping constant `c_q = log PB_q(m)/log q`, the normalized slack `delta_q = (alpha_q - 3/4 - c_q)/alpha_q`, and the yes/no test `3/4 + c_q < alpha_q`.
- The test has an equivalent constant-space form `gap_q(m) = (q-m) q^(-3/4) - PB_q(m) > 0`; both are computed and their agreement is asserted at every printed row and across `3 <= q < 20000`.
- Bases printed: `1000, 2000, 3000, 3689, 3690, 5000, 10^4, 10^5, 10^6, 10^9` at `m = 1`, then the largest `m` at each of `10^4, 10^5, 10^6` with `gap_q(m) > 0`.
- The sign change of `gap_q(1)` between `q = 3689` and `q = 3690`, the stepwise increase of `gap_q(1)` across every step of `3690..10^5`, and the three maxima `m = 6, 78, 451` are asserted in the binary and pinned in tests.

## ROUNDING

- Arithmetic is `f64`; every quantity is a smooth composition of `ln`, `sqrt` and `powf` on inputs exact in `f64`, so the relative error stays near `10^-15`.
- All printed rounding carries a directional guard of `10^-12`, seven decades above that error and seven below the fifth printed digit.
- Upper-bound columns round up: `c_q` prints `ceil((c_q + 10^-12) * 10^5) / 10^5`.
- Lower-bound columns round down: `delta_q` prints `floor((delta_q - 10^-12) * 10^5) / 10^5`, `alpha_q` the same at six digits, so `alpha_q` never prints as `1.000000`.
- Printed strings are built from the scaled integers, never by formatting the float again, so no second rounding can move a digit.
- `delta_q` is formed from the unrounded `c_q`, then rounded once; the sign test uses the unrounded values.
- At `q = 3690` the margin is `~5.9 * 10^-6` in `delta_q`, below the fifth digit: the printed columns cannot display the sign there, and the certificate is the printed `gap` band and the assertion, not the rounded columns.

## RUN

- `CARGO_BUILD_JOBS=4 cargo run --release --manifest-path research/lab/Cargo.toml -p mertens-numerology`
- Milliseconds; prints only, writes nothing; the tables are emitted as markdown rows by the generator itself.
- `cargo test -p mertens-numerology` pins every table row as a rendered string, the sign change at `3689 -> 3690`, the exhaustive step sweep on `3690..10^5` with its smallest step, the three `m` maxima, the directionality of every rounding, the floor `c_q >= 0`, the harmonic bound against the exact harmonic numbers to `n = 2000`, and `Phi_q` against the exact shifted-grid kernel sum on a `4001`-point grid at `q = 50, 101, 200`.

## WITNESSES

- `c_q >= 0` at every `q`: Parseval forces `sum_r |g_F((t+r)/q)|^2 = q(q-m)` for every `t`, so the recursion never contracts and a negative `c_q` is an arithmetic error.
- The kernel bound is loose but not absurd: on the sampled grid the exact `sup_t sum_r |D_q((t+r)/q)|` sits within `20%` of `Phi_q` at `q = 50, 101, 200`, and the sup is attained near `t = 1/2`, not at the singular point.
- `gap_q(1)` is negative at `q = 3689` and positive at `q = 3690`, and steps up at all `96310` steps of `3690..10^5`; the smallest step is `>= 0.00003172`, at the top of the range, where the `q^(-3/4)` growth of the mass term is closest to the `4/(pi q)` jump of the harmonic term.
- The excluded-digit budget grows like `sqrt(q)` up to the `log` loss: the largest admissible `m` reads `6, 78, 451` at `10^4, 10^5, 10^6`, against `sqrt(q) = 100, 316, 1000`.
- `c_q` tracks `(ln ln q + ln(2/pi))/ln q` to within `0.01` at `q = 10^12`, and `delta_q` climbs toward `1/4` from below across the printed bases.
