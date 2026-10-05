# digit-progressions

- The numbers of the digit progressions on [mobius](../../../notes/mobius.md): the digit sets `P = {u, u + r, .., u + (L-1) r}` inside `{0..base-1}`, whose transform is `|hat I_L(r t)|` with `I_L = {0..L-1}`, the uniform certificate `Lambda(L, base)` of every interval, the two sharp certificates at the half lengths, the odd digits `{1, 3, .., base-2}` at odd base and the half interval `{0..base/2 - 1}` at even base, the denominator class a step loses, and a sanity meter.
- `wall`: in 120-bit interval arithmetic, the odd digits' wall at the bars `1/5` and `1/4`, the closed form `(lambda - 1)/(fill - 1)` certified at every odd base from the wall to the point where the tail `((2/pi) log base + c_inf + 3.4/base)(base + 1)/(base - 1)` clears and failing at the base below; the half interval's wall at even base at both bars, its closed form `2 lambda_H/base` certified at the wall and failing two below, with the base from which its gap increases; `alpha_1` at three larger bases; the limit constant `3/2 + (2/pi) gamma + (1 - 2/pi)/2`; and `B(c)` at seven values of `c`, the least base from which the smoothed bound `2 + K(base)/c < base^(1/5)` holds, the wall of `K` and not of `Lambda` itself, certified at `B(c)`, failing at `B(c) - 1`, with the base from which the gap increases.
- `census`: the one-step reading `max G_L` on `201` shifts against `Lambda(L, base)` at every length `2..base-1` of every base `3..80`, on `101` at `99..106` and `41` at `1000..1003`, each asserted; the slack at bases `10006` and `10007`; the cut, the least `alpha_1` over those bases and, at each `B(c)`, the worst cell and the lengths within `0.02` of `1/5`; and at bases `3..13`, every step and length with the start at its largest, the grid sums read from the progression's own digits against the identity through `gcd(r, base^i)` and against `r_1 Lambda^i`, each asserted.
- `check`: the two new pieces of the odd digits' proof, the unsigned `aP` sum and the `aQ` telescope; on a grid of shifts at every odd base `3..401` and six larger, the closed form of `S`, the bound on `G` and `T phi <= (lambda - 1) phi`; at every even base `4..400` and six larger, `S = (base/2)(a + b)`, the bound on `G` and `T phi <= lambda_H phi`; and the grid sums from the digits of the odd digits, the half interval at even base and the even and odd digits at even base against `(3/2) r_1 lambda^i`; each asserted.
- `lost`: at ten pairs of base and step, the product `prod_(j < 200) |D_L(r base^j l/d)|/L` maximised over `l` at every `d = 2..60`, split into the pure lost class `g | r_0`, the mixed class `g e` and the rest; `1` on the pure class by construction, and asserted below `10^-3` off the class, the one test there that can fail.
- `meter`: below `10^7`, the Mertens meter, the prime sum against its main term, and the primes at the longest length holding any with `abs(1 - ratio)` times their square root, on the odd digits at base `101`, the half interval, the odd digits and the step-`3` set `{1, 4, .., 97}` at base `100`, a sanity print far below every wall and never evidence for a theorem.

## THE ROUNDING

- `wall` evaluates every closed form in `mpmath` interval arithmetic at 120 bits; lower bounds print their interval's lower end truncated down, upper bounds the upper end rounded up.
- `census`, `check` and `lost` run in float64 and assert strict inequalities with margins far above float error, except the bound on `G` at `t = 0`, tight by design; upper bounds print rounded up and lower bounds truncated down at the last printed digit.
- `meter` and the slack lines print readings with no direction.

## RUN

- `uv run python research/lab/py/digit-progressions/progressions.py wall` in under a second.
- `uv run python research/lab/py/digit-progressions/progressions.py census` in 16 seconds.
- `uv run python research/lab/py/digit-progressions/progressions.py check` in 10 seconds.
- `uv run python research/lab/py/digit-progressions/progressions.py lost` in under a second.
- `uv run python research/lab/py/digit-progressions/progressions.py meter` in 2 seconds.
- Prints only, writes nothing; every check raises if it fails.

## WITNESSES

- The odd digits: wall `94953` at `1/5`, certified at every odd base `94953..94961`, failing at `94951`, `1/5 - alpha_1 >= 1.0702 * 10^-7` there; wall `3799` at `1/4`; `alpha_1 <= 0.1993888` at `100003`, `0.1761233` at `1000003`.
- The half interval at even base: wall `61270` at `1/5`, failing at `61268`, `1/5 - alpha_1 >= 7.6701 * 10^-8` there; wall `2414` at `1/4`; `alpha_1 <= 0.1944267` at `100004`; the limit constant at most `2.04916`.
- The uniform walls: `B(9/10) = 121872`, `B(3/4) = 377657`, `B(1/2) = 4980971`, `B(1/3) = 66917470`, `B(1/4) = 417306678`, `B(1/5) = 1705120674`, `B(1/10) = 125105365918`.
- The census: reading over `Lambda` at most `0.679350` at bases `3..80` and `0.756412` overall, at base `1003`, `L = 1002`; least `alpha_1` `0.271118` at those bases; `3436` progression cells, identity to `1.892 * 10^-12`, at most `0.693572` of `r_1 Lambda^i`.
- The check: `aP` at most `0.938401` of its bound, `aQ` at most `0.189469`; odd digits `0.999857` and `0.999866`, half interval at even base `0.996854` and `0.997022`; grid sums at most `0.551913`, `0.650027`, `0.373274`.
- The lost class: `1` on the pure class by construction, between `0` and `0.9380` on the mixed class, at most `7.273 * 10^-6` elsewhere.
- Readings: `max |M|/A^(1/2)` `1.553` and `1.589`; prime sum over main term `1.0002`, `0.9969`, `0.9978`, `0.9863`, the last `1.45` times `11221^(-1/2)` for its `11221` primes of length `4`.
