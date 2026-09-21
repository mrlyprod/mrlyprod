# sumset-density

- Computes the sumset `S = A + B` of the base-3 design `A = {0, 1}` and the base-4 design `B = {0, 1}`, the object of Erdos problem 125, exactly as a bit array up to `3^K`, and reads its density `D(x) = card(S meet [1, x])/x`.
- The bit array holds one bit per integer in `[0, 3^K]`; the members of `A` are set directly, and each power `4^j <= 3^K` is folded in by one shift-or pass, `S |= S << 4^j`, so `floor(K log_4 3) + 1` passes over the array build the whole sumset, `14` at `K = 17` and `18` at `K = 22`. The other order, `B` direct and the powers of `3` shifted, is the control.
- The scan reads `card(S meet [1, x])` at `x = 3^k`, `4^m`, `floor(3^k/2)`, `floor(4^m/3)` and at the centres `d = (3^k - 1)/2 + (4^m - 1)/3`, and the maximum and minimum of `D(x)` over each window `[3^k, 3^(k+1))`, each window `[2^j, 2^(j+1))` and over all of `[1, 3^K]`, each with its location. Maxima and minima are compared exactly by cross multiplication; a density prints truncated to six places; an exponent `log(card)/log(x)` prints truncated.
- The energy verb computes the additive energy `E(k, m) = card{(a, b, a', b') in A_k^2 x B_m^2 : a + b = a' + b'}` exactly, as `sum over t of 2^(z_3(t) + z_4(t))` with `t` ranging over the `3^m` integers with base-4 digits in `{-1, 0, 1}`, `z_4(t)` the number of zero digits there and `z_3(t)` the number of zero digits in the balanced ternary expansion of `t` on `k` digits, or nothing when `abs(t) > (3^k - 1)/2`; a test checks it against the histogram of the representation function at three pairs. It prints, per pair `(k, m)` with `4^m` within a factor `3` of `3^k`, the fill `card(S meet [0, d])/(d + 1)`, the energy, the flat energy `4^(k+m)/(d + 1)`, their ratio `Q` rounded up, and the Cauchy-Schwarz bound `1/Q` truncated down.

## VERBS

- `density K` builds `S` to `3^K` and prints the readings above. Runtime `0.09` s at `K = 17`, `1.3` s at `K = 20`, `13` s at `K = 22` on a `4` GB bit array; `K = 23` wants `12` GB.
- `energy K` builds `S` to `3^K` and prints the energy table for every pair with `d <= 3^K`. Runtime `0.3` s at `K = 17`, `39` s at `K = 22`.
- `control` compares the shift-or array against a double loop over `A x B` at `3^13`, the two shift orders against each other at `3^17`, the first `58` non-members against the terms of [A367090](https://oeis.org/A367090), and tests the reflection `x -> d - x` on `S meet [0, d]` at every centre below `3^17`. Runtime `0.08` s.

## RUN

```
CARGO_BUILD_JOBS=4 cargo run --release -p sumset-density -- density 17
CARGO_BUILD_JOBS=4 cargo run --release -p sumset-density -- density 22
CARGO_BUILD_JOBS=4 cargo run --release -p sumset-density -- energy 22
CARGO_BUILD_JOBS=4 cargo run --release -p sumset-density -- control
```

- `CARGO_BUILD_JOBS=4 cargo test --release -p sumset-density`, `8` tests, under `0.1` s after the build.

## READS

- `card(S meet [1, 3^22]) = 26666749554`, `D(3^22) = 0.849772`.
- `D(3^k)` at `k = 4..22`: `0.975308, 0.835390, 0.858710, 0.887517, 0.908855, 0.864959, 0.778472, 0.837186, 0.858264, 0.874244, 0.814704, 0.763392, 0.831183, 0.858962, 0.881342, 0.792352, 0.767893, 0.831191, 0.849772`.
- `D(4^m)` at `m = 3..17`: `0.968750, 0.843750, 0.860351, 0.897460, 0.859313, 0.791305, 0.837238, 0.868845, 0.806823, 0.783585, 0.838184, 0.875988, 0.785523, 0.793552, 0.845272`.
- Maximum of `D` over `[3^k, 3^(k+1))` at `k = 5..21`: `0.913419, 0.903768, 0.912038, 0.931596, 0.913781, 0.875566, 0.875469, 0.881621, 0.908274, 0.885045, 0.865671, 0.882855, 0.886340, 0.910650, 0.874408, 0.865858, 0.872186`; minimum: `0.835390, 0.852729, 0.887517, 0.858945, 0.778468, 0.778472, 0.822506, 0.858264, 0.806430, 0.763391, 0.763392, 0.815887, 0.858962, 0.785230, 0.767893, 0.767875, 0.818358`.
- Minimum of `D` over `[1, 3^22]`: `0.763391` at `x = 3^15 - 1`; the maximum is `1` at `x <= 61`.
- The reflection `x -> d - x` fixes `S meet [0, d]` at every clean centre and at no mixed centre with `d >= 449`.
- `Q(k, m)` over the `27` pairs with `6 <= k <= 22` lies in `[1.467705, 2.060586]`, the maximum at `(16, 12)`; the Cauchy-Schwarz bound `1/Q` is at least `0.485298` on every one of them, against fills between `0.834213` and `0.928391`. Fitted as `3^(eta k)` on two endpoints, `Q` grows at `eta = 0.015852` from `k = 6` to `k = 22` and at `eta = 0.001987` from `k = 11` to `k = 22`, both rounded up.

## WITNESSES

- `cobham.md` section "Object S: the base-3 design plus the base-4 design", every number there.
