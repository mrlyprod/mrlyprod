use crate::bang::factory;
use crate::bang::universe::Code;
use mrlycore::errors::{value_error, Result};

const PALETTE: [&str; 12] = [
    "#ff5f5f", "#ffb347", "#ffe066", "#8ce99a", "#4dd4c0", "#63c7ff", "#7c9dff", "#b28dff",
    "#ff8ad4", "#ff9f7a", "#c0e86b", "#5ad1a0",
];

const WIDEST: usize = 1 << 18;

struct Solid {
    filled: Vec<bool>,
    ranks: Vec<usize>,
    weights: Vec<usize>,
    count: usize,
    number: usize,
    level: usize,
    side: usize,
    top: usize,
}

impl Solid {
    fn new(code: Code, number: usize, level: usize, base: usize) -> Result<Solid> {
        if level < 1 {
            return value_error("level must be at least 1.");
        }
        let side = match number.checked_pow(level as u32) {
            Some(side) if side <= WIDEST => side,
            _ => return value_error(format!("the side must stay at or below {WIDEST}.")),
        };
        let pattern = factory::create(code, number, 3, base, 1)?;
        let filled: Vec<bool> = pattern.bytes().iter().map(|&byte| byte != 0).collect();
        let mut ranks = vec![0usize; filled.len()];
        let mut weights = Vec::new();
        let mut count = 0;
        for (flat, &live) in filled.iter().enumerate() {
            if !live {
                continue;
            }
            ranks[flat] = count;
            let (i, rest) = (flat / (number * number), flat % (number * number));
            weights.push(i + rest / number + rest % number);
            count += 1;
        }
        Ok(Solid {
            filled,
            ranks,
            weights,
            count,
            number,
            level,
            side,
            top: side / number,
        })
    }

    fn holds(&self, x: usize, y: usize, z: usize) -> bool {
        let (mut a, mut b, mut c) = (x, y, z);
        for _ in 0..self.level {
            let flat =
                ((a % self.number) * self.number + b % self.number) * self.number + c % self.number;
            if !self.filled[flat] {
                return false;
            }
            a /= self.number;
            b /= self.number;
            c /= self.number;
        }
        true
    }

    fn rank(&self, point: [u32; 3]) -> usize {
        let (x, y, z) = (
            point[0] as usize / self.top,
            point[1] as usize / self.top,
            point[2] as usize / self.top,
        );
        self.ranks[(x * self.number + y) * self.number + z]
    }
}

// PROFILE

/// Counts the filled cells on every diagonal plane `x + y + z = s`, for `s` in `0..=3*(side - 1)`.
///
/// The count is the coefficient of the digit polynomial, the product over the scales of the
/// pattern's weight sums, so no cell of the solid is ever built.
///
/// ```
/// let counts = mrlymath::three::profile(126, 2, 4, 2).unwrap();
/// assert_eq!(counts[15..=30].iter().copied().collect::<Vec<u128>>(), vec![81u128; 16]);
/// ```
pub fn profile(code: Code, number: usize, level: usize, base: usize) -> Result<Vec<u128>> {
    let solid = Solid::new(code, number, level, base)?;
    let span = 3 * (solid.side - 1) + 1;
    let mut poly = vec![0u128; span];
    poly[0] = 1;
    let mut step = 1usize;
    for _ in 0..level {
        let mut next = vec![0u128; span];
        for (exponent, &count) in poly.iter().enumerate() {
            if count == 0 {
                continue;
            }
            for &weight in &solid.weights {
                let slot = exponent + step * weight;
                match next[slot].checked_add(count) {
                    Some(total) => next[slot] = total,
                    None => return value_error("the slice counts overflow a u128."),
                }
            }
        }
        poly = next;
        step *= number;
    }
    Ok(poly)
}

/// Returns the first and last height a profile fills, or none when the design is empty.
pub fn support(counts: &[u128]) -> Option<(usize, usize)> {
    let low = counts.iter().position(|&count| count > 0)?;
    let high = counts.iter().rposition(|&count| count > 0)?;
    Some((low, high))
}

// SLICE

/// Lists the filled cells on the diagonal plane `x + y + z = height`, as `x, y, z` triples.
///
/// ```
/// assert_eq!(mrlymath::three::diagonal_slice(126, 2, 3, 2, 10).unwrap().len(), 27);
/// ```
pub fn slice(
    code: Code,
    number: usize,
    level: usize,
    base: usize,
    height: usize,
) -> Result<Vec<[u32; 3]>> {
    let solid = Solid::new(code, number, level, base)?;
    let last = solid.side - 1;
    let mut out = Vec::new();
    if height > 3 * last {
        return Ok(out);
    }
    for x in height.saturating_sub(2 * last)..=last.min(height) {
        let rest = height - x;
        for y in rest.saturating_sub(last)..=last.min(rest) {
            let z = rest - y;
            if solid.holds(x, y, z) {
                out.push([x as u32, y as u32, z as u32]);
            }
        }
    }
    Ok(out)
}

// PROJECTION

/// Projects a cell down the `(1,1,1)` axis: `u = (x - y)/sqrt 2`, `v = (x + y - 2z)/sqrt 6`.
pub fn project(point: [u32; 3]) -> (f64, f64) {
    let (x, y, z) = (point[0] as f64, point[1] as f64, point[2] as f64);
    ((x - y) / 2f64.sqrt(), (x + y - 2.0 * z) / 6f64.sqrt())
}

/// Returns the integer shadow `(x - y, x + y - 2z)`, the projection with its irrational scales dropped.
pub fn shadow(point: [u32; 3]) -> (i64, i64) {
    let (x, y, z) = (point[0] as i64, point[1] as i64, point[2] as i64);
    (x - y, x + y - 2 * z)
}

// SVG

/// Draws the given diagonal slices as one circle per cell, coloured by height slot and top-scale corner.
///
/// The frame is tight around the projected points and carries no background, so the drawing sits on
/// whatever the page is.
pub fn svg(
    code: Code,
    number: usize,
    level: usize,
    base: usize,
    heights: &[usize],
    scale: usize,
) -> Result<String> {
    let solid = Solid::new(code, number, level, base)?;
    let mut circles: Vec<(f64, f64, &str)> = Vec::new();
    for (slot, &height) in heights.iter().enumerate() {
        for point in slice(code, number, level, base, height)? {
            let (u, v) = project(point);
            let key = slot * solid.count.max(1) + solid.rank(point);
            circles.push((u, v, PALETTE[key % PALETTE.len()]));
        }
    }
    if circles.is_empty() {
        return value_error("nothing to render.");
    }
    let low_u = circles.iter().map(|c| c.0).fold(f64::MAX, f64::min);
    let high_u = circles.iter().map(|c| c.0).fold(f64::MIN, f64::max);
    let low_v = circles.iter().map(|c| c.1).fold(f64::MAX, f64::min);
    let high_v = circles.iter().map(|c| c.1).fold(f64::MIN, f64::max);
    let radius = 0.35 * scale as f64;
    let width = (high_u - low_u) * scale as f64 + 2.0 * radius;
    let height = (high_v - low_v) * scale as f64 + 2.0 * radius;
    let mut out = vec![format!(
        "<svg width=\"{width:.2}\" height=\"{height:.2}\" viewBox=\"0 0 {width:.2} {height:.2}\" xmlns=\"http://www.w3.org/2000/svg\">"
    )];
    for (u, v, fill) in circles {
        let cx = (u - low_u) * scale as f64 + radius;
        let cy = (high_v - v) * scale as f64 + radius;
        out.push(format!(
            "<circle cx=\"{cx:.2}\" cy=\"{cy:.2}\" r=\"{radius:.2}\" fill=\"{fill}\"/>"
        ));
    }
    out.push("</svg>".to_string());
    Ok(out.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeMap, HashSet};

    fn scan(code: Code, number: usize, level: usize, base: usize) -> Vec<Vec<[u32; 3]>> {
        let cell = crate::three::create(code, number, level, base).unwrap();
        let grid = cell.types();
        let side = grid.shape[0];
        let mut out = vec![Vec::new(); 3 * (side - 1) + 1];
        for (flat, &site) in grid.bytes().iter().enumerate() {
            if site == 0 {
                continue;
            }
            let (x, rest) = (flat / (side * side), flat % (side * side));
            let (y, z) = (rest / side, rest % side);
            out[x + y + z].push([x as u32, y as u32, z as u32]);
        }
        out
    }

    fn digit_build(level: usize) -> Vec<[u32; 3]> {
        let corners = factory::code_to_corners(126, 3, 2).unwrap();
        let mut points = vec![[0u32, 0, 0]];
        for _ in 0..level {
            let mut next = Vec::with_capacity(points.len() * corners.len());
            for point in &points {
                for corner in &corners {
                    next.push([
                        2 * point[0] + corner[0] as u32,
                        2 * point[1] + corner[1] as u32,
                        2 * point[2] + corner[2] as u32,
                    ]);
                }
            }
            points = next;
        }
        points
    }

    fn constant_gasket(level: usize, weight_two: bool) -> HashSet<[u32; 3]> {
        let corners = if weight_two {
            [[0u32, 1, 1], [1, 0, 1], [1, 1, 0]]
        } else {
            [[1u32, 0, 0], [0, 1, 0], [0, 0, 1]]
        };
        let mut set: HashSet<[u32; 3]> = HashSet::new();
        set.insert([0, 0, 0]);
        for k in 0..level {
            let mut next = HashSet::new();
            for point in &set {
                for corner in &corners {
                    next.insert([
                        point[0] + (corner[0] << k),
                        point[1] + (corner[1] << k),
                        point[2] + (corner[2] << k),
                    ]);
                }
            }
            set = next;
        }
        set
    }

    fn odd_trinomials(layer: usize) -> usize {
        let width = layer + 1;
        let mut current = vec![0u8; width * width];
        current[0] = 1;
        for step in 1..=layer {
            let mut next = vec![0u8; width * width];
            for i in 0..=step {
                for j in 0..=step - i {
                    let mut sum = current[i * width + j];
                    if i > 0 {
                        sum += current[(i - 1) * width + j];
                    }
                    if j > 0 {
                        sum += current[i * width + j - 1];
                    }
                    next[i * width + j] = sum % 2;
                }
            }
            current = next;
        }
        current.iter().filter(|&&value| value == 1).count()
    }

    fn det3(m: [[i64; 3]; 3]) -> i64 {
        m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
            - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    }

    #[test]
    fn the_slices_agree_with_a_scan_of_the_solid() {
        for code in [23u128, 105, 126, 127] {
            for number in [2usize, 3] {
                for level in 1..3 {
                    let scanned = scan(code, number, level, 2);
                    let counts = profile(code, number, level, 2).unwrap();
                    assert_eq!(counts.len(), scanned.len());
                    for (height, wanted) in scanned.iter().enumerate() {
                        assert_eq!(
                            counts[height],
                            wanted.len() as u128,
                            "code {code} number {number} level {level} height {height}"
                        );
                        let mut got = slice(code, number, level, 2, height).unwrap();
                        let mut want = wanted.clone();
                        got.sort_unstable();
                        want.sort_unstable();
                        assert_eq!(got, want, "code {code} number {number} level {level}");
                    }
                }
            }
        }
    }

    #[test]
    fn the_digit_build_pairs_with_every_slice_to_level_eight() {
        for level in 1..=8usize {
            let last = (1usize << level) - 1;
            let mut buckets: Vec<Vec<[u32; 3]>> = vec![Vec::new(); 3 * last + 1];
            for point in digit_build(level) {
                buckets[(point[0] + point[1] + point[2]) as usize].push(point);
            }
            let counts = profile(126, 2, level, 2).unwrap();
            assert_eq!(counts.len(), buckets.len());
            for (height, bucket) in buckets.iter_mut().enumerate() {
                assert_eq!(
                    counts[height],
                    bucket.len() as u128,
                    "level {level} height {height}"
                );
                let mut got = slice(126, 2, level, 2, height).unwrap();
                got.sort_unstable();
                bucket.sort_unstable();
                assert_eq!(got, *bucket, "level {level} height {height}");
            }
        }
    }

    #[test]
    fn every_slice_of_one_two_six_holds_three_to_the_level() {
        for level in 1..=14usize {
            let counts = profile(126, 2, level, 2).unwrap();
            let (low, high) = support(&counts).unwrap();
            assert_eq!((low, high), ((1 << level) - 1, (1 << (level + 1)) - 2));
            let wanted = 3u128.pow(level as u32);
            assert!(counts[low..=high].iter().all(|&count| count == wanted));
            assert_eq!(counts.iter().sum::<u128>(), 6u128.pow(level as u32));
        }
    }

    #[test]
    fn the_two_central_heights_carry_two_times_three_to_the_level() {
        for (index, &total) in [18u128, 54, 162, 486, 1458, 4374, 13122].iter().enumerate() {
            let level = index + 2;
            let counts = profile(126, 2, level, 2).unwrap();
            let low = ((1usize << level) - 1) + (1 << (level - 1)) - 1;
            assert_eq!(counts[low] + counts[low + 1], total);
            assert_eq!(counts[low], 3u128.pow(level as u32));
        }
    }

    #[test]
    fn the_central_union_splits_evenly_by_coordinate_order() {
        let orders = [
            [0, 1, 2],
            [0, 2, 1],
            [1, 0, 2],
            [1, 2, 0],
            [2, 0, 1],
            [2, 1, 0],
        ];
        for level in 2..=8usize {
            let low = ((1usize << level) - 1) + (1 << (level - 1)) - 1;
            let mut union = slice(126, 2, level, 2, low).unwrap();
            union.extend(slice(126, 2, level, 2, low + 1).unwrap());
            let mut classes = [0usize; 6];
            let mut ties = 0;
            for point in &union {
                if point[0] == point[1] || point[1] == point[2] || point[0] == point[2] {
                    ties += 1;
                    continue;
                }
                for (index, order) in orders.iter().enumerate() {
                    if point[order[0]] < point[order[1]] && point[order[1]] < point[order[2]] {
                        classes[index] += 1;
                    }
                }
            }
            assert_eq!(ties, 6);
            let each = 3usize.pow(level as u32 - 1) - 1;
            assert!(classes.iter().all(|&size| size == each), "level {level}");
            assert_eq!(union.len(), 2 * 3usize.pow(level as u32));
        }
    }

    #[test]
    fn the_central_union_is_six_pieces_of_seven_two_nine_at_level_seven() {
        let low = 127 + 63;
        let mut pieces: BTreeMap<(usize, usize), usize> = BTreeMap::new();
        let mut shadows = HashSet::new();
        let mut total = 0;
        for (slot, height) in [low, low + 1].into_iter().enumerate() {
            for point in slice(126, 2, 7, 2, height).unwrap() {
                let corner = ((point[0] >> 6) * 4 + (point[1] >> 6) * 2 + (point[2] >> 6)) as usize;
                *pieces.entry((slot, corner)).or_default() += 1;
                shadows.insert(shadow(point));
                total += 1;
            }
        }
        assert_eq!(total, 4374);
        assert_eq!(pieces.len(), 6);
        assert!(pieces.values().all(|&size| size == 729));
        assert_eq!(shadows.len(), 4374);
    }

    #[test]
    fn the_central_union_is_six_gaskets_from_the_level_below() {
        for level in 2..=8usize {
            let base = (1usize << level) - 1;
            let half = 1usize << (level - 1);
            let mut pieces: Vec<HashSet<[u32; 3]>> = Vec::new();
            let mut slices: HashSet<[u32; 3]> = HashSet::new();
            for offset in [half - 1, half] {
                let gasket = constant_gasket(level - 1, offset & 1 == 1);
                let corners = if (offset >> (level - 1)) & 1 == 1 {
                    [[0u32, 1, 1], [1, 0, 1], [1, 1, 0]]
                } else {
                    [[1u32, 0, 0], [0, 1, 0], [0, 0, 1]]
                };
                for corner in &corners {
                    pieces.push(
                        gasket
                            .iter()
                            .map(|point| {
                                [
                                    corner[0] * half as u32 + point[0],
                                    corner[1] * half as u32 + point[1],
                                    corner[2] * half as u32 + point[2],
                                ]
                            })
                            .collect(),
                    );
                }
                slices.extend(slice(126, 2, level, 2, base + offset).unwrap());
            }
            assert_eq!(pieces.len(), 6);
            let each = 3usize.pow(level as u32 - 1);
            assert!(
                pieces.iter().all(|piece| piece.len() == each),
                "level {level}"
            );
            for (index, piece) in pieces.iter().enumerate() {
                for other in &pieces[index + 1..] {
                    assert!(piece.is_disjoint(other), "level {level}");
                }
            }
            let union: HashSet<[u32; 3]> = pieces.iter().flatten().copied().collect();
            assert_eq!(union.len(), 2 * 3usize.pow(level as u32));
            assert_eq!(union, slices, "level {level}");
        }
    }

    #[test]
    fn the_central_union_carries_the_order_twelve_symmetry() {
        let orders = [
            [0, 1, 2],
            [0, 2, 1],
            [1, 0, 2],
            [1, 2, 0],
            [2, 0, 1],
            [2, 1, 0],
        ];
        for level in 2..=8usize {
            let low = ((1usize << level) - 1) + (1 << (level - 1)) - 1;
            let below: HashSet<[u32; 3]> =
                slice(126, 2, level, 2, low).unwrap().into_iter().collect();
            let above: HashSet<[u32; 3]> = slice(126, 2, level, 2, low + 1)
                .unwrap()
                .into_iter()
                .collect();
            let union: HashSet<[u32; 3]> = below.union(&above).copied().collect();
            for order in &orders {
                let turned: HashSet<[u32; 3]> = union
                    .iter()
                    .map(|point| [point[order[0]], point[order[1]], point[order[2]]])
                    .collect();
                assert_eq!(turned, union, "level {level}");
            }
            let last = (1u32 << level) - 1;
            let flip = |set: &HashSet<[u32; 3]>| -> HashSet<[u32; 3]> {
                set.iter()
                    .map(|point| [last - point[0], last - point[1], last - point[2]])
                    .collect()
            };
            assert_eq!(flip(&below), above, "level {level}");
            assert_eq!(flip(&above), below, "level {level}");
            let mut ties: Vec<[u32; 3]> = union
                .iter()
                .copied()
                .filter(|point| {
                    point[0] == point[1] || point[1] == point[2] || point[0] == point[2]
                })
                .collect();
            ties.sort_unstable();
            let m = (1u32 << (level - 1)) - 1;
            let mut wanted: Vec<[u32; 3]> = Vec::new();
            for triple in [[m, m, m + 1], [m, m + 1, m + 1]] {
                for order in &orders {
                    wanted.push([triple[order[0]], triple[order[1]], triple[order[2]]]);
                }
            }
            wanted.sort_unstable();
            wanted.dedup();
            assert_eq!(wanted.len(), 6);
            assert_eq!(ties, wanted, "level {level}");
        }
    }

    #[test]
    fn every_scheduled_slice_is_the_digit_gasket() {
        let ones = [[1u32, 0, 0], [0, 1, 0], [0, 0, 1]];
        let twos = [[0u32, 1, 1], [1, 0, 1], [1, 1, 0]];
        for level in 1..=6usize {
            let low = (1usize << level) - 1;
            for offset in 0..(1usize << level) {
                let mut set: HashSet<[u32; 3]> = HashSet::new();
                set.insert([0, 0, 0]);
                for k in 0..level {
                    let corners = if (offset >> k) & 1 == 1 { twos } else { ones };
                    let mut next = HashSet::new();
                    for point in &set {
                        for corner in &corners {
                            next.insert([
                                point[0] + (corner[0] << k),
                                point[1] + (corner[1] << k),
                                point[2] + (corner[2] << k),
                            ]);
                        }
                    }
                    set = next;
                }
                let got: HashSet<[u32; 3]> = slice(126, 2, level, 2, low + offset)
                    .unwrap()
                    .into_iter()
                    .collect();
                assert_eq!(got, set, "level {level} offset {offset}");
            }
        }
    }

    #[test]
    fn the_neighbours_profile_at_level_four() {
        for (code, span, nonempty, low, high) in [
            (63u128, (0usize, 30usize), 31usize, 1u128, 81u128),
            (105, (0, 30), 16, 1, 81),
            (111, (0, 30), 31, 1, 111),
            (126, (15, 30), 16, 81, 81),
            (127, (0, 30), 31, 1, 162),
        ] {
            let counts = profile(code, 2, 4, 2).unwrap();
            assert_eq!(support(&counts).unwrap(), span);
            let live: Vec<u128> = counts[span.0..=span.1]
                .iter()
                .copied()
                .filter(|&count| count > 0)
                .collect();
            assert_eq!(live.len(), nonempty, "code {code}");
            assert_eq!(*live.iter().min().unwrap(), low, "code {code}");
            assert_eq!(*live.iter().max().unwrap(), high, "code {code}");
            let flat = live.len() == span.1 - span.0 + 1 && low == high;
            assert_eq!(flat, code == 126);
        }
    }

    #[test]
    fn the_one_two_seven_cut_matches_no_closed_form() {
        for (index, &top) in [3u128, 12, 45, 162, 594, 2187].iter().enumerate() {
            let level = index + 1;
            let counts = profile(127, 2, level, 2).unwrap();
            assert_eq!(*counts.iter().max().unwrap(), top);
            assert_eq!(*counts.iter().filter(|&&count| count > 0).min().unwrap(), 1);
            assert_eq!(counts.iter().sum::<u128>(), 7u128.pow(level as u32));
            let closed = 4 * (level as u128 + 5) * 3u128.pow(level as u32 - 1);
            assert_ne!(top, closed);
        }
    }

    #[test]
    fn the_flat_slice_is_the_odd_layer_of_pascals_pyramid() {
        for (index, &wanted) in [3usize, 9, 27, 81, 243].iter().enumerate() {
            let level = index + 1;
            let low = (1usize << level) - 1;
            let points = slice(126, 2, level, 2, low).unwrap();
            assert_eq!(points.len(), 3usize.pow(level as u32));
            assert_eq!(points.len(), wanted);
            assert_eq!(odd_trinomials(low), wanted);
        }
    }

    #[test]
    fn the_flat_slice_has_pairwise_disjoint_binary_supports() {
        for level in 1..=6usize {
            let last = (1u32 << level) - 1;
            let got: HashSet<[u32; 3]> = slice(126, 2, level, 2, last as usize)
                .unwrap()
                .into_iter()
                .collect();
            let mut wanted: HashSet<[u32; 3]> = HashSet::new();
            for x in 0..=last {
                for y in 0..=last - x {
                    let z = last - x - y;
                    if x & y == 0 && y & z == 0 && x & z == 0 {
                        wanted.insert([x, y, z]);
                    }
                }
            }
            assert_eq!(got, wanted, "level {level}");
            assert_eq!(got.len(), 3usize.pow(level as u32));
        }
    }

    #[test]
    fn the_twenty_three_slices_are_three_to_the_digit_sum() {
        assert_eq!(
            &profile(23, 2, 3, 2).unwrap()[..8],
            &[1u128, 3, 3, 9, 3, 9, 9, 27]
        );
        for level in 1..=6u32 {
            let counts = profile(23, 2, level as usize, 2).unwrap();
            for (height, &count) in counts.iter().enumerate() {
                let wanted = if height < (1 << level) {
                    3u128.pow((height as u32).count_ones())
                } else {
                    0
                };
                assert_eq!(count, wanted, "level {level} height {height}");
            }
        }
    }

    #[test]
    fn the_six_cut_codes_are_canonical() {
        for code in [23u128, 63, 105, 111, 126, 127] {
            let orbit = crate::bang::universe::orbit(code, 3);
            assert_eq!(*orbit.iter().next().unwrap(), code);
        }
    }

    #[test]
    fn the_centred_corners_are_the_octahedron_axes() {
        let corners = factory::code_to_corners(126, 3, 2).unwrap();
        let doubled: Vec<[i64; 3]> = corners
            .iter()
            .map(|corner| {
                [
                    2 * corner[0] as i64 - 1,
                    2 * corner[1] as i64 - 1,
                    2 * corner[2] as i64 - 1,
                ]
            })
            .collect();
        assert_eq!(doubled.len(), 6);
        for axis in &doubled {
            assert!(doubled.contains(&[-axis[0], -axis[1], -axis[2]]));
        }
        let basis = [doubled[0], doubled[1], doubled[2]];
        let mat = [
            [basis[0][0], basis[1][0], basis[2][0]],
            [basis[0][1], basis[1][1], basis[2][1]],
            [basis[0][2], basis[1][2], basis[2][2]],
        ];
        let det = det3(mat);
        assert_eq!(det, -4);
        for axis in &doubled {
            let mut hits = 0;
            for k in 0..3 {
                let mut probe = mat;
                probe[0][k] = axis[0];
                probe[1][k] = axis[1];
                probe[2][k] = axis[2];
                let value = det3(probe);
                if value == 0 {
                    continue;
                }
                assert_eq!(value.abs(), det.abs());
                hits += 1;
            }
            assert_eq!(hits, 1);
        }
    }

    #[test]
    fn the_svg_draws_one_circle_for_every_cell() {
        let art = svg(126, 2, 3, 2, &[10, 11], 4).unwrap();
        assert_eq!(art.matches("<circle").count(), 54);
        assert!(!art.contains("<rect"));
        let fills: HashSet<&str> = art
            .split("fill=\"")
            .skip(1)
            .map(|piece| &piece[..7])
            .collect();
        assert_eq!(fills.len(), 6);
        assert!(svg(0, 2, 2, 2, &[3], 4).is_err());
        assert!(profile(126, 2, 0, 2).is_err());
    }
}
