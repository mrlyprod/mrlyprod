# spectra-from-cuts

- Composes the two-tile claim of `notes/spectra.md` with the carry automaton of `notes/cuts.md` run at an odd base `b` instead of base 3.
- The objects: `P_b` is the digit polynomial of the solid with at most one odd coordinate, `P_b(t) = E^2 (E + 3 O)` with `E = 1 + t^2 + .. + t^(b-1)` and `O = t + t^3 + .. + t^(b-2)`; `g = 3(b-1)/2` is the target digit; the automaton is `M[c, c'] = P_b[c + g - b c']` on `abs(c) <= 1`.
- `block` recomputes the even block `[[M[0,0], M[0,1]], [M[1,0] + M[-1,0], M[1,1] + M[-1,1]]]` at every odd `b = 3..25`, checks that no carry leaves `abs(c) <= 1`, that the block equals the closed form spectra prints for the class of `b mod 4`, that `(hexagons, triangles) = ((M^n)[0,0], (M^n)[1,0] + (M^n)[-1,0])` obeys the block at levels `0..8`, and that a direct digit-sum census at heights `h_n - 1, h_n, h_n + 1`, `h_n = 3(b^n - 1)/2`, returns the same pairs at low levels.
- `forms` derives the five coefficients `P_b[g], P_b[g+1], P_b[g+b], P_b[g+b-1], P_b[g+b+1]` as polynomials in `b` within each class of `b mod 4`, by inclusion-exclusion on `((1 - x^n)/(1 - x))^3` and `(1 - x^(n-1))(1 - x^n)^2/(1 - x)^3` with `n = (b+1)/2`, prints the base from which each extraction's active terms are stable, and checks every form against the digit polynomial at odd `b = 3..101`, which covers every base below the thresholds.
- `split` takes the trace and determinant of the block in each class, and proves `rho_b > fill/b` for `b = 3 mod 4` and `rho_b < fill/b` for `b = 1 mod 4` as polynomial inequalities in `n`: real roots isolated exactly, all below the first `n` of the class, and the sign read at that `n`.
- `classes` checks that the residue-class sums of `P_b` modulo `b` are the row sums of the automaton, prints them as polynomials in `b`, and checks `P_b(w) (1 + w)^3 + 2` is divisible by `1 + w + .. + w^(b-1)`, so `P_b(w) = -2/(1 + w)^3` at every `b`-th root of unity `w != 1`.
- Exact arithmetic throughout, integers and sympy; the whole run takes under one second.

## RUN

```
uv run python research/lab/py/spectra-from-cuts/compose.py
uv run python research/lab/py/spectra-from-cuts/compose.py block
uv run python research/lab/py/spectra-from-cuts/compose.py forms
uv run python research/lab/py/spectra-from-cuts/compose.py split
uv run python research/lab/py/spectra-from-cuts/compose.py classes
```

## WITNESSES

- `spectra.md` THE CLAIM, the matrices and the closed form: verb `block`, twelve odd bases, no mismatch.
- `spectra.md` THE COMPOSITION, the five coefficients and the block in each class: verb `forms`, run inside `split`.
- `spectra.md` THE COMPOSITION, the sign of `rho_b - fill/b` in each class: verb `split`.
- `spectra.md` THE COMPOSITION, the residue-class sums and the root-of-unity value: verb `classes`.
- `cuts.md` The ladder in every dimension, the base axis at `dim = 3`: verb `block`.
