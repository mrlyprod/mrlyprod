# slice-coprimality

- Coprimality of the points of a 3D parity design on a diagonal slice `x + y + z = s`.
- Checks `A_s = Sum_{d | s, d squarefree} mu(d) N_s^(d)` at every height, the base-prime peel `N_s^(q)(n) = [000 in P][q | s] N_{s/q}(n-1)`, and the prime-slice visibility bound.
- Measures the local price of a prime aggregated over the heights it divides, against `1/p^2`, and at `p = 2` against the parity-walk formula.
- Counts the central slice `s* = 3(q^n - 1)/2`: its visible density, its per-prime locals, the independence product, its peel, the `(9, -12)` recurrence and the peel ratio.
- Designs are the parity codes carpet `{000,100,010,001}` (`k = 20` at `q = 3`), net `{110,101,011,111}` (`k = 7`), tree `{000,001}` (`k = 12`).

## RUN

- `uv run python research/lab/slice-coprimality/slices.py`
- One core, about three seconds.

## DOMAIN

- Enumerated: carpet `q = 3` to `n = 4`, `q = 4` to `n = 3`, `q = 5` to `n = 3`; net `q = 3` to `n = 7`; tree `q = 3` to `n = 7`.
- The `n = 6` aggregated locals come from a residue transfer on `(Z/p)^3` and the central slice from a meet-in-the-middle over half levels; the transfer is checked against enumeration at `n = 3`, the meet-in-the-middle against the height recursion at `n = 2..7`.
- The central count and its peel run to `n = 14` on the one-dimensional height recursion.

## WITNESSES

- `coprime.md:266,268` - zero Mobius mismatches at every height over the four design-base pairs, worst hidden count on a prime slice 3, peel exact at `q = 3, 5`, no net point with `3 | gcd` on all `7^7` points.
- `coprime.md:270` - `0.040902` against `1/25`, `0.020446` against `1/49`, `p = 11, 13` still converging.
- `coprime.md:271` - `0.2850378 = 9121792/32002048`, walk formula and count equal on the integer.
- `coprime.md:272` - tree dichotomy, zero visible on even heights and zero even gcds on odd, 1.49 million points each side.
- `coprime.md:274,275` - `0.89216, 0.89776` against `0.57143, 0.61067, 0.65218`; `R_7 = 1093`; `q = 5` reading `0.345, 0.492, 0.560`; independence product `0.64780, 0.89764, 0.55741` against measured `0.65218, 0.89776, 0.56006`.
- `coprime.md:276,277` - `6, 42, 306, 2250, 16578, 122202, 900882`, the peel `3, 27, 207, 1539, 11367, 83835, 618111`, both on the `(9, -12)` recurrence to `n = 14`, ratio `0.093070331` against `(sqrt(33) - 5)/8 = 0.0930703308`.
- `DISCOVERIES.md:35,36` - the same numbers, plus `2^1092 = 1 mod 1093^2` and the tree dichotomy carried to `n = 7`.
- `slices.md:415` - the pointer: `1/p^2` on the slice against `1/p^3` in the solid.
