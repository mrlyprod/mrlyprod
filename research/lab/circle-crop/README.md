# Circle Crop

- Counts a design's filled cells inside a ball and measures the error against the density and against the design's own self-similar main term: the circle-count section of [crop](../../crop.md).
- The designs are code 7 at `D = 2` (the carpet, `mrlymath::bang::factory::create(7, 3, 2, 2, L)`) and code 23 at `D = 3` (the sponge, `create(23, 3, 3, 2, L)`).
- One convention throughout: cells are indexed `x` in `[0, 3^L)^D`, a cell counts when its centre `x + 1/2` lies in the closed Euclidean ball `|y| <= r`, corner balls sit at the lattice corner `0` and centre balls at the grid centre `3^L/2`.
- The corner sweep runs every integer radius `r = 1 .. 3^L - 1`, the centre sweep every `r = 1 .. (3^L - 1)/2`, the inscribed radius about the grid centre.
- All of it is integer arithmetic: a cell's three squared distances to the ball centre are doubled to stay integral, and `r` is read off by integer square root, so `N(r)`, the census columns and the defect `delta(r) = N(3r) - m N(r)` are exact.
- A row prints `N` the count, `in` and `cut` the filled cells fully inside and crossing, `Nfull` the same three for the whole grid, the exact `delta`, the running maxima, and the error `E` against the self-similar main term with the certified band `Elow <= |E| <= Ehigh`.
- The main term is `M(r) = lim N(3^j r) / m^j`; the row prints `E = N(r) - N(3^depth r) / m^depth` at the deepest radius the grid reaches and bands it by `cut(3^depth r) / m^depth`, which is the exact bound the crop page proves.
- Centre rows print `err_num`, the exact integer `3^(DL) N - fill * Nfull`, so the density error is a printed rational, and `rel`, its size against the main term `rho_L Nfull`.
- Every row is asserted before printing: `in <= N <= in + cut` both for the design and for the whole grid, `|delta(r)| <= cut(3r) + m cut(r)`, `|E| - band <= cut(r)`, and the crossing bound `Nfull_cut <= 3r + 5` in `D = 2`, `Nfull_cut <= pi sqrt(3) (r^2 + 1)` in `D = 3`.
- The `E` band is live only where the grid reaches a deeper radius: the `totals` line prints `rows` and `banded`, `22028` rows of which `6802` carry a band of depth `1` or more, and a `bands` line prints the split per design.
- The corner count does not depend on the level, so each level is asserted equal to its predecessor over the whole shared range before the deepest level prints its rows.
- The oracle runs at every level, on fewer sample radii as the grid grows and on none at the largest level of each design, `41` radii in all: the exact `Frac` classifier of `mrlymath::shape::census` against the integer sweep, both the filled and the whole-grid columns.
- `window` lines give the maximum per triadic window with its location, `trend` lines give the running maximum's growth over one triadic step as min, mean, max, a log-log fit and the endpoint slope, and `resonance` lines give the values at `r = 3^n` with the rank of `|delta(3^n)|` and of `cut(3^n)` inside the window `r = 3^n .. 3^(n+1) - 1`, the rank being the fraction of the window at most the value at `3^n`; past the deepest radius the defect reaches, the defect fields read `beyond_reach`.
- `transform` lines evaluate `hat mu(t) = prod_(j >= 1) P(t/3^j)/m` with the unnormalised `P(u) = sum_(d in S) e(-d . u)`, and assert both `hat mu(3t) = (P(t)/m) hat mu(t)` and `|P(t)/m| = 1` exactly on integer `t`.

## RUN

- `bash scripts/cargo.sh cargo run --release --manifest-path research/lab/Cargo.toml -p circle-crop`
- Under half a minute, peak grid `3^9` squared and `3^6` cubed; prints only, writes nothing.

## WITNESSES

- crop.md the level-free corner count: each level against the one below, `L = 6` against `L = 5` on `r = 1..242` up to `L = 9` against `L = 8` on `r = 1..6560` for the carpet, `L = 4` against `L = 3` on `r = 1..26` up to `L = 6` against `L = 5` on `r = 1..242` for the sponge.
- crop.md the crossing exponent: carpet `min = 0.871371`, `mean = 0.898741`, `max = 0.969141`, fit `0.870673`, endpoints `0.898794` over `r = 27..19682`; sponge `1.704391`, `1.733764`, `1.757218`, `1.654302`, `1.720961` over `r = 27..728`.
- crop.md the defect exponent: carpet `min = 0.220478`, `mean = 0.527490`, `max = 1.015046`, fit `0.544749`, endpoints `0.648815` over `r = 27..6560`; sponge `0.645285`, `1.056561`, `1.730726`, `1.001255`, `1.273634` over `r = 27..242`.
- crop.md the measure brackets: carpet `mu(B_1)` in `[0.750767350, 0.751113415]` at `r = 6561`, sponge in `[0.475928750, 0.485478125]` at `r = 243`.
- crop.md the powers of three: `delta` and its window maximum and rank at `r = 1, 3, ..., 2187` on the carpet and `r = 1, 3, ..., 81` on the sponge, the crossing rank out to `r = 6561` and `r = 243`.
- crop.md the tombstone off the lattice: carpet `|hat mu(t)| = 0.59332804` and `|hat mu(3t)| = 0.29666402` at `t = (1/2, 0)`, both `0.10072687` at `t = (1, 0)`.
- crop.md the crossing maximum's place: `at_over_start = 2.9671` for the carpet at `k = 5, 6, 7, 8`, the argument tripling exactly from `r = 721`; the defect's is `2.3333, 2.7407, 2.8724, 2.8628, 2.7979` at `k = 3..7`.
- crop.md the centre hole: first filled cell at `r = 122` and `r = 365` on the carpet at `L = 6, 7`, at `r = 20` and `r = 58` on the sponge at `L = 4, 5`, against the middle block's inradius `121, 364, 13, 40`.
- crop.md the centre relative error: `1` through the hole at every level, falling only to `0.059636` (carpet `L = 7`) and `0.301915` (sponge `L = 5`) at the inscribed radius.
