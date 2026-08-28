# dimension-one-ladder

- Regenerates the coprime terms of the three base-three mask designs, the moment ladder of the ternary gasket, and the occupied-ray census.
- Terms: `A(n)` for the carpet, sponge and Vicsek plus by Mobius over the quotient box with a subset-lattice inner count, checked against brute force and the kept `terms/` files.
- Ladder: exact carry matrices for the even moments `E_2K(G_a)`, a direct convolution as second builder, factored characteristic polynomials, the rungs `beta_0^(2K)` to `2K = 20`, the Fourier identities and master bound on every prime `5..199`, the dyadic tail.
- Census: every ray of the level-`n` gasket by exact enumeration, per-octave counts, `Z`, and the multiplier distribution at `n = 13`.

## RUN

- `CARGO_BUILD_JOBS=4 cargo run --release --manifest-path research/lab/Cargo.toml -p dimension-one-ladder` runs the carpet and plus to `n = 18`, the sponge to `n = 16`, the census at `n = 13..16`, about two minutes on eight cores.
- `... -p dimension-one-ladder -- terms carpet 20` or `census 13 14` picks one job; the kept rows reach `n = 20` for the two planar designs and `n = 18` for the sponge, and print from `terms/` labelled stored only.
- `uv run python research/lab/dimension-one-ladder/ladder.py` runs the ladder in about a second.

## WITNESSES

- coprime.md:82 and DISCOVERIES.md:27 the carpet gap halving per level to `-3.52e-07` at `n = 20`, the sponge gap `-3.45e-05` at `n = 18`.
- coprime.md:125 `lambda_6 = 57 + 6 sqrt 46 = 97.693980`, `lambda_8 = 456 + 3 sqrt 11017 = 770.885694`, energies matched on the integer.
- coprime.md:131-134 the ladder rows `0.434233, 0.443624, 0.446717, 0.447598` with `kappa_2K = 1.535026, 1.829430, 1.949148, 1.985806`.
- coprime.md:138 the `E_10` row, the factored `M_10` polynomial, `lambda_10 = 6664.113662506`, `kappa_10 = 1.985805792712`, `beta_0^(10) = 0.447597813454`, `0.000880502992` above the eighth.
- coprime.md:140-141 rows `12..20` to `0.447930346`, `6.42e-7` under the wall `0.447930987882`; DISCOVERIES.md:336 the Perron roots `59307.487289 .. 387432198.159`.
- DISCOVERIES.md:114 the master bound on primes `5..199` with zero violations; DISCOVERIES.md:169 and :275 the tenth rung, whose `M_10` this study rebuilds by a second builder.
- coprime.md:160 and DISCOVERIES.md:164 the `n = 13` census `1,044,842 / 1,044,840`, `699,508` singles at 17% of `Z`, `339,530` in `[2,5]` at 55%, ten shifts at 14%, `sum M = 1,577,940`, `max M = 376`.
- coprime.md:161 and DISCOVERIES.md:332 totals `3,151,658`, `9,491,966`, `28,545,342`, peak `occ(8,n)/3^8 = 2.217, 2.492, 2.740`, band exponents `0.5249 / 0.6052, 0.5677, 0.5348 / 0.6096, 0.5765`; coprime.md:151 `Z = 4,003,372 .. 99,800,950` at `n = 13..16`.
