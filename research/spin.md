# Spin

Turn a design about its centre and look at the average. A seed: the instrument exists, the first identities are proved, the census is still to run.

**Proved** means a proof is given here; **Verified** means recomputed by a crate test; **Conjecture** means neither. The generators are `mrlynum::spin` and the host fixture of `mrlyweb`. The [spin demo](../demos/spin/) shows the infinite spin, the [radial demo](../demos/radial/) the finite ones and the harmonics each keeps.

## The identities

- Write a picture in circular harmonics, `f(r, phi) = sum_m f_m(r) e^(i m phi)`. The average over the `q` rotations by multiples of `2 pi / q` keeps exactly the orders with `q | m`; the average over all rotations keeps `m = 0`. **Proved**: a rotation by `2 pi / q` multiplies `f_m` by `e^(2 pi i m / q)`, and the `q`-th roots of unity sum to zero unless `q | m`.
- A screen turning a design by `p/q` of a turn per frame, `p/q` in lowest terms, shows only `q` orientations, so a long afterglow shows the `q`-average. A design of rotation order `g` has `f_m = 0` unless `g | m`, so the lowest surviving order is `lcm(q, g)`: the petal count. **Proved**; witness `the_harmonics_read_the_rotation_order`.
- The infinite spin of a raster is its ring profile `F(r)`, the exact circle mean, and `int 2 pi r F(r) dr` is the fill. **Proved**; witness `the_mass_of_the_profile_is_the_fill`. The carpet's profile is zero to `r = side/6`, its central hole. **Verified.**
- A plane wave averaged over a turn is `J0(|k| r)`, so the infinite spin is the Hankel transform of the radially averaged spectrum, ringing at the lattice norms `a^2 + b^2` and `a^2 + ab + b^2`. **Proved.**
- The primes decide the rings. The square lattice rings at `sqrt(n)`, `n` in [A001481](REFS.md), with weight `r2(n) = 4 (d1 - d3)`, [A004018](REFS.md): silent exactly where a prime `3 (mod 4)` divides `n` to an odd power, and summing to `4 zeta(s) L(s, chi_4)`, the zeta of `Z[i]`. The hexagonal lattice rings at [A003136](REFS.md) with weight [A004016](REFS.md), `6 zeta(s) L(s, chi_-3)`, the zeta of `Z[omega]`: the two L-functions of [pi](pi.md) and [bases](bases.md). The mass of a spun lattice is the Gauss circle count, and Hardy's Bessel series for its error is this ring expansion ([Hardy 1915](REFS.md)). **Proved.** The [gaussian demo](../demos/gaussian/) paints the primes of both rings and bars the weights.

## Known elsewhere

- A picture laid on a slightly turned copy shows concentric circles ([Glass 1969](REFS.md)).
- Orientation-averaged scattering from the sponge and the carpet is log-periodic with period the scaling factor ([Cherny, Anitas, Osipov and Kuklin 2011](REFS.md)).
- The spherical average of `|mu^|^2` decides whether a fractal's distance set has positive length ([Mattila 1987](REFS.md)).
- Self-similar sets with irrational rotations have no exceptional projection; the designs have none, so a sponge's exceptional shadows are rational ([Falconer, Fraser and Jin 2015](REFS.md)).

## The questions

- The coprime law does not spin. Flat layers at coprime odd scales are uncorrelated, the stack's prime detector; their ring profiles correlate at `+0.38` for `(3, 5)`, `-0.33` for `(5, 7)`, `+0.38` for `(9, 13)`, no better than `gcd` pairs. The cancellation is separable in `x` and `y`, and the spin discards the angle. **Refuted**; witness `the_coprime_law_dies_under_the_spin`.
- A Gaussian Farey: scale `n` adds rings at `sqrt(k)/n`, and the radii new at `n` should be counted by the primitive representations of `k` in `Z[i]` modulo units, the spun `phi(n)` of [farey](farey.md). **Conjecture.**

The rest is **Conjecture** until run.

- Spin dimension: `M(r) = int_0^r 2 pi s F(s) ds ~ r^D` with a ripple of period `log q` in `log r`; the slope should match `dimension`, the ripple should separate codes of equal dimension.
- Spin spectrum: `P_m = int |f_m(r)|^2 2 pi r dr` is a rotation-invariant fingerprint. Are designs outside one symmetry orbit spin-isospectral?
- Powder rings: the ring average of the power spectrum should fall as `-D` under the Bessel rings.
- The sponge's shadow: projected area by direction, its minimum, the census at level `L` in direction `(a, b, c)`.
