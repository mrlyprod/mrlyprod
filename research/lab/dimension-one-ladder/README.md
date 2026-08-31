# dimension-one-ladder

- Regenerates the coprime terms of the three base-three mask designs, the moment ladder of the ternary gasket, and the occupied-ray census.
- Terms: `A(n)` for the carpet, sponge and Vicsek plus by Mobius over the quotient box with a subset-lattice inner count, checked against brute force and the kept `terms/` files.
- Ladder: exact carry matrices for the even moments `E_2K(G_a)`, a direct convolution as second builder, factored characteristic polynomials, the strongly connected component of the zero state and the energy cap, a Sturm bracket for `lambda_10`, the rungs `beta_0^(2K)` to `2K = 20`, the Fourier identities and master bound on every prime `5..199`, the dyadic tail and the order-10 main range.
- Census: every ray of the level-`n` gasket by exact enumeration, per-octave counts, `Z`, and the multiplier distribution at `n = 13`.

## RUN

- `CARGO_BUILD_JOBS=4 cargo run --release --manifest-path research/lab/Cargo.toml -p dimension-one-ladder` runs the carpet and plus to `n = 18`, the sponge to `n = 16`, the census at `n = 13..16`, about two minutes on eight cores.
- `... -p dimension-one-ladder -- terms carpet 20` or `census 13 14` picks one job; the kept rows reach `n = 20` for the two planar designs and `n = 18` for the sponge, and print from `terms/` labelled stored only.
- `uv run python research/lab/dimension-one-ladder/ladder.py` runs the ladder in about a second.

## WITNESSES

- coprime.md:82 and DISCOVERIES.md:27 the carpet gap halving per level to `-3.52e-07` at `n = 20`, the sponge gap `-3.45e-05` at `n = 18`.
- coprime.md:126 `lambda_6 = 57 + 6 sqrt 46 = 97.693980`, `lambda_8 = 456 + 3 sqrt 11017 = 770.885694`, energies matched on the integer.
- coprime.md:128-129 the ladder with the fourth-term exponents `0.883757, 0.687305, 0.545409, 0.443659`; coprime.md:133-136 the rows `0.434233, 0.443624, 0.446717, 0.4475978` with `kappa_2K = 1.535026, 1.829430, 1.949148, 1.985806`.
- coprime.md:139 and DISCOVERIES.md:169 the energy cap: components `1, 7, 7, 19` of `1, 9, 9, 25` states, ratios `E_2K(G_a)/lambda_2K^a` at `a = 6` of `1, 0.942327, 0.790590, 0.643725`.
- coprime.md:140-142 and DISCOVERIES.md:170 the master bound at order `2K`, the seam `3a` against `n`, the constant `2/(1 - 3^(-Lambda_2K/2K)) <= 5.1843`, worst ratio `0.7839` at `(11, 2)` for the order-10 block alone over 833 cases and `0.8755` at `(13, 4)` for the min over all orders, zero violations either way, worst main-range sum over cap `0.8630`.
- coprime.md:143-144 and DISCOVERIES.md:171 the `E_10` row, the factored `M_10` polynomial, the Sturm bracket `lambda_10 < 6664.1136626`, `kappa_10 > 1.985805792698`, `beta_0^(10) > 0.447597813453`.
- coprime.md:146-147 rows `12..20` to `0.447930346`, `6.42e-7` under the wall `0.447930987882`; DISCOVERIES.md:353 the Perron roots `59307.487289 .. 387432198.159`.
- DISCOVERIES.md:114 the pincer window `(0.4475978, 0.6402122]`; DISCOVERIES.md:172 and :292 the tenth rung, whose `M_10` this study rebuilds by a second builder.
- coprime.md:168 and DISCOVERIES.md:165 the `n = 13` census `1,044,842 / 1,044,840`, `699,508` singles at 17% of `Z`, `339,530` in `[2,5]` at 55%, ten shifts at 14%, `sum M = 1,577,940`, `max M = 376`.
- coprime.md:169 and DISCOVERIES.md:349 totals `3,151,658`, `9,491,966`, `28,545,342`, peak `occ(8,n)/3^8 = 2.217, 2.492, 2.740`, band exponents `0.5249 / 0.6052, 0.5677, 0.5348 / 0.6096, 0.5765`; coprime.md:159 `Z = 4,003,372 .. 99,800,950` at `n = 13..16`.
