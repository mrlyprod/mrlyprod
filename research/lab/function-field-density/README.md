# function-field-density

- Digit-restricted coprimality over `F_3[t]`: among the `2^n` polynomials of degree below `n` with every coefficient in `{0, 1}`, the ordered coprime density over all `4^n` pairs including the zero polynomial, by Moebius inversion over the exact factorisation of every polynomial, cross-checked against Euclid on every pair at `n = 1..4`.
- The marginal divisibility `pi(p)` at `n = 10`, the Euler product on those marginals through degree 5, and the exact chance of sharing no prime of degree at most 5 that the product misses.
- Mixed radix: five digit schedules at 12 positions, 4096 points each, coprime counts among unordered distinct pairs by Moebius inversion, the Euler product through 13 against the exact small-prime joint, and the per-prime local factors.
- Domain as stated on the page: `n` up to 14 for the function field, 12 digit positions for the schedules.

## RUN

- `uv run python research/lab/function-field-density/density.py`
- `uv run python research/lab/function-field-density/radix.py`

## WITNESSES

- coprime.md:194 `0.564176`, `0.563471`, `0.562833` at `n = 10, 12, 14`, gap `0.000333` from `9/16`
- coprime.md:195 `9/16 = 0.562500` and `pi(t) = 1/2`
- coprime.md:196 `0.333984`, `0.333008`, mean deviations `0.003798` and `0.001881` at degrees 2 and 3
- coprime.md:197 `0.560193` through degree 5, crossing `9/16` between degrees 3 and 4, exact `592189/1048576 = 0.564755`, difference `-0.004563`
- coprime.md:200 `gamma = 1.261860`, `gamma/2 = 0.630930`
- coprime.md:205 4096 points, `8386560` pairs, pure base 2 `0.607874`
- coprime.md:209 `4286664`, `0.511135`; :210 `5640929`, `0.672615`; :211 `5097972`; :212 `4316493`, `0.514692`; :213 `0`
- coprime.md:215 alternating gap `0.161480`
- coprime.md:216 residues `1024, 1024, 1024, 1024, 0, 0`, halves `1/2` and `1/4`, log advantage `0.223144`
- coprime.md:217 prime 5 at `-0.014253`, equal factors at 2, 7, 11, 13
- coprime.md:218 `0.520112` and `0.640940`, `+0.000012`, underprediction `0.043340`
- DISCOVERIES.md:318 the `F_3[t]` line, every number above
- DISCOVERIES.md:467 `1.261860` and `0.630930`
