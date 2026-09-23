use std::f64::consts::{PI, SQRT_2};
use std::sync::OnceLock;

/// The largest radius the closed tube formula reaches, `1/6`: past it the walls across an arm and the centre cube's edge cylinders start to meet.
pub const EDGE: f64 = 1.0 / 6.0;

/// The covering radius of the plus, `sqrt(2)/6`, the distance from the centre of the cube to the sponge: every radius from here on swallows the plus whole.
pub const COVER: f64 = SQRT_2 / 6.0;

const PLUS: f64 = 7.0 / 27.0;
const DEPTH: usize = 40;
const DIGITS: usize = 40;
const FLOOR: i64 = 32;
const REACH: i64 = 14;
const NEGLIGIBLE: f64 = 1e-17;
const NODES: usize = 16;
const WEIGHT: [f64; 3] = [3.0, 2.0, 3.0];
const RUNNING: [f64; 3] = [0.0, 3.0, 5.0];

// DISTANCE

fn split(x: f64) -> (usize, f64) {
    let digit = (3.0 * x).floor().clamp(0.0, 2.0);
    (digit as usize, (3.0 * x - digit).clamp(0.0, 1.0))
}

fn carpet(mut across: f64, mut along: f64) -> f64 {
    let mut scale = 1.0 / 3.0;
    for _ in 0..DIGITS {
        let ((a, fa), (b, fb)) = (split(across), split(along));
        if a == 1 && b == 1 {
            return scale * fa.min(1.0 - fa).min(fb).min(1.0 - fb);
        }
        across = fa;
        along = fb;
        scale /= 3.0;
    }
    0.0
}

fn gap(v: f64) -> f64 {
    (v - 1.0 / 3.0).abs().min((v - 2.0 / 3.0).abs())
}

fn plus(point: [f64; 3]) -> f64 {
    let middle = point.map(|v| (1.0 / 3.0..=2.0 / 3.0).contains(&v));
    let gaps = point.map(gap);
    match (0..3).find(|&k| !middle[k]) {
        None => {
            let mut sorted = gaps;
            sorted.sort_by(f64::total_cmp);
            sorted[0].hypot(sorted[1])
        }
        Some(axis) => {
            let v = point[axis];
            let along = 3.0 * if v < 1.0 / 3.0 { v } else { v - 2.0 / 3.0 };
            let (i, j) = ((axis + 1) % 3, (axis + 2) % 3);
            let wall_i = carpet(3.0 * point[j] - 1.0, along) / 3.0;
            let wall_j = carpet(3.0 * point[i] - 1.0, along) / 3.0;
            gaps[i].hypot(wall_i).min(gaps[j].hypot(wall_j))
        }
    }
}

// HOLES

fn sweep(t: f64, radius: f64) -> f64 {
    if t >= radius {
        return PI * radius * radius / 4.0;
    }
    t / 2.0 * (radius * radius - t * t).sqrt() + radius * radius / 2.0 * (t / radius).asin()
}

fn moment(t: f64, radius: f64) -> f64 {
    let r2 = radius * radius;
    if t >= radius {
        return r2 * radius / 3.0;
    }
    let rest = r2 - t * t;
    t * t * (3.0 * r2 * r2 - 3.0 * r2 * t * t + t.powi(4))
        / (3.0 * (r2 * radius + rest * rest.sqrt()))
}

fn piece(from: f64, to: f64, width: f64, slope: f64, radius: f64) -> f64 {
    if to <= from {
        return 0.0;
    }
    width * (sweep(to, radius) - sweep(from, radius))
        - slope * (moment(to, radius) - moment(from, radius))
}

fn hole(side: f64, radius: f64) -> f64 {
    4.0 * piece(0.0, side / 2.0, side, 2.0, radius)
}

fn partial(side: f64, cut: f64, radius: f64) -> f64 {
    let left = piece(0.0, (side / 2.0).min(cut), side, 2.0, radius);
    let right = piece((side - cut).max(0.0), side / 2.0, side, 2.0, radius);
    let bottom = if cut <= side / 2.0 {
        piece(0.0, cut, cut, 1.0, radius)
    } else {
        piece(0.0, side - cut, cut, 1.0, radius) + piece(side - cut, side / 2.0, side, 2.0, radius)
    };
    left + right + 2.0 * bottom
}

fn places(mut count: f64, digits: usize) -> Vec<usize> {
    let mut out = Vec::with_capacity(digits);
    for _ in 0..digits {
        let rest = (count / 3.0).floor();
        out.push(((count - 3.0 * rest) as usize).min(2));
        count = rest;
    }
    out
}

fn column(index: f64, digits: usize) -> f64 {
    places(index, digits).iter().map(|&k| WEIGHT[k]).product()
}

fn below(count: f64, digits: usize) -> f64 {
    if count >= 3f64.powi(digits as i32) {
        return 8f64.powi(digits as i32);
    }
    let (mut total, mut prefix, mut eight) = (0.0, 1.0, 8f64.powi(digits as i32));
    for &k in places(count, digits).iter().rev() {
        eight /= 8.0;
        total += prefix * RUNNING[k] * eight;
        prefix *= WEIGHT[k];
    }
    total
}

fn levels(radius: f64) -> usize {
    ((1.0 / radius).log(3.0).ceil() as i64 + REACH).max(FLOOR) as usize
}

fn width(level: usize) -> f64 {
    3f64.powi(-(level as i32) - 1)
}

fn wall(radius: f64, top: usize) -> f64 {
    let mut total = 0.0;
    for level in 1..=top {
        total += 8f64.powi(level as i32 - 1) * hole(width(level), radius);
    }
    total / 2.0 + radius * (8.0f64 / 9.0).powi(top as i32) / 18.0
}

fn strip(radius: f64, top: usize) -> f64 {
    let (mut total, mut share) = (0.0, 0.0);
    for level in 1..=top {
        let side = width(level);
        let ratio = radius / side;
        let columns = 3f64.powi(level as i32 - 1);
        let mut here = 0.0;
        let full = ((ratio - 2.0) / 3.0).floor() + 1.0;
        if full > 0.0 {
            here += below(full.min(columns), level - 1) * hole(side, radius);
        }
        let cut = ((ratio - 1.0) / 3.0).floor();
        let start = (3.0 * cut + 1.0) * side;
        if cut >= 0.0 && cut < columns && start < radius && radius < start + side {
            here += column(cut, level - 1) * partial(side, radius - start, radius);
        }
        total += here;
        share = here / (8f64.powi(level as i32 - 1) * radius * side * side);
    }
    total + share * radius * (8.0f64 / 9.0).powi(top as i32) / 9.0
}

fn inside(radius: f64) -> f64 {
    let top = levels(radius);
    (PI + 8.0) * radius * radius - 8.0 * SQRT_2 * radius.powi(3)
        + 48.0 * (wall(radius, top) - strip(radius, top))
}

fn crown(radius: f64) -> f64 {
    let lap = (radius * radius - EDGE * EDGE).max(0.0).sqrt();
    PI * radius * radius
        - 8.0 * SQRT_2 * radius.powi(3)
        - 4.0 * radius * radius * (EDGE / radius).min(1.0).acos()
        + (2.0 / 3.0 + 16.0 * lap * lap) * lap
        + 2.0 / 9.0
}

// CORNER

fn legendre() -> &'static [(f64, f64)] {
    static RULE: OnceLock<Vec<(f64, f64)>> = OnceLock::new();
    RULE.get_or_init(|| {
        let n = NODES as f64;
        (1..=NODES)
            .map(|i| {
                let mut x = (PI * (i as f64 - 0.25) / (n + 0.5)).cos();
                let mut slope = 1.0;
                for _ in 0..64 {
                    let (mut low, mut high) = (1.0, x);
                    for k in 2..=NODES {
                        let k = k as f64;
                        (low, high) = (high, ((2.0 * k - 1.0) * x * high - (k - 1.0) * low) / k);
                    }
                    slope = n * (x * high - low) / (x * x - 1.0);
                    let step = high / slope;
                    x -= step;
                    if step.abs() < 1e-17 {
                        break;
                    }
                }
                (x, 2.0 / ((1.0 - x * x) * slope * slope))
            })
            .collect()
    })
}

fn pair(radius: f64, start: f64, side: f64, line: f64) -> f64 {
    let (half, reach) = (side / 2.0, line - start);
    let angle = |depth: f64| (depth / radius).clamp(0.0, 1.0).asin();
    let edge = |y: f64| ((radius - y) * (radius + y)).max(0.0).sqrt().atan2(y);
    let (low, high) = (edge(line), edge(start));
    let mut cuts = vec![low, high];
    for depth in [half, reach, side - reach] {
        cuts.push(angle(depth));
    }
    for shift in [start, start + side] {
        let spread = 2.0 * radius * radius - shift * shift;
        if spread >= 0.0 {
            cuts.push(angle(
                ((radius - shift) * (radius + shift) / (spread.sqrt() + shift)).abs(),
            ));
        }
    }
    cuts.retain(|phi| (low..=high).contains(phi));
    cuts.sort_by(f64::total_cmp);
    let mut total = 0.0;
    for window in cuts.windows(2) {
        let (middle, span) = ((window[0] + window[1]) / 2.0, (window[1] - window[0]) / 2.0);
        for &(x, weight) in legendre() {
            let phi = middle + span * x;
            let depth = radius * phi.sin();
            let across = radius - start - 2.0 * radius * (phi / 2.0).sin().powi(2);
            let width = reach.min(side - depth) - across.max(depth);
            if depth < half && width > 0.0 {
                total += weight * span * (half - depth) * width * depth;
            }
        }
    }
    4.0 * total
}

// EXPORTS

/// The Minkowski dimension of the Menger sponge, `log(20)/log(3)`, the similarity dimension of its 20 maps of ratio `1/3`.
pub fn dimension() -> f64 {
    20f64.ln() / 3f64.ln()
}

/// The Euclidean distance from a point of the unit cube to the Menger sponge, exact to the last binary place.
///
/// The point descends the base-3 digits while its digit triple holds at most one `1`, each step
/// dividing the distance by 3, since inside a retained subcube the distance to the sponge is the
/// distance to that subcube's copy of it. Once it lands in the plus of seven removed cubes, the
/// distance is the one to the twelve edges of the centre cube or to the four carpets walling the
/// arm it sits in. Coordinates outside `[0, 1]` are clamped; the window centre `(1/2, 1/2, 0)`
/// reads `1/6` and the centre of the cube `sqrt(2)/6`.
pub fn distance(point: [f64; 3]) -> f64 {
    let mut local = point.map(|v| v.clamp(0.0, 1.0));
    let mut scale = 1.0;
    for _ in 0..DIGITS {
        let digits = local.map(split);
        if digits.iter().filter(|(d, _)| *d == 1).count() >= 2 {
            return scale * plus(local);
        }
        local = digits.map(|(_, rest)| rest);
        scale /= 3.0;
    }
    0.0
}

/// The volume `T(radius)` of the points of the plus of seven removed level-1 cubes within `radius` of the sponge, or `None` on `(1/6, sqrt(2)/6)`, where no closed form is known.
///
/// On `(0, 1/6]` it is `(pi + 8) r^2 - 8 sqrt(2) r^3 + 48 (V1 - A1)`, the centre cube's edge
/// cylinders plus the arcsine hole sums of the 24 wall carpets, `V1` over half a wall and `A1`
/// over the strip along a shared edge, both summed level by level and closed by their geometric
/// tails. The one term left out is `24 Deep`, `Deep` the corner volume beyond both wall tubes,
/// at most `24 x 3.842e-5 < 9.23e-4` at `1/6` and `5.81e-9` at `1/12` by the paper's bound on
/// `Deep`, so this is the upper edge of the certified enclosure:
/// `T(1/6) = (pi + 8)/36 - sqrt(2)/27`. From `sqrt(2)/6` on the plus is swallowed, `7/27`.
pub fn tube(radius: f64) -> Option<f64> {
    if radius.is_nan() || (radius > EDGE && radius < COVER) {
        return None;
    }
    if radius <= 0.0 {
        return Some(0.0);
    }
    if radius >= COVER {
        return Some(PLUS);
    }
    Some(inside(radius))
}

/// The volume `Deep(radius)` of the points of one arm's quarter beyond the tubes of both walls it touches, at every radius, to double precision.
///
/// With `line = min(radius, 1/6)`, the quarter is `[0, line]^2` across the arm times its length,
/// and every point of `Deep` sits, through both of its wall coordinates, in one and the same hole
/// of the wall carpet that the line `across = line` cuts: a crossing hole. The volume is the sum
/// over crossing holes of one arcsine integral, `2^(m-1)` holes of side `3^(-m-1)` at every level
/// `m` for `line = 1/6`, none at all for `1/12`, whose triple `1/4 = 0.0202...` has no digit `1`.
/// It vanishes from `sqrt(10)/18` on, where the arms are swallowed. Each integral is elementary, but its
/// expanded arcsine form cancels at deep levels, so it is summed by 16-point Gauss-Legendre on each smooth
/// piece in the angle `asin(R/radius)`, `R` the depth that a wall point needs.
pub fn deep(radius: f64) -> f64 {
    if !(radius > 0.0 && radius.is_finite()) {
        return 0.0;
    }
    let line = radius.min(EDGE);
    let square = radius * radius;
    let (mut local, mut start, mut count, mut total) = (3.0 * line, 0.0, 1.0, 0.0);
    for level in 1..=levels(radius) {
        let side = width(level);
        let digit = (3.0 * local).floor().clamp(0.0, 2.0);
        local = 3.0 * local - digit;
        if digit == 1.0 {
            if count * 2.0 * (side / 2.0).powi(5) < NEGLIGIBLE * square * square {
                break;
            }
            total += count * pair(radius, start + side, side, line);
        }
        start += digit * side;
        count *= if digit == 1.0 { 2.0 } else { 3.0 };
    }
    total
}

/// The volume `T(radius)` of the points of the plus within `radius` of the sponge at every radius, `Deep` included, to double precision.
///
/// On `(0, 1/6]` it is the closed form of `tube` less `24 Deep`. On `[1/6, sqrt(2)/6]` the arms
/// hold `2/9 - 24 Deep` and the centre cube `pi r^2 - 8 sqrt(2) r^3 - 4 r^2 acos(1/(6r)) +
/// (2/3 + 16 c^2) c` with `c = sqrt(r^2 - 1/36)`, the edge cylinders overlapping along the faces;
/// from `sqrt(2)/6` on the plus is swallowed, `7/27`.
pub fn exact(radius: f64) -> f64 {
    if radius <= 0.0 {
        return 0.0;
    }
    if radius >= COVER {
        return PLUS;
    }
    let body = if radius <= EDGE {
        inside(radius)
    } else {
        crown(radius)
    };
    body - 24.0 * deep(radius)
}

/// The volume of the points of the unit cube within `radius` of the sponge, `sum_k (20/27)^k T(3^k radius)`, or `None` when some `3^k radius` falls where `T` has no closed form.
///
/// The cube is the plus and 20 retained subcubes, each holding a copy of the whole picture at a
/// third of the scale, so the volume is `T(radius)` plus `20/27` of the volume at three times the
/// radius, and the sum stops at the first radius past `sqrt(2)/6`, where the volume is the cube.
pub fn volume(radius: f64) -> Option<f64> {
    if radius.is_nan() {
        return None;
    }
    if radius <= 0.0 {
        return Some(0.0);
    }
    let (mut total, mut weight, mut reach) = (0.0, 1.0, radius);
    while reach < COVER {
        total += weight * tube(reach)?;
        weight *= 20.0 / 27.0;
        reach *= 3.0;
    }
    Some(total + weight)
}

/// The Minkowski reading `radius^(D-3)` times the volume inside the cube, the number whose limit as the radius shrinks would be the sponge's Minkowski content.
pub fn reading(radius: f64) -> Option<f64> {
    Some(radius.powf(dimension() - 3.0) * volume(radius)?)
}

/// The periodic function `p` of Kombrink, Pearse and Winter at `radius`: the reading's limit profile, unchanged when the radius is multiplied by 3, or `None` on the phases `(1/6, sqrt(2)/6]` where `T` has no closed form.
///
/// The radius is first carried by powers of 3 into `(sqrt(2)/18, sqrt(2)/6]`; on
/// `(sqrt(2)/18, 1/6]` it is `r^(D-3) (20/27 + sum_(l = 0..40) (27/20)^l T(r/3^l))`, and the
/// sponge is Minkowski measurable exactly when this is constant. It is not: the certificate puts
/// `p(1/12)` in `[2.122718, 2.122723]` and `p(1/6)` in `[2.135019, 2.136794]`.
pub fn profile(radius: f64) -> Option<f64> {
    if !(radius > 0.0 && radius.is_finite()) {
        return None;
    }
    let mut phase = radius;
    while phase > COVER {
        phase /= 3.0;
    }
    while phase <= COVER / 3.0 {
        phase *= 3.0;
    }
    if phase > EDGE {
        return None;
    }
    let (mut total, mut weight, mut reach) = (20.0 / 27.0, 1.0, phase);
    for _ in 0..=DEPTH {
        total += weight * tube(reach)?;
        weight *= 27.0 / 20.0;
        reach /= 3.0;
    }
    Some(phase.powf(dimension() - 3.0) * total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_reads_the_named_points() {
        let near = |a: f64, b: f64| (a - b).abs() < 1e-15;
        assert!(near(distance([0.5, 0.5, 0.5]), SQRT_2 / 6.0));
        assert!(near(distance([1.0 / 6.0, 1.0 / 6.0, 0.5]), SQRT_2 / 18.0));
        assert!(near(distance([0.5, 0.5, 1.0]), 1.0 / 6.0));
        assert!(near(distance([2.0 / 3.0, 0.5, 5.0 / 6.0]), 1.0 / 18.0));
        assert_eq!(distance([0.0, 0.25, 1.0]), 0.0);
    }

    #[test]
    fn tube_meets_the_identity_at_one_sixth() {
        let closed = (PI + 8.0) / 36.0 - SQRT_2 / 27.0;
        assert!((tube(EDGE).unwrap() - closed).abs() < 1e-13);
        assert_eq!(tube(0.2), None);
        assert_eq!(tube(COVER), Some(PLUS));
    }

    #[test]
    fn tube_lies_in_its_enclosures() {
        let twelfth = tube(1.0 / 12.0).unwrap();
        let eighth = tube(1.0 / 8.0).unwrap();
        assert!((0.180947086..=0.180947093).contains(&twelfth));
        assert!((0.234186414..=0.234701259).contains(&eighth));
    }

    #[test]
    fn tube_is_bracketed_by_the_distance_raster() {
        let n = 36;
        let cell = 1.0 / (3.0 * n as f64);
        let half = cell * 3f64.sqrt() / 2.0;
        let mut seen = Vec::with_capacity(2 * n * n * n);
        for (corner, copies) in [([1.0, 1.0, 1.0], 1.0), ([1.0, 1.0, 0.0], 6.0)] {
            for a in 0..n {
                for b in 0..n {
                    for c in 0..n {
                        let at = [a, b, c].map(|v| (v as f64 + 0.5) * cell);
                        let point = [0, 1, 2].map(|k| corner[k] / 3.0 + at[k]);
                        seen.push((distance(point), copies));
                    }
                }
            }
        }
        let volume = cell.powi(3);
        for (radius, closed) in [(1.0 / 12.0, tube(1.0 / 12.0).unwrap()), (0.2, exact(0.2))] {
            let count = |edge: f64| {
                seen.iter()
                    .filter(|(d, _)| *d <= edge)
                    .map(|(_, c)| c)
                    .sum::<f64>()
            };
            assert!(
                count(radius - half) * volume <= closed && closed <= count(radius + half) * volume
            );
        }
    }

    #[test]
    fn deep_lives_in_the_crossing_holes() {
        assert_eq!(deep(1.0 / 12.0), 0.0);
        assert_eq!(deep(0.18), 0.0);
        assert!((deep(EDGE) - 1.8882e-6).abs() < 1.4e-8);
        assert!((deep(1.0 / 8.0) - 4.7432e-8).abs() < 2.9e-9);
        let terms: Vec<f64> = (1..=10)
            .map(|m| {
                let side = width(m);
                2f64.powi(m as i32 - 1) * pair(EDGE, EDGE - side / 2.0, side, EDGE)
            })
            .collect();
        assert!(terms
            .windows(2)
            .skip(1)
            .all(|p| (p[1] / p[0] * 243.0 / 2.0 - 1.0).abs() < 0.02));
        let radius: f64 = 0.17;
        let low = (radius * radius - 1.0 / 324.0).sqrt();
        let (n, m) = (48, 240);
        let (step, pace) = ((EDGE - low) / n as f64, (1.0 / 9.0) / m as f64);
        let half = (2.0 * step * step + pace * pace).sqrt() / 2.0;
        let (mut inner, mut outer) = (0.0, 0.0);
        for a in 0..n {
            for b in 0..n {
                for c in 0..m {
                    let u = 1.0 / 3.0 + low + (a as f64 + 0.5) * step;
                    let v = 1.0 / 3.0 + low + (b as f64 + 0.5) * step;
                    let d = distance([1.0 / 9.0 + (c as f64 + 0.5) * pace, u, v]);
                    inner += f64::from(u8::from(d > radius + half));
                    outer += f64::from(u8::from(d > radius - half));
                }
            }
        }
        let volume = step * step * pace;
        let closed = deep(radius);
        assert!(inner * volume <= closed && closed <= outer * volume);
    }

    #[test]
    fn exact_joins_at_one_sixth_and_lies_in_the_enclosures() {
        assert!((crown(EDGE) - inside(EDGE)).abs() < 1e-13);
        assert!((crown(COVER) - PLUS).abs() < 1e-15);
        assert!((0.180947086..=0.180947093).contains(&exact(1.0 / 12.0)));
        assert!((0.234186414..=0.234701259).contains(&exact(1.0 / 8.0)));
        assert!((0.256188319..=0.257110405).contains(&exact(EDGE)));
        let walk: Vec<f64> = (1..=400).map(|k| exact(k as f64 * COVER / 400.0)).collect();
        assert!(walk.windows(2).all(|pair| pair[0] < pair[1]));
        assert_eq!(exact(COVER), PLUS);
    }

    #[test]
    fn profile_lies_in_its_bands_and_swings() {
        let twelfth = profile(1.0 / 12.0).unwrap();
        let eighth = profile(1.0 / 8.0).unwrap();
        let sixth = profile(1.0 / 6.0).unwrap();
        assert!((2.122718..=2.122723).contains(&twelfth));
        assert!((2.134668..=2.135742).contains(&eighth));
        assert!((2.135019..=2.136794).contains(&sixth));
        assert!(sixth - twelfth >= 0.012296);
        assert!((profile(1.0 / 36.0).unwrap() - twelfth).abs() < 1e-12);
        assert_eq!(profile(0.2), None);
    }

    #[test]
    fn reading_climbs_onto_the_profile() {
        let radius = 1.0 / 12.0;
        let gaps: Vec<f64> = (0..10)
            .map(|k| radius / 3f64.powi(k))
            .map(|r| profile(r).unwrap() - reading(r).unwrap())
            .collect();
        assert!(gaps[0] > 0.1 && gaps[9] > 0.0 && gaps[9] < gaps[0] / 20.0);
        assert!(gaps.windows(2).all(|pair| pair[1] < pair[0]));
        assert_eq!(volume(COVER), Some(1.0));
    }
}
