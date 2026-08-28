# Complex Dimensions

- The complex dimensions of the four one-base designs, base 3 `{0,2}`, base 5 `{0,2,4}`, base 15 `{0,4,10,14}` and base 15 `{0,2,4,10,12,14}`: the 81 predicted poles `s = d + 2 pi i m/ln(n)`, `m = -40..40`, checked against `1 - k n^(-s)`, the numerator `D(s)` at each, and `D(1) = 1 - k/n`.
- The two-ratio control with ratios `1/3` and `1/5`: its `d`, its 21 Moran roots in `Re [-3,3]`, `Im [-40,40]` by Newton from a grid, the winding number of `1 - 3^(-s) - 5^(-s)` over that box, and the worst offset of the roots from each candidate spacing.
- Composition: alternating bases multiplies into the product base, checked as integers and as 256 exact intervals.
- The box count `N(eps)` of each cover, `g(u) = ln N(e^-u) - d u` on `u in [ln(1/0.03), ln(1e6)]` at 3000 points, its Blackman periodogram peak on a direct DFT grid, and the variance explained by folding `u` modulo `ln 3`, `ln 5`, `ln 15` in 40 bins, for the four designs, the two-ratio control and a Thue-Morse aperiodic control.
- The inner tube `V(eps)` in closed form, `M(eps) = eps^(d-1) V(eps)` to `u = 60`, its swing per window, the periodicity defect `M(eps) = M(eps/p)`, the decay of the two-ratio swing, and the Cantor limit profile `2^(1-d) (t^(d-1) + t^d)` against the measured tube.

## RUN

- `uv run python research/lab/complex-dimensions/complex_dimensions.py`
- About 15 seconds; prints only, writes nothing.

## WITNESSES

- dimensions.md:14-15 and dimensions.md:302 name two scratch passes; this study is the one pass that replaces both.
- dimensions.md:47-51 all 81 poles at `m = -40..40` kill the denominator to `5e-14`, minimum `|D(s)| = 0.500000` for the Cantor design, `D(1) = 1 - k/n` for all four.
- dimensions.md:62-65 `d` 0.630930, 0.682606, 0.511916, 0.661642 and `omega` 5.719202, 3.903963, 2.320188, 2.320188.
- dimensions.md:74-76 every periodogram peak within 1% of `2*pi/ln(n)`.
- dimensions.md:87-92 folding 0.192, 0.030, 0.016; 0.018, 0.509, 0.034; 0.007, 0.022, 0.667; 0.009, 0.030, 0.534; 0.082, 0.047, 0.038; 0.007, 0.012, 0.115.
- dimensions.md:106-113 the alternation equals base 15 `{0,4,10,14}` and the 256 intervals are identical.
- dimensions.md:158-164 `d = 0.518370`, 21 roots and winding number 21, real parts `-0.699926` to `0.518370`, offsets 0.43, 0.17, 0.38.
- dimensions.md:181-184 minimum `2.494975716` at `t = 0.584963`, maximum `2.583040469`, swing 3.53%, measured tube within `4.9e-9` at the minimum.
- dimensions.md:186-191 swings flat from `u = 15` to `u = 60` (`eps = 8.8e-27`), periodicity at the own base to `1e-9` or better, the two-ratio swing decaying `3.79%` to `0.42%`.
- DISCOVERIES.md:360 the poles of `1/(1 - k q^(-s))` on one vertical line of period `2 pi/ln(q)`.
