# Digit Uniform Bound

- One bound on the digit transform of every one-missing-digit set in a base at once: it names the base and forgets the digit, so what it proves is a statement about `q` and not a list of bases.
- The target is the `l^1` exponent `alpha_1` of a one-missing-digit set against the threshold `1/4`, where the pair route turns a Mobius asymptotic on.

## THE DOMINATION

- The set is `F = {0..q-1}` less `{a0}`, `k = q - 1`, and its normalised transform is `hat F(t) = (1/(q-1)) Sum_(a != a0) e(a t)`.
- The full residue sum is the Dirichlet kernel up to a phase: `Sum_(a<q) e(a t) = e(((q-1)/2) t) D_q(t)` with `D_q(t) = sin(q pi t)/sin(pi t)`.
- Subtracting the excluded term and taking absolute values, `|hat F(t)| = |e(((q-1)/2) t) D_q(t) - e(a0 t)|/(q-1) <= (|D_q(t)| + 1)/(q - 1) =: u_q(t)`, for every `t` and every `a0`.
- The right side names no digit, so `G(w) = sup_(cell w) |hat F| <= sup_(cell w) u_q` dominates all `q` excluded digits at once, and `|hat F| <= 1` gives the free sharpening `u_q -> min(1, u_q)`.
- The cost is exact: the triangle inequality throws away the phase `e((a0 - (q-1)/2) t)`, so `u_q` is attained only where that phase is antipodal to the kernel and the bound is strict everywhere else. The uniform bound is weaker than the per-digit one at every base, by construction.

## THE RUN DECOMPOSITION

- The shift sandwich reads `alpha_1 <= (1/N) log_q max_x Sigma_N(x)` with `Sigma_N(x) = Sum_(i < q^N) Prod_(j<N) |hat F(q^j (x + i/q^N))|`, and `Sigma_N <= Sigma_N^u`, the same sum with `u_q` in place of `|hat F|`.
- Expanding `Prod_j (|D_q(q^j t)| + 1)` over subsets `E` of `{0..N-1}` and telescoping each maximal run, `D_q(t) D_q(q t) ... D_q(q^(l-1) t) = D_(q^l)(t)`, gives the exact identity `(q-1)^N Sigma_N^u(x) = Sum_E Sum_(i<q^N) Prod_(runs (s,l) of E) |D_(q^l)(q^s (x + i/q^N))|`.
- Every term is a Lebesgue sum of a Dirichlet kernel over an arithmetic progression of points. Peeling the run of largest `s` first, whose factor depends on the fewest low digits of `i`, bounds each term by `q^N Prod_r lambda_(l_r)` with `lambda_l = L_(q^l)/q^l` and `L_M = max_x Sum_(j<M) |D_M(x + j/M)|`.
- The peel is checked term by term: every one of the `2^N - 1` run terms sits under `q^N Prod_r lambda_(l_r)` and `max_x Sigma_N^u` sits under `a_N (q/(q-1))^N`, at `q = 3, 4` to `N = 4` and `q = 5, 7, 10` to `N = 3`, slack about `12%`.
- Summing over `E` by first letter gives `a_N = a_(N-1) + Sum_(l<N) lambda_l a_(N-1-l) + lambda_N`, whose growth root solves `z = 1 + Sum_(l>=1) lambda_l z^(-l)`, and `alpha_1 <= log_q(z q/(q-1))`.

## THE LEBESGUE CONSTANT

- Writing the `M` points as distances `d_j` to the nearest integer and `|sin(M pi x)| = |sin(M pi d_j)|`, the sum is `s(theta) Sum_j 1/sin(pi d_j)` at offset `theta` in `[0, 1/2]`, with pairs at `(m + theta)/M` and `(m + 1 - theta)/M`.
- With `mu = pi(m + 1/2)/M` and `delta = pi(1/2 - theta)/M`, the pair identity `s = cos(M delta)` and `csc(mu - delta) + csc(mu + delta) = 2 sin mu cos delta/(sin^2 mu - sin^2 delta)` turn the pair into `2 csc mu` times `X = cos(M delta) cos delta sin^2 mu/(sin^2 mu - sin^2 delta)`.
- For `m >= 1`, `sin delta/sin mu <= delta/mu = r/(2m+1) <= r/3` with `r = 2 M delta/pi` in `[0,1]`, and `cos(pi r/2) <= 1 - r^2` because `2 sin^2(pi r/4) >= r^2`, so `X <= (1 - r^2)/(1 - r^2/9) <= 1` and the pair is largest at `theta = 1/2`.
- For `m = 0` the pair is at most `(M/pi)(1 + pi^2/(24 M^2)) max_theta sin(pi theta)(1/theta + 1/(1 - theta)) = (4M/pi)(1 + o(1))`, which is the `theta = 1/2` value again.
- At `theta = 1/2` the sum is `2 Sum_(j < M/2) csc(x_j)`, `x_j = (2j+1) pi/(2M)`; splitting `csc = 1/x + g` with `g` convex increasing, the midpoint rule gives `Sum g(x_j) <= (M/pi) Int_0^(pi/2) g = (M/pi) log(4/pi)`, and `H_(2n) - H_n/2` bounds the reciprocal-odd sum.
- Hence `L_M <= M((2/pi) log M + gamma' ) + 2/pi` with `gamma' = (2/pi)(gamma + log(8/pi)) = 0.96252282676`, printed and used rounded up at `0.9625229`, so `lambda_l <= (2/pi) l log q + gamma' + (2/pi) q^(-l)` at every `l`, and the coarser `lambda_l <= (2/pi) l log q + c_0` with `c_0 = 0.97` needs `q^l >= 86`, since `0.97 - gamma' = 0.0074771` has to cover `(2/pi)/q^l`. The scan confirms the shape: `L_M/M - (2/pi) log M` falls from `0.965217` at `M = 4` to `0.962523`, and the maximising offset is `1/2` at every `M` tested.

## THE THRESHOLD

- With the exact hypothesis `lambda_l <= c_1 l log q + gamma' + c_1 q^(-l)`, `c_1 = 2/pi`, the root equation is `(z-1)^3 = c_1 (log q) z + gamma'(z - 1) + c_1 (z-1)^2/(q z - 1)`, and `alpha_1 < 1/4` follows from `z < q^(1/4)(1 - 1/q)`. The coarser `c_0 = 0.97` form drops the last term and is the same statement wherever `q^l >= 86`.
- Certified at 120 bits at `w = q^(1/4)(1 - 1/q)` for every `q` in `[125, 3000)`, and the chain fails at `q = 124`; the certificate first checks that the constant it uses sits above the true `gamma'`, which the earlier printed `0.9625153` did not.
- Above `q = 211` the closed-form cap `z <= 1 + sqrt(2 c_1 log q + c_0)`, which follows from `z >= 2` and `z <= 2(z-1)`, is below `q^(1/4)(1 - 1/q)` and both sides are monotone, so the tail needs no evaluation at all.
- The threshold is `q_u = 125`: `alpha_1 < 0.249980` at `q = 125`, and it falls from there at every excluded digit, with no per-base spectral computation anywhere in the chain. The `c_0 = 0.97` form of the same chain gives `126`, one base worse, so the sharper constant strengthens that statement and does not contradict it.

## ANY NUMBER OF EXCLUDED DIGITS

- Nothing in the chain used `m = 1`. With `E` any set of `m` excluded digits, `|Sum_(e in E) e(e t)| <= m` gives `|hat F(t)| <= min(1, (|D_q(t)| + m)/(q - m)) =: u_(q,m)(t)`, again naming no digit, and the free cap `|hat F| <= 1` still applies.
- The run decomposition gains one weight. Expanding `Prod_j (|D_q(q^j t)| + m)` over subsets `E` of `{0..N-1}` carries `m^(N - |E|)` on the positions outside `E`, and each maximal run of `E` telescopes to the same Dirichlet kernel, so `(q-m)^N Sigma_N^u(x) = Sum_E m^(N - |E|) Sum_(i<q^N) Prod_(runs (s,l) of E) |D_(q^l)(q^s(x + i/q^N))|`, the identity checked term for term against the direct product.
- The peel is unchanged, since it bounds one run term at a time; summing over `E` by first letter gives `a_N = m a_(N-1) + m Sum_(l<N) lambda_l a_(N-1-l) + lambda_N` and `max_x Sigma_N^u <= a_N (q/(q-m))^N`.
- The growth root solves `z = m + m Sum_(l>=1) lambda_l z^(-l)`, so with `lambda_l <= c_1 l log q + gamma' + c_1 q^(-l)` the certificate is `(z - m)(z - 1)^2 <= m(c_1 (log q) z + gamma'(z - 1) + c_1 (z-1)^2/(q z - 1))` and `alpha_1 < e` follows from `z < q^e (1 - m/q)`. At `m = 1` this is the cubic of the section above, so the general form is a check on the special one and not a rewrite of it.
- The thresholds, certified at 120 bits on `[q_u, 3000)` with the chain failing at `q_u - 1`: against `1/4` it gives `q_u(1) = 125`, `q_u(2) = 649`, `q_u(3) = 1873`; against the weaker bar `1/3` it gives `32`, `105`, `230`. So every base `q >= 649` clears `alpha_1 < 1/4` at every excluded pair, with no per-pair spectral computation anywhere. The `1/3` rung at `m = 1` lands at `q = 32`, where `q^l = 32` is below `86`, so it is exactly the rung the coarser `c_0` form may not be used on.
- The cost of uniformity grows with `m` because the phase thrown away is `m` characters rather than one: the majorant loses `m/(q-m)` where the truth loses the interference between them, and the two-digit numeric floor is far below `650` (`lab/digit-transform-norms`, verb `pairfail`).

## THE WINDOW CEILING

- The same majorant run through the window machine measures how far the uniform idea can reach at all: `U(w) = min(1, (min(q, S_1(w) S_2(w)) + 1)/(q-1))` on the cell `[w/q^n, (w+1)/q^n)`, the transfer matrix `(M y)(v) = Sum_(c<q) U(v q + c) y((v q + c) mod q^(n-1))`, and `alpha_1 <= log_q rho`.
- The supremum is enclosed in closed form, never sampled. `|sin(pi t)|` is concave on `[0,1]`, so `sup_cell 1/|sin(pi t)| = 1/sin(pi delta(w))` exactly with `delta(w) = min(w, q^n - 1 - w)/q^n`; and `sup_cell |sin(q pi t)|` is `1` when the image interval `[w, w+1]/q^(n-1)` contains a half-integer and `max` of the two endpoint values otherwise. Both are monotone closed forms, so there is no sub-scan and no Lipschitz slack.
- The ceiling: the uniform window bound clears `1/4` first at `q = 75` and at every base above it, and the bound moves by less than `0.001` from three window digits to four (`0.340029 -> 0.339085` at `q = 21`, `0.297650 -> 0.297296` at `q = 34`), so no refinement of the window rescues a base near `74`.
- So `q = 75` is the floor of the digit-uniform route as a numeric fact, `q_u = 126` is the floor of what the route proves uniformly in `q`, and the difference is slack in the run decomposition, not in the majorant.

## THE CONSISTENCY CHECK

- Where both apply the uniform bound must be weaker than the per-digit ladder, and it is at every base tested, never stronger: `0.461532` against `0.323432` at base 9 missing `0`, `0.440960` against `0.350684` at base 10 missing `5`, `0.344129` against `0.283414` at base 20 missing `6`, `0.339085` against `0.250088` at base 21 missing `0`, `0.299529` against `0.253000` at base 33 missing `15`, `0.297296` against `0.249371` at base 34 missing `16`.
- The gap closes as the base grows, `0.1381` at `q = 9` down to `0.0479` at `q = 34`, which is the discarded phase costing `1/(q-1)`.

## THE CERTIFICATE

- The threshold inequality is `mpmath.iv` at 120 bits: `q^(1/4)`, `log q` and every product and difference are intervals, and a row passes only when the lower endpoint of the margin is positive.
- The window rows are floats with every operation pushed outward by `nextafter` and every sine inflated by `2^-45` in the unsafe direction, the Perron root bounded above by Collatz-Wielandt, `M y <= mu y` componentwise giving `rho <= mu`, with the float Perron vector as the test vector and a positive floor on it.
- The power iteration accepts a root only after fifty consecutive relative moves under `1e-13` and at least three hundred steps; a shorter test admits the transient cluster of near-zero cells, whose row sums are `q`, and returns a root too large by a factor of sixteen.
- Every printed exponent is `log_q mu` rounded up and asserted against `mu < q^e` before it prints, and the certified `L_M` rows are the exact maximum over the offset, not a fit.

## RUN

- `uv run python research/lab/digit-uniform-bound/ubound.py check` in 5 seconds: the domination on nine bases and every distinct digit, the run identity against the direct product at five base-level pairs, the Lebesgue scan, the threshold and its certificate, and the consistency rows.
- `domination`, `identity`, `peel`, `lebesgue`, `threshold`, `window`, `least`, `compare` run the pieces; `least` is 6 seconds and `window` is 5.
- Prints only, writes nothing; every row that has a target asserts it, and the run ends by raising if any row is off.

## WITNESSES

- The domination `|hat F| <= (|D_q| + 1)/(q-1)` for every excluded digit, with the digit-slack column `1.000000` at `q = 3` falling to `0.010045` at `q = 200`.
- The exact run decomposition of `Sigma_N^u` into `2^N` Lebesgue sums of Dirichlet kernels at moduli `q^l`, and the peel bounding each of them.
- `L_M <= M((2/pi) log M + 0.9625153) + 2/pi`, with the maximum at the half-offset and the scanned constant falling from `0.965217` to `0.962523`.
- The threshold `q_u = 125`, certified at 120 bits on `[125, 3000)`, the chain failing at `124`; the `c_0 = 0.97` form of the same chain gives `126`.
- The window ceiling `q = 75` at three digits, and the consistency rows against the per-digit ladder.
