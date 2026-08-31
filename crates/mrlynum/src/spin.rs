use std::f64::consts::{PI, SQRT_2};

fn centre(size: usize) -> f64 {
    size as f64 / 2.0
}

/// The radius of the corner circle of a square raster of the side, the last radius a profile reads.
pub fn reach(size: usize) -> f64 {
    size as f64 / SQRT_2
}

/// The arcs of the circle of the radius about the raster's centre: each as its start angle, end angle and the value of the one cell it lies in, zero outside.
pub fn arcs(data: &[f32], size: usize, radius: f64) -> Vec<(f64, f64, f32)> {
    assert_eq!(data.len(), size * size, "data must be size*size");
    if size == 0 {
        return Vec::new();
    }
    let c = centre(size);
    let r = radius.max(1e-9);
    let mut cuts = vec![0.0, 2.0 * PI];
    let low = (c - r).floor().max(0.0) as usize;
    let high = ((c + r).ceil() as usize).min(size);
    for line in low..=high {
        let u = (line as f64 - c) / r;
        if u.abs() < 1.0 {
            let a = u.acos();
            cuts.push(a);
            cuts.push(2.0 * PI - a);
            let b = u.asin();
            cuts.push(b.rem_euclid(2.0 * PI));
            cuts.push((PI - b).rem_euclid(2.0 * PI));
        }
    }
    cuts.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut out = Vec::with_capacity(cuts.len());
    for pair in cuts.windows(2) {
        let (a, b) = (pair[0], pair[1]);
        if b <= a {
            continue;
        }
        let mid = (a + b) / 2.0;
        let x = c + r * mid.cos();
        let y = c + r * mid.sin();
        let mut value = 0.0;
        if x >= 0.0 && y >= 0.0 {
            let (col, row) = (x as usize, y as usize);
            if col < size && row < size {
                value = data[row * size + col];
            }
        }
        out.push((a, b, value));
    }
    out
}

/// The exact mean of a square raster over the circle of the radius about its centre, each cell read as a constant and the outside as zero.
///
/// ```
/// let solid = vec![1.0; 16];
/// assert!((mrlynum::spin::ring(&solid, 4, 1.0) - 1.0).abs() < 1e-12);
/// assert!(mrlynum::spin::ring(&solid, 4, 3.0).abs() < 1e-12);
/// ```
pub fn ring(data: &[f32], size: usize, radius: f64) -> f64 {
    arcs(data, size, radius)
        .iter()
        .map(|&(a, b, v)| v as f64 * (b - a))
        .sum::<f64>()
        / (2.0 * PI)
}

/// The circular-harmonic power of a raster: for every order `m` up to the last, the energy `sum |c_m(r)|^2 2 pi r dr` of its `m`-th harmonic over rings radii, each ring's coefficient exact from its arcs.
pub fn harmonics(data: &[f32], size: usize, rings: usize, orders: usize) -> Vec<f64> {
    let rings = rings.max(2);
    let far = reach(size);
    let step = far / (rings - 1) as f64;
    let mut power = vec![0.0; orders + 1];
    for k in 0..rings {
        let r = k as f64 * step;
        let pieces = arcs(data, size, r);
        let mut re = vec![0.0; orders + 1];
        let mut im = vec![0.0; orders + 1];
        for &(a, b, v) in &pieces {
            if v == 0.0 {
                continue;
            }
            let v = v as f64;
            re[0] += v * (b - a);
            for m in 1..=orders {
                let f = m as f64;
                re[m] += v * ((f * b).sin() - (f * a).sin()) / f;
                im[m] += v * ((f * b).cos() - (f * a).cos()) / f;
            }
        }
        for m in 0..=orders {
            let (x, y) = (re[m] / (2.0 * PI), im[m] / (2.0 * PI));
            power[m] += (x * x + y * y) * 2.0 * PI * r * step;
        }
    }
    power
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// The rotation order a harmonic power spectrum reveals: the gcd of the orders carrying more than a ten-thousandth of the power, the share pixel aliasing stays under, or zero when none does.
pub fn turns(power: &[f64]) -> usize {
    let total: f64 = power.iter().sum();
    power
        .iter()
        .enumerate()
        .skip(1)
        .filter(|&(_, &p)| p > 1e-4 * total)
        .fold(0, |g, (m, _)| gcd(g, m))
}

/// The petals a full radial stack of the copies shows on a design of the rotation order: their least common multiple.
pub fn petals(copies: usize, order: usize) -> usize {
    if copies == 0 || order == 0 {
        return 0;
    }
    copies / gcd(copies, order) * order
}

/// The way radial copies merge: their mean, their sum, their union, their meet, their parity or what the first keeps that no other has.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blend {
    /// The mean of the copies.
    Mean,
    /// The sum of the copies.
    Sum,
    /// The largest copy.
    Union,
    /// The smallest copy.
    Meet,
    /// The sum folded to its parity.
    Parity,
    /// The first copy less the largest of the rest, floored at zero.
    Difference,
}

impl Blend {
    /// Reads a blend by name: mean, sum, union, meet, parity or difference.
    pub fn named(name: &str) -> Option<Blend> {
        match name {
            "mean" => Some(Blend::Mean),
            "sum" => Some(Blend::Sum),
            "union" => Some(Blend::Union),
            "meet" => Some(Blend::Meet),
            "parity" => Some(Blend::Parity),
            "difference" => Some(Blend::Difference),
            _ => None,
        }
    }

    fn fold(self, values: &[f32]) -> f32 {
        let sum: f32 = values.iter().sum();
        let max = values.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        match self {
            Blend::Mean => sum / values.len() as f32,
            Blend::Sum => sum,
            Blend::Union => max,
            Blend::Meet => values.iter().cloned().fold(f32::INFINITY, f32::min),
            Blend::Parity => 1.0 - (sum.rem_euclid(2.0) - 1.0).abs(),
            Blend::Difference => {
                let rest = values[1..].iter().cloned().fold(0.0, f32::max);
                (values[0] - rest).max(0.0)
            }
        }
    }
}

/// Stacks a raster radially: copies turned by multiples of the step, in turns, about the centre and merged by the blend, on an output raster of the side whose inscribed circle is the source's corner circle, every pixel the mean of samples by samples points.
pub fn radial(
    data: &[f32],
    size: usize,
    out: usize,
    copies: usize,
    step: f64,
    blend: Blend,
    samples: usize,
) -> Vec<f32> {
    assert_eq!(data.len(), size * size, "data must be size*size");
    let copies = copies.max(1);
    let samples = samples.max(1);
    let c = centre(size);
    let scale = 2.0 * reach(size) / out as f64;
    let turns: Vec<(f64, f64)> = (0..copies)
        .map(|k| {
            let angle = 2.0 * PI * step * k as f64;
            (angle.cos(), angle.sin())
        })
        .collect();
    let mut values = vec![0.0f32; copies];
    let mut field = Vec::with_capacity(out * out);
    for i in 0..out {
        for j in 0..out {
            let mut total = 0.0;
            for a in 0..samples {
                for b in 0..samples {
                    let px =
                        (j as f64 + (b as f64 + 0.5) / samples as f64 - out as f64 / 2.0) * scale;
                    let py =
                        (i as f64 + (a as f64 + 0.5) / samples as f64 - out as f64 / 2.0) * scale;
                    for (k, &(cos, sin)) in turns.iter().enumerate() {
                        let x = c + px * cos + py * sin;
                        let y = c - px * sin + py * cos;
                        values[k] = if x >= 0.0 && y >= 0.0 && x < size as f64 && y < size as f64 {
                            data[y as usize * size + x as usize]
                        } else {
                            0.0
                        };
                    }
                    total += blend.fold(&values);
                }
            }
            field.push(total / (samples * samples) as f32);
        }
    }
    field
}

/// The ring profile: the circle means at steps radii spaced evenly from the centre to the corner circle.
pub fn profile(data: &[f32], size: usize, steps: usize) -> Vec<f32> {
    let steps = steps.max(2);
    let far = reach(size);
    (0..steps)
        .map(|k| ring(data, size, far * k as f64 / (steps - 1) as f64) as f32)
        .collect()
}

/// The wheel: a profile spread over a square raster of the side, the corner circle it ends on drawn as the inscribed circle, every pixel reading the profile at its own radius.
pub fn wheel(profile: &[f32], size: usize) -> Vec<f32> {
    let last = profile.len().saturating_sub(1);
    if last == 0 {
        return vec![profile.first().copied().unwrap_or(0.0); size * size];
    }
    let c = centre(size);
    let scale = 2.0 * last as f64 / size as f64;
    let mut out = Vec::with_capacity(size * size);
    for row in 0..size {
        for col in 0..size {
            let (dx, dy) = (col as f64 + 0.5 - c, row as f64 + 0.5 - c);
            let t = (dx * dx + dy * dy).sqrt() * scale;
            let i = (t.floor() as usize).min(last);
            let f = (t - i as f64) as f32;
            let value = if i == last {
                profile[last]
            } else {
                profile[i] * (1.0 - f) + profile[i + 1] * f
            };
            out.push(value);
        }
    }
    out
}

/// The mass a profile carries, the trapezoid integral of `2 pi r F(r)` in cells of the raster it came from.
pub fn mass(profile: &[f32], size: usize) -> f64 {
    let last = profile.len().saturating_sub(1);
    if last == 0 {
        return 0.0;
    }
    let step = reach(size) / last as f64;
    let weight = |k: usize| 2.0 * PI * (k as f64 * step) * profile[k] as f64;
    let inner: f64 = (1..last).map(weight).sum();
    step * (inner + (weight(0) + weight(last)) / 2.0)
}

/// The mass a profile carries inside the radius, the trapezoid integral of `2 pi r F(r)` from the centre out, in cells of the raster it came from.
///
/// ```
/// let solid = vec![1.0; 64];
/// let rings = mrlynum::spin::profile(&solid, 8, 4000);
/// let inner = mrlynum::spin::mass_within(&rings, 8, 3.0);
/// assert!((inner / (9.0 * std::f64::consts::PI) - 1.0).abs() < 1e-3);
/// ```
pub fn mass_within(profile: &[f32], size: usize, radius: f64) -> f64 {
    let last = profile.len().saturating_sub(1);
    if last == 0 || radius <= 0.0 {
        return 0.0;
    }
    let step = reach(size) / last as f64;
    let weight = |k: usize| 2.0 * PI * (k as f64 * step) * profile[k] as f64;
    let full = (radius / step).floor().min(last as f64) as usize;
    let mut total = 0.0;
    for k in 1..=full {
        total += (weight(k - 1) + weight(k)) / 2.0 * step;
    }
    if full < last {
        let rest = radius - full as f64 * step;
        let share = rest / step;
        let edge = weight(full) + (weight(full + 1) - weight(full)) * share;
        total += (weight(full) + edge) / 2.0 * rest;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlycore::atoms;

    fn floats(grid: &mrlycore::tensor::Tensor) -> Vec<f32> {
        grid.bytes().iter().map(|&b| b as f32).collect()
    }

    #[test]
    fn a_solid_square_leaves_through_four_arcs() {
        let side = 8usize;
        let solid = floats(&atoms::ones_2d(side));
        for r in [0.5, 2.0, 3.99] {
            assert!((ring(&solid, side, r) - 1.0).abs() < 1e-12);
        }
        for r in [4.5, 5.0, 5.5] {
            let expect = 1.0 - 4.0 / PI * (side as f64 / 2.0 / r).acos();
            assert!((ring(&solid, side, r) - expect).abs() < 1e-12);
        }
        assert!(ring(&solid, side, reach(side) + 0.01).abs() < 1e-12);
    }

    #[test]
    fn the_carpet_opens_on_a_black_disc() {
        let carpet = atoms::carpet_nd(3, 2).kron(&atoms::carpet_nd(3, 2));
        let data = floats(&carpet);
        assert!(ring(&data, 9, 1.4).abs() < 1e-12);
        assert!(ring(&data, 9, 2.0) > 0.0);
        assert!((ring(&data, 9, 1.0) - 0.0).abs() < 1e-12);
    }

    #[test]
    fn the_mass_of_the_profile_is_the_fill() {
        let carpet = atoms::carpet_nd(3, 2)
            .kron(&atoms::carpet_nd(3, 2))
            .kron(&atoms::carpet_nd(3, 2));
        let data = floats(&carpet);
        let fills = data.iter().sum::<f32>() as f64;
        assert_eq!(fills, 512.0);
        let rings = profile(&data, 27, 4000);
        assert!((mass(&rings, 27) - fills).abs() / fills < 0.002);
    }

    #[test]
    fn the_spin_mass_scales_by_the_fill_about_a_filled_corner() {
        let tile = atoms::carpet_nd(3, 2);
        let carpet = tile.kron(&tile).kron(&tile).kron(&tile);
        let side = 81usize;
        let wide = 2 * side;
        let mut data = vec![0.0f32; wide * wide];
        let bytes = carpet.bytes();
        for row in 0..side {
            for col in 0..side {
                data[(row + side) * wide + col + side] = bytes[row * side + col] as f32;
            }
        }
        let rings = profile(&data, wide, 4000);
        assert!(mass_within(&rings, wide, 1e-6).abs() < 1e-9);
        for radius in [12.0f64, 18.0, 27.0] {
            let near = mass_within(&rings, wide, radius);
            let far = mass_within(&rings, wide, 3.0 * radius);
            assert!(near > 0.0);
            assert!((far / (8.0 * near) - 1.0).abs() < 0.08, "radius {radius}");
        }
        let whole = mass_within(&rings, wide, reach(wide));
        assert!((whole - mass(&rings, wide)).abs() / whole < 1e-9);
    }

    #[test]
    fn the_wheel_reads_the_profile_by_radius() {
        let rings = profile(&floats(&atoms::ones_2d(8)), 8, 64);
        let spun = wheel(&rings, 16);
        assert_eq!(spun.len(), 256);
        assert!((spun[8 * 16 + 8] - 1.0).abs() < 1e-6);
        assert!((spun[8 * 16 + 12] - 1.0).abs() < 1e-6);
        assert!(spun[8 * 16 + 15] < 0.2);
        assert_eq!(spun[0], 0.0);
        assert_eq!(profile(&[], 0, 0).len(), 2);
    }

    fn at(field: &[f32], out: usize, size: usize, px: f64, py: f64) -> f32 {
        let scale = 2.0 * reach(size) / out as f64;
        let col = (px / scale + out as f64 / 2.0) as usize;
        let row = (py / scale + out as f64 / 2.0) as usize;
        field[row * out + col]
    }

    #[test]
    fn two_squares_at_an_eighth_turn_make_a_star() {
        let solid = floats(&atoms::ones_2d(8));
        let stack = |blend| radial(&solid, 8, 64, 2, 0.125, blend, 1);
        let probe = |blend, px, py| at(&stack(blend), 64, 8, px, py);
        assert_eq!(probe(Blend::Union, 0.0, 0.0), 1.0);
        assert_eq!(probe(Blend::Union, 0.0, -5.0), 1.0);
        assert_eq!(probe(Blend::Meet, 0.0, -5.0), 0.0);
        assert_eq!(probe(Blend::Mean, 0.0, -5.0), 0.5);
        assert_eq!(probe(Blend::Sum, 0.0, 0.0), 2.0);
        assert_eq!(probe(Blend::Parity, 0.0, -5.0), 1.0);
        assert_eq!(probe(Blend::Parity, 0.0, 0.0), 0.0);
        assert_eq!(probe(Blend::Difference, 0.0, -5.0), 0.0);
        assert_eq!(probe(Blend::Difference, 3.8, -3.8), 1.0);
        assert_eq!(Blend::named("soup"), None);
    }

    #[test]
    fn the_harmonics_read_the_rotation_order() {
        let square = harmonics(&floats(&atoms::ones_2d(8)), 8, 128, 12);
        assert!(square[0] > 0.0);
        assert!(square[4] > 1e-3 * square[0]);
        assert!(square[8] > 1e-3 * square[0]);
        for m in [1, 2, 3, 5, 6, 7, 9, 10, 11] {
            assert!(square[m] < 1e-9 * square[0], "m {m}");
        }
        assert_eq!(turns(&square), 4);
        let mut bar = vec![0.0f32; 16];
        for i in [5, 6, 9, 10, 4, 11] {
            bar[i] = 1.0;
        }
        assert_eq!(turns(&harmonics(&bar, 4, 64, 8)), 2);
        let mut blob = vec![0.0f32; 16];
        blob[0] = 1.0;
        assert_eq!(turns(&harmonics(&blob, 4, 64, 8)), 1);
        assert_eq!(turns(&[1.0, 0.0, 0.0]), 0);
        assert_eq!(
            (petals(6, 4), petals(8, 4), petals(5, 1), petals(3, 0)),
            (12, 8, 5, 0)
        );
    }
}
