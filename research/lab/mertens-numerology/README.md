# mertens-numerology

- Computes the kernel-sum constants attached to a base-`q` digit set that omits `m` of the `q` digits, and the exponent bookkeeping they force.
- The digit symbol is `g_F(t) = sum_{d in F} e(d t)` and the Dirichlet kernel is `D_q(t) = sum_{d=0}^{q-1} e(d t)`; the one-step constant of the shifted-grid `l^1` recursion is `B_q(F) = sup_t sum_{r mod q} |g_F((t+r)/q)|`.
- Splitting `|g_F| <= |D_q| + |g_E|` over the excluded set `E` and using Parseval on the `q` shifted points gives the elementary upper bound `B_q(F) <= q PB_q(m)` with `PB_q(m) = sqrt(m) + Phi_q/q`.
- The kernel constant is `Phi_q = (4/pi) q + (2q/pi) H(ceil((q-2)/2)) + (1 - 2/pi)(q - 2) + 0.727` with the harmonic upper bound `H(n) = ln n + gamma + 1/(2n)`; it is `~ (2/pi) q ln q`, so `PB_q(m)/q -> 0` but only logarithmically.
- Prints per base `q`: the digit-mass exponent `alpha_q = log(q-m)/log q`, the exponent bookkeeping constant `c_q = log PB_q(m)/log q`, the normalized slack `delta_q = (alpha_q - 3/4 - c_q)/alpha_q`, and the yes/no test `3/4 + c_q < alpha_q`.
- The test has an equivalent constant-space form `gap_q(m) = (q-m) q^(-3/4) - PB_q(m) > 0`; both are computed and their agreement is asserted at every printed row and across `3 <= q < 20000`.
- Bases printed: `1000, 2000, 3000, 3689, 3690, 5000, 10^4, 10^5, 10^6, 10^9` at `m = 1`, then the largest `m` at each of `10^4, 10^5, 10^6` with `gap_q(m) > 0`.
- The sign change of `gap_q(1)` between `q = 3689` and `q = 3690`, the stepwise increase of `gap_q(1)` across every step of `3690..10^5`, and the three maxima `m = 6, 78, 451` are asserted in the binary and pinned in tests.
- At the wall the margin is printed at ten significant digits from the cancellation-reduced form `delta_q = ln(1 + gap_q(m)/PB_q(m)) / (alpha_q ln q)`, which is algebraically identical to `(alpha_q - 3/4 - c_q)/alpha_q` but never subtracts two numbers of size `1` to reach one of size `10^-6`.
- A ladder block replaces the fixed exponent `3/4` by `b(a)`, the exponent Baker-Harman and Zhang buy from a common zero-free half plane `sigma > a` for Dirichlet L-functions, and prints the wall `q_0(a)`, the least `q >= 3` with `(q-1) q^(-b(a)) > PB_q(1)`.
- `b(a)` is carried as an exact rational and compared by cross multiplication, never in floating point: `a + 1/4`, `4/5` and `(a+1)/2` on the three ranges of the first table, `(8a - 7a^2)/(4 - 2a)` on `[1/2, 4/7]` for the second, the row printing whichever is smaller and `both` where they meet.
- The ladder also prints `Q(b)`, the monotone floor: `PB_{q+1}(1) - PB_q(1) < 1.291/(q-2)` for `q >= 40` while `(q-1) q^(-b)` gains at least `(1-b)(q+1)^(-b)` per step, so `gap_q(a, 1)` steps up at every `q` with `(1-b)(q-2)(q+1)^(-b) >= 1.291`, and that quantity increases in `q`.
- Below the floor the ladder closes the range by hand-free means: an exhaustive scan clears `3 <= q < 3690` at every rung, and on `[3690, Q(b)]` the smooth majorant `q^(1-b) - PB_q^-(1)` dominates the gap.
- The majorant's derivative `(1-b) q^(-b) - (2/pi)/(q-2) - 2(1-2/pi)/q^2` crosses zero once, from negative to positive, so the majorant has one interior minimum and its maximum on the range sits at an endpoint; both endpoint values are negative.
- The floor bound is not per-rung: at every `b` in `[3/4, 1)` the same constants give `gap_{Q(b)}(b, 1) < -1.56` and a negative majorant at both ends of `[3690, Q(b)]`, and those constants are pinned in tests alongside a `b`-grid that reproduces the bound.
- `Q(b) < q_0(a)` at every printed rung, so each wall is the least `q`, and `gap_q(a, 1)` steps up from it on without any scan.
- A ladder `m`-corollary block prints the largest `m` with `PB_q(m) < (q-m) q^(-b(a))` at `q = 10^7`, for the rungs whose wall lies below `10^7`.
- A cost-out table sets `delta_q` beside the defect exponent `m/(2(q-m) ln q)` carried by the level-`x^(alpha/2)` distribution bound for digit strings, at the `m = 1` bases from `q = 3689` on and at the three corollary maxima, with the least `q` of the scan `3690..10^5` at which the saving exceeds the defect.

## ROUNDING

- Arithmetic is `f64`; every quantity is a smooth composition of `ln`, `sqrt` and `powf` on inputs exact in `f64`, so the relative error of an undifferenced quantity stays near `10^-15`, and only `gap_q` and `delta_q` lose digits, to cancellation, by the bounded amount below.
- The five-digit columns carry a directional guard of `10^-12`, three decades above that error and seven below the fifth printed digit.
- Upper-bound columns round up: `c_q` prints `ceil((c_q + 10^-12) * 10^5) / 10^5`.
- Lower-bound columns round down: `delta_q` prints `floor((delta_q - 10^-12) * 10^5) / 10^5`, `alpha_q` the same at six digits, so `alpha_q` never prints as `1.000000`.
- Printed strings are built from the scaled integers, never by formatting the float again, so no second rounding can move a digit.
- `delta_q` is formed from the unrounded `c_q`, then rounded once; the sign test uses the unrounded values.
- Scientific rows carry a relative guard of `10^-10` instead: `sci(x, d, up)` scales `x` by `1 +/- 10^-10`, then rounds the mantissa away from or toward zero so the printed string is a true upper or lower bound on `x`.
- The guard sits five decades below the last digit of a six-digit cost-out row and one decade below the last digit of a ten-digit margin row, and above the worst cancellation loss in the file, which is the direct form of `delta_q` at `q = 3689`: terms of size `1` differencing to `-2.4 * 10^-6`, a relative loss of about `5 * 10^-11`; `gap_q(1)` there loses about `5 * 10^-12`.
- A ladder wall prints as an exact integer only when it sits below `2^53` and both neighbouring gaps exceed `1024` ulps of the terms differenced; otherwise the row prints `<=` and a scientific upper bound, which is what the certificate supports.
- At `q = 3690` the margin is `delta_q >= 5.863425182 * 10^-6`, below the fifth digit: the rounded columns cannot display the sign there, and the certificate is the margin block and the `gap` assertion, not the five-digit columns.

## RUN

- `CARGO_BUILD_JOBS=4 cargo run --release --manifest-path research/lab/Cargo.toml -p mertens-numerology`
- Milliseconds; prints only, writes nothing; the tables are emitted as markdown rows by the generator itself.
- `cargo test -p mertens-numerology` pins every table row as a rendered string, the four margin strings and their bound direction, the ten cost-out rows, the crossover `q` and its step count, the agreement of the two `delta_q` forms to `10^-9` relative, the sign change at `3689 -> 3690`, the exhaustive step sweep on `3690..10^5` with its smallest step, the three `m` maxima, the directionality of every rounding, the floor `c_q >= 0`, the harmonic bound against the exact harmonic numbers to `n = 2000`, and `Phi_q` against the exact shifted-grid kernel sum on a `4001`-point grid at `q = 50, 101, 200`.
- The ladder adds: the ten ladder rows and the six `m`-corollary rows as rendered strings, the GRH rung reading `b = 3/4` and `q_0 = 3690`, Zhang strictly below Baker-Harman at every rational of denominator `<= 200` inside `(1/2, 4/7)` and equal at both ends, `Q(b)` least and below every wall, `PB_q^-` below `PB_q` on `3 <= q < 20000`, no rung closing below `3690`, each wall below `4 * 10^6` reproduced by an exhaustive scan from `q = 3`, the `log10 q_0(a)` trend line, the constants of the general-`b` floor bound with the `b`-grid behind them, and the last gap down-step `662 -> 663` below the floor at `b = 3/4`.

## WITNESSES

- mobius.md a power saving under GRH at large base, the `l^1` floor: `c_q >= 0` at every `q`, since Parseval forces `sum_r |g_F((t+r)/q)|^2 = q(q-m)` for every `t`, so the recursion never contracts and a negative `c_q` is an arithmetic error.
- mobius.md a power saving under GRH at large base, step 3: the kernel bound is loose but not absurd, the exact `sup_t sum_r |D_q((t+r)/q)|` sitting within `20%` of `Phi_q` at `q = 50, 101, 200` on the sampled grid, with the sup attained near `t = 1/2` and not at the singular point.
- mobius.md a power saving under GRH at large base, step 5 and the wall: `gap_q(1)` is negative at `q = 3689` and positive at `q = 3690`, and steps up at all `96310` steps of `3690..10^5`, the smallest step `>= 0.00003172` at the top of the range, where the `q^(-3/4)` growth of the mass term is closest to the `4/(pi q)` jump of the harmonic term.
- mobius.md a power saving under GRH at large base, the `m`-corollary: the excluded-digit budget grows like `sqrt(q)` up to the `log` loss, the largest admissible `m` reading `6, 78, 451` at `10^4, 10^5, 10^6` against `sqrt(q) = 100, 316, 1000`.
- mobius.md a power saving under GRH at large base, the shape at large base: `c_q` tracks `(ln ln q + ln(2/pi))/ln q` to within `0.01` at `q = 10^12`, and `delta_q` climbs toward `1/4` from below across the printed bases.
- mobius.md a power saving under GRH at large base, the margin at the wall: `delta_q <= -2.395807653 * 10^-6` at `q = 3689` and `delta_q >= 5.863425182 * 10^-6` at `q = 3690`, the two `delta_q` forms agreeing to `10^-9` relative at every printed base, which is the independent check on the digits printed.
- mobius.md a power saving under GRH at large base, the rungs: the ladder is monotone in `a`, `q_0` reading `3690, 8578, 33547, 92317, 92317, 3107080, 6939524168` and then three scientific bounds, so a wider zero-free half plane costs a higher base and nothing else.
- mobius.md a power saving under GRH at large base, the input `b(a)`: the GRH rung `a = 1/2` reproduces the wall `3690` exactly and Zhang meets Baker-Harman there, so the sharper second table moves no GRH number.
- mobius.md a power saving under GRH at large base, what a weaker half plane spends first: the `m`-budget at `q = 10^7`, each rung printed with its `a`, its `b(a)` and its source, reads `1971` at `1/2`, `3/4`, both; `1002` at `13/25`, `1417/1850`, Zhang; `365` at `11/20`, `913/1160`, Zhang; `176` at `4/7`, `4/5`, both; `176` at `3/5`, `4/5`, BH; and `8` at `2/3`, `5/6`, BH.
- mobius.md a power saving under GRH at large base, the cost-out: the saving is below the defect at both `q = 3689` and `q = 3690` and above it from `q = 3692` on, the crossover two steps past the wall, the difference rising at every one of the `96310` steps of `3690..10^5` without a proof of monotonicity beyond the scan.
