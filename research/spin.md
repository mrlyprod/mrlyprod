# Spin

Turn a design about its centre and look at the average. A seed: the instrument exists, the first identities are proved, the census is still to run.

**Proved** means a proof is given here; **Verified** means recomputed by a crate test; **Conjecture** means neither. The generators are `mrlynum::spin` and the host fixture of `mrlyweb`. The [spin demo](../demos/spin.html) shows the infinite spin, the [radial demo](../demos/radial.html) the finite ones and the harmonics each keeps.

## The identities

- Write a picture in circular harmonics, `f(r, phi) = sum_m f_m(r) e^(i m phi)`. The average over the `q` rotations by multiples of `2 pi / q` keeps exactly the orders with `q | m`; the average over all rotations keeps `m = 0`. **Proved**: a rotation by `2 pi / q` multiplies `f_m` by `e^(2 pi i m / q)`, and the `q`-th roots of unity sum to zero unless `q | m`.
- A screen turning a design by `p/q` of a turn per frame, `p/q` in lowest terms, shows only `q` orientations, so a long afterglow shows the `q`-average. A design of rotation order `g` has `f_m = 0` unless `g | m`, so the lowest surviving order is `lcm(q, g)`: the petal count. **Proved**; witness `the_harmonics_read_the_rotation_order`.
- The infinite spin of a raster is its ring profile `F(r)`, the exact circle mean, and `int 2 pi r F(r) dr` is the fill. **Proved**; witness `the_mass_of_the_profile_is_the_fill`. The carpet's profile is zero to `r = side/6`, its central hole. **Verified.**
- A plane wave averaged over a turn is `J0(|k| r)`, so the infinite spin is the Hankel transform of the radially averaged spectrum, ringing at the lattice norms `a^2 + b^2` and `a^2 + ab + b^2`. **Proved.**

## Known elsewhere

- A picture laid on a slightly turned copy shows concentric circles ([Glass 1969](REFS.md)).
- Orientation-averaged scattering from the Menger sponge and the carpet is log-periodic, `I(q) q^D` with period the scaling factor ([Cherny, Anitas, Osipov and Kuklin 2011](REFS.md)).
- The spherical average of `|mu^|^2` decides whether a fractal's distance set has positive length ([Mattila 1987](REFS.md)).
- Self-similar sets with irrational rotations have no exceptional projection; the designs have no rotation, so a sponge's exceptional shadows are rational ([Falconer, Fraser and Jin 2015](REFS.md)).

## The questions

All **Conjecture** until run.

- Spin dimension: `M(r) = int_0^r 2 pi s F(s) ds ~ r^D` with a ripple of period `log q` in `log r`; the slope should match `dimension`, the ripple should separate codes of equal dimension.
- Spin spectrum: `P_m = int |f_m(r)|^2 2 pi r dr` is a rotation-invariant fingerprint. Are designs outside one symmetry orbit spin-isospectral?
- Powder rings: the ring average of the power spectrum should fall as `-D` under Bessel rings at the lattice norms.
- The sponge's shadow: projected area by direction, its minimum, the census at level `L` in direction `(a, b, c)`.
- Overlap: `A(theta) = |f ∩ R_theta f|` has mean `sum_m P_m` by Parseval, one check on the other.
