# design-zeta

- The Dirichlet series of a digit design, `zeta_F(s) = sum_(n in S_F) n^(-s)` over the base-`q` integers whose digits all lie in `F`, continued to the whole plane and read for its zeros.
- Abscissa `alpha = log_q k`, `k = card F`; the object is Burnol 2026's `K(s)`, and the continuation is his Proposition 4.1 in a peeled form.
- Peeling: with `E_j(w)` the sum over the exactly-`j`-digit elements and `G_P(w) = sum_(j >= P) E_j(w)`, the digit map gives `(1 - k q^(-w)) G_P(w) = E_P(w) + sum_(l >= 1) binom(-w, l) q^(-w-l) gamma_l G_P(w+l)` with `gamma_l = sum_(a in F) a^l`, and `zeta_F(w) = D_(P-1)(w) + G_P(w)` for the finite Dirichlet polynomial `D_(P-1)` over `S_F` below `q^(P-1)`.
- The `l`-series has ratio `max F/q^P`, so the peel depth buys the convergence; the small quantity `G_P` is carried directly and never as a difference of two large ones.
- Truncation is proved and propagated: `abs(G_P(w)) <= k_1 (k q^(-Re w))^(P-1)/(1 - k q^(-Re w))` bounds the base of the ladder, a binomial majorant with a geometric remainder bounds the `l`-cut, and the same recursion that carries the value carries the bound, so every printed value carries the error it is good to.
- Residues come off the same ladder: at `s_(m,j) = alpha - m + 2 pi i j/log q` the factor `1 - k q^(-s)` has derivative `log q`, so the residue is the ladder numerator over `log q`.
- Zeros are counted by the argument principle on pole-free vertical strips, the phase accumulated along an adaptively refined contour that halves any segment whose phase step exceeds one radian, with the largest surviving step printed as the certificate; a box of winding one is then bisected by quadrant and polished by Muller.

## RUN

- `uv run python research/lab/design-zeta/design_zeta.py`
- `uv run python research/lab/design-zeta/design_zeta.py --full` runs the census to `Im s = 60` and adds the base-10 columns.
- Prints only, writes nothing. The default run is about three minutes, the full run about twelve.

## CONTROLS

- Base 2 on the full digit set is the whole integer line, so `zeta_F` is `zeta` there. The engine matches mpmath's `zeta` to the printed bound at `s = 2`, at the first zero, at `0.3 + 40i`, at `-1 + 2i` and at `0.5 + 100i`, gives residue exactly `1` at `alpha = 1`, and gives residue zero at `alpha + 2 pi i/log 2`, where the design family has a pole and the integers do not.
- The same census machinery run on that column finds exactly six zeros in `0.02 < Re s < 0.98`, `0.02 < Im s < 40`, one in each of the six unit boxes holding the first six zeros of `zeta`.
- The residues at `q = 3` reproduce the certified interval enclosures of `lab/burnol-residue` digit for digit on both `F = {0,1}` and `F = {0,2}`, by a route that shares no code with them.
