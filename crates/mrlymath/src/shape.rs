use mrlycore::errors::{value_error, Result};
use mrlycore::tensor::Tensor;
use mrlynum::classics::gcd;

// FRACTIONS

/// An exact rational number with a positive, reduced denominator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Frac {
    /// The numerator, carrying the sign.
    pub num: i64,
    /// The denominator, always positive.
    pub den: i64,
}

fn reduce(num: i128, den: i128) -> Frac {
    assert!(den != 0, "Frac denominator must be nonzero");
    let sign = if den < 0 { -1 } else { 1 };
    let g = (gcd(num.unsigned_abs(), den.unsigned_abs()) as i128).max(1);
    Frac {
        num: i64::try_from(sign * num / g).expect("Frac numerator overflow"),
        den: i64::try_from(sign * den / g).expect("Frac denominator overflow"),
    }
}

impl Frac {
    /// Builds the reduced fraction num over den, panicking on a zero denominator.
    pub fn new(num: i64, den: i64) -> Frac {
        reduce(num as i128, den as i128)
    }
    /// Wraps an integer as a fraction over one.
    pub fn whole(num: i64) -> Frac {
        Frac { num, den: 1 }
    }
}

impl std::ops::Add for Frac {
    type Output = Frac;
    /// Returns the exact sum, panicking when the reduced result overflows i64.
    fn add(self, other: Frac) -> Frac {
        reduce(
            self.num as i128 * other.den as i128 + other.num as i128 * self.den as i128,
            self.den as i128 * other.den as i128,
        )
    }
}

impl std::ops::Sub for Frac {
    type Output = Frac;
    /// Returns the exact difference, panicking when the reduced result overflows i64.
    fn sub(self, other: Frac) -> Frac {
        reduce(
            self.num as i128 * other.den as i128 - other.num as i128 * self.den as i128,
            self.den as i128 * other.den as i128,
        )
    }
}

impl std::ops::Mul for Frac {
    type Output = Frac;
    /// Returns the exact product, panicking when the reduced result overflows i64.
    fn mul(self, other: Frac) -> Frac {
        reduce(
            self.num as i128 * other.num as i128,
            self.den as i128 * other.den as i128,
        )
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    let g = gcd(a.unsigned_abs() as u128, b.unsigned_abs() as u128) as i128;
    i64::try_from(a as i128 / g * b as i128).expect("lcm overflow")
}

// SHAPES

/// A closed half-space: the points x with normal dot x at most offset.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Half {
    /// The integer outward normal.
    pub normal: Vec<i64>,
    /// The rational offset the linear form stays under.
    pub offset: Frac,
}

/// An exact region of the unit box, scaled onto the lattice by the side.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Shape {
    /// The closed ball of the given rational center and radius.
    Ball {
        /// The rational center, one coordinate per axis.
        center: Vec<Frac>,
        /// The rational radius.
        radius: Frac,
    },
    /// The intersection of closed half-spaces.
    Polytope {
        /// The bounding walls.
        walls: Vec<Half>,
    },
    /// The complement of the inner shape: In and Out swap, Cut stays.
    Anti(Box<Shape>),
}

/// Where one lattice cell sits relative to a shape.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Region {
    /// The cell lies fully outside the shape.
    Out = 0,
    /// The cell crosses the shape's boundary.
    Cut = 1,
    /// The cell lies fully inside the shape.
    In = 2,
}

impl Region {
    /// Swaps In and Out, keeping Cut.
    pub fn flip(self) -> Region {
        match self {
            Region::Out => Region::In,
            Region::Cut => Region::Cut,
            Region::In => Region::Out,
        }
    }
}

// CLASSIFICATION

fn classify_half(wall: &Half, side: usize, index: &[usize]) -> Region {
    let bound = wall.offset.num as i128 * side as i128;
    let den = wall.offset.den as i128;
    let mut low: i128 = 0;
    let mut high: i128 = 0;
    for (axis, &n) in wall.normal.iter().enumerate() {
        let n = n as i128;
        let i = index[axis] as i128;
        if n >= 0 {
            low += n * i;
            high += n * (i + 1);
        } else {
            low += n * (i + 1);
            high += n * i;
        }
    }
    if high * den <= bound {
        Region::In
    } else if low * den > bound {
        Region::Out
    } else {
        Region::Cut
    }
}

fn ball_scale(center: &[Frac], radius: Frac) -> i64 {
    center.iter().fold(radius.den, |l, c| lcm(l, c.den))
}

fn classify_ball(center: &[Frac], radius: Frac, side: usize, index: &[usize]) -> Region {
    let l = ball_scale(center, radius) as i128;
    let s = side as i128;
    let r = radius.num as i128 * (l / radius.den as i128) * s;
    if r < 0 {
        return Region::Out;
    }
    let rr = r * r;
    let mut near: i128 = 0;
    let mut far: i128 = 0;
    for (axis, c) in center.iter().enumerate() {
        let cc = c.num as i128 * (l / c.den as i128) * s;
        let lo = index[axis] as i128 * l;
        let hi = lo + l;
        let dn = cc.clamp(lo, hi) - cc;
        near += dn * dn;
        let df = (lo - cc).abs().max((hi - cc).abs());
        far += df * df;
    }
    if far <= rr {
        Region::In
    } else if near > rr {
        Region::Out
    } else {
        Region::Cut
    }
}

/// Places one lattice cell relative to the shape, exactly, with no floats.
///
/// The cell at the index occupies the closed box from the index to the index plus one on each axis, and the shape's unit-box coordinates are scaled by the side.
///
/// A polytope is judged wall by wall: Out means some wall excludes the cell, In means every wall contains it, and Cut means neither - so a cell that touches each wall's feasible side separately reads Cut even when the wall intersection misses it, a conservative call that never mislabels In or Out.
pub fn classify(shape: &Shape, side: usize, index: &[usize]) -> Region {
    match shape {
        Shape::Ball { center, radius } => classify_ball(center, *radius, side, index),
        Shape::Polytope { walls } => {
            let mut cut = false;
            for wall in walls {
                match classify_half(wall, side, index) {
                    Region::Out => return Region::Out,
                    Region::Cut => cut = true,
                    Region::In => {}
                }
            }
            if cut {
                Region::Cut
            } else {
                Region::In
            }
        }
        Shape::Anti(inner) => classify(inner, side, index).flip(),
    }
}

/// Classifies every cell of the grid, packing Out, Cut and In as 0, 1 and 2; the first extent sets the lattice side.
pub fn regions(shape: &Shape, dims: &[usize]) -> Tensor {
    let side = dims.first().copied().unwrap_or(0);
    let rank = dims.len();
    let mut out = Tensor::new(dims.to_vec());
    let mut index = vec![0usize; rank];
    for flat in 0..out.size() {
        let mut rem = flat;
        for axis in (0..rank).rev() {
            index[axis] = rem % dims[axis];
            rem /= dims[axis];
        }
        out.bytes_mut()[flat] = classify(shape, side, &index) as u8;
    }
    out
}

// CATALOG

/// Lists the named shapes of a dimension.
pub fn shapes(dimension: usize) -> Vec<&'static str> {
    let mut out = vec!["ball", "box", "diamond"];
    if dimension == 2 {
        out.extend(["triangle", "octagon"]);
    }
    if dimension == 3 {
        out.extend(["octahedron", "tetrahedron", "pyramid"]);
    }
    out
}

fn half(normal: Vec<i64>, offset: Frac) -> Half {
    Half { normal, offset }
}

fn axis_normal(dimension: usize, axis: usize, sign: i64) -> Vec<i64> {
    let mut n = vec![0i64; dimension];
    n[axis] = sign;
    n
}

fn box_shape(dimension: usize, radius: Frac) -> Shape {
    let h = Frac::new(1, 2);
    let mut walls = Vec::with_capacity(2 * dimension);
    for axis in 0..dimension {
        walls.push(half(axis_normal(dimension, axis, 1), h + radius));
        walls.push(half(axis_normal(dimension, axis, -1), radius - h));
    }
    Shape::Polytope { walls }
}

fn diamond_walls(dimension: usize, radius: Frac) -> Vec<Half> {
    (0..1usize << dimension)
        .map(|bits| {
            let normal: Vec<i64> = (0..dimension)
                .map(|axis| if (bits >> axis) & 1 == 1 { -1 } else { 1 })
                .collect();
            let toward = Frac::new(normal.iter().sum(), 2);
            half(normal, toward + radius)
        })
        .collect()
}

fn diamond(dimension: usize, radius: Frac) -> Shape {
    Shape::Polytope {
        walls: diamond_walls(dimension, radius),
    }
}

fn triangle(radius: Frac) -> Shape {
    let h = Frac::new(1, 2);
    let low = radius - Frac::new(3, 2);
    Shape::Polytope {
        walls: vec![
            half(vec![1, 0], h + radius),
            half(vec![-1, -2], low),
            half(vec![-1, 2], h + radius),
        ],
    }
}

fn octagon(radius: Frac) -> Shape {
    let cut = radius * Frac::new(3, 2);
    let mut walls = match box_shape(2, radius) {
        Shape::Polytope { walls } => walls,
        _ => unreachable!(),
    };
    walls.extend(diamond_walls(2, cut));
    Shape::Polytope { walls }
}

fn tetrahedron(radius: Frac) -> Shape {
    let normals = [[1, 1, 1], [-1, -1, 1], [-1, 1, -1], [1, -1, -1]];
    let walls = normals
        .iter()
        .map(|n| {
            let toward = Frac::new(n.iter().sum(), 2);
            half(n.to_vec(), toward + radius)
        })
        .collect();
    Shape::Polytope { walls }
}

fn pyramid(radius: Frac) -> Shape {
    let h = Frac::new(1, 2);
    let low = radius - Frac::new(3, 2);
    Shape::Polytope {
        walls: vec![
            half(vec![1, 0, 0], h + radius),
            half(vec![-1, -2, 0], low),
            half(vec![-1, 2, 0], h + radius),
            half(vec![-1, 0, -2], low),
            half(vec![-1, 0, 2], h + radius),
        ],
    }
}

/// Builds a named shape of the dimension, centered at one half on every axis, or an error for an unknown or irrational name.
///
/// Regular hexagons and equilateral triangles have irrational walls in the grid frame, so they are excluded on purpose rather than approximated.
pub fn named(name: &str, dimension: usize, radius: Frac) -> Result<Shape> {
    match (name, dimension) {
        ("ball", _) => Ok(Shape::Ball {
            center: vec![Frac::new(1, 2); dimension],
            radius,
        }),
        ("box", _) => Ok(box_shape(dimension, radius)),
        ("diamond", _) => Ok(diamond(dimension, radius)),
        ("triangle", 2) => Ok(triangle(radius)),
        ("octagon", 2) => Ok(octagon(radius)),
        ("octahedron", 3) => Ok(diamond(3, radius)),
        ("tetrahedron", 3) => Ok(tetrahedron(radius)),
        ("pyramid", 3) => Ok(pyramid(radius)),
        ("hexagon", 2) | ("equilateral", 2) => {
            value_error("a regular hexagon or equilateral triangle has irrational walls on the grid; no exact crop exists.")
        }
        _ => value_error(format!("unknown shape {name} in dimension {dimension}.")),
    }
}

// CROPPING

/// Zeroes every cell of the design outside the shape, keeping Cut cells on request; anti-crop is Shape::Anti.
pub fn crop(types: &Tensor, shape: &Shape, keep_cut: bool) -> Tensor {
    let map = regions(shape, &types.shape);
    let mut out = types.clone();
    for flat in 0..out.size() {
        let region = map.bytes()[flat];
        let keep = region == Region::In as u8 || (keep_cut && region == Region::Cut as u8);
        if !keep {
            out.put(flat, 0);
        }
    }
    out
}

/// The refine output ceiling in cells.
pub const REFINE_LIMIT: usize = 20_000_000;

/// Replicates each design cell base to the extra per axis and keeps a sub-cell only where its own region passes, or an error past the cell ceiling.
pub fn refine(
    types: &Tensor,
    shape: &Shape,
    base: usize,
    extra: usize,
    keep_cut: bool,
) -> Result<Tensor> {
    if base < 1 {
        return value_error("base must be at least 1.");
    }
    let exp = match u32::try_from(extra) {
        Ok(e) => e,
        Err(_) => return value_error(format!("refine output would exceed {REFINE_LIMIT} cells.")),
    };
    let factor = match base.checked_pow(exp) {
        Some(f) => f,
        None => return value_error(format!("refine output would exceed {REFINE_LIMIT} cells.")),
    };
    let mut cells: usize = 1;
    let mut dims = Vec::with_capacity(types.shape.len());
    for &n in &types.shape {
        let grown = match n.checked_mul(factor) {
            Some(g) => g,
            None => {
                return value_error(format!("refine output would exceed {REFINE_LIMIT} cells."))
            }
        };
        cells = match cells.checked_mul(grown) {
            Some(c) if c <= REFINE_LIMIT => c,
            _ => return value_error(format!("refine output would exceed {REFINE_LIMIT} cells.")),
        };
        dims.push(grown);
    }
    let side = dims.first().copied().unwrap_or(0);
    let rank = dims.len();
    let mut out = Tensor::typed(dims.clone(), types.dtype());
    let mut index = vec![0usize; rank];
    let mut parent = vec![0usize; rank];
    for flat in 0..out.size() {
        let mut rem = flat;
        for axis in (0..rank).rev() {
            index[axis] = rem % dims[axis];
            rem /= dims[axis];
        }
        for axis in 0..rank {
            parent[axis] = index[axis] / factor;
        }
        let value = types.at(types.index(&parent));
        if value == 0 {
            continue;
        }
        let region = classify(shape, side, &index);
        let keep = region == Region::In || (keep_cut && region == Region::Cut);
        if keep {
            out.put(flat, value);
        }
    }
    Ok(out)
}

// CENSUS

/// The per-region tallies of a shape over a design, indexed Out, Cut, In.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShapeCensus {
    /// The cell count of each region.
    pub cells: [usize; 3],
    /// The filled-cell count of each region.
    pub filled: [usize; 3],
}

/// Tallies the design's cells and filled cells per region of the shape.
pub fn census(shape: &Shape, types: &Tensor) -> ShapeCensus {
    let map = regions(shape, &types.shape);
    let mut out = ShapeCensus {
        cells: [0; 3],
        filled: [0; 3],
    };
    for flat in 0..types.size() {
        let region = map.bytes()[flat] as usize;
        out.cells[region] += 1;
        if types.at(flat) != 0 {
            out.filled[region] += 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bang::factory::create;
    use mrlycore::atoms;

    struct Lcg(u64);

    impl Lcg {
        fn next(&mut self) -> u64 {
            self.0 = self
                .0
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            self.0 >> 33
        }
        fn pick(&mut self, lo: i64, hi: i64) -> i64 {
            lo + (self.next() % (hi - lo + 1) as u64) as i64
        }
    }

    fn corners(index: &[usize]) -> Vec<Vec<i128>> {
        let d = index.len();
        (0..1usize << d)
            .map(|bits| {
                index
                    .iter()
                    .enumerate()
                    .map(|(k, &i)| (i + ((bits >> k) & 1)) as i128)
                    .collect()
            })
            .collect()
    }

    fn oracle_half(wall: &Half, side: usize, index: &[usize]) -> Region {
        let bound = wall.offset.num as i128 * side as i128;
        let den = wall.offset.den as i128;
        let mut ins = 0;
        let all = corners(index);
        for corner in &all {
            let value: i128 = wall
                .normal
                .iter()
                .zip(corner)
                .map(|(&n, &c)| n as i128 * c)
                .sum();
            if value * den <= bound {
                ins += 1;
            }
        }
        if ins == all.len() {
            Region::In
        } else if ins == 0 {
            Region::Out
        } else {
            Region::Cut
        }
    }

    fn oracle_ball(center: &[Frac], radius: Frac, side: usize, index: &[usize]) -> Region {
        let l = ball_scale(center, radius) as i128;
        let s = side as i128;
        let r = radius.num as i128 * (l / radius.den as i128) * s;
        if r < 0 {
            return Region::Out;
        }
        let scaled: Vec<i128> = center
            .iter()
            .map(|c| c.num as i128 * (l / c.den as i128) * s)
            .collect();
        let far = corners(index)
            .iter()
            .map(|corner| {
                corner
                    .iter()
                    .zip(&scaled)
                    .map(|(&p, &c)| (p * l - c) * (p * l - c))
                    .sum::<i128>()
            })
            .max()
            .unwrap();
        let near: i128 = scaled
            .iter()
            .enumerate()
            .map(|(axis, &c)| {
                let lo = index[axis] as i128 * l;
                let d = c.clamp(lo, lo + l) - c;
                d * d
            })
            .sum();
        if far <= r * r {
            Region::In
        } else if near > r * r {
            Region::Out
        } else {
            Region::Cut
        }
    }

    fn oracle(shape: &Shape, side: usize, index: &[usize]) -> Region {
        match shape {
            Shape::Ball { center, radius } => oracle_ball(center, *radius, side, index),
            Shape::Polytope { walls } => {
                let mut cut = false;
                for wall in walls {
                    match oracle_half(wall, side, index) {
                        Region::Out => return Region::Out,
                        Region::Cut => cut = true,
                        Region::In => {}
                    }
                }
                if cut {
                    Region::Cut
                } else {
                    Region::In
                }
            }
            Shape::Anti(inner) => oracle(inner, side, index).flip(),
        }
    }

    fn random_shape(rng: &mut Lcg, dimension: usize) -> Shape {
        let core = if rng.pick(0, 1) == 0 {
            Shape::Ball {
                center: (0..dimension)
                    .map(|_| Frac::new(rng.pick(-4, 8), rng.pick(1, 4)))
                    .collect(),
                radius: Frac::new(rng.pick(0, 6), rng.pick(1, 3)),
            }
        } else {
            Shape::Polytope {
                walls: (0..rng.pick(1, 4))
                    .map(|_| Half {
                        normal: (0..dimension).map(|_| rng.pick(-3, 3)).collect(),
                        offset: Frac::new(rng.pick(-6, 6), rng.pick(1, 4)),
                    })
                    .collect(),
            }
        };
        if rng.pick(0, 3) == 0 {
            Shape::Anti(Box::new(core))
        } else {
            core
        }
    }

    fn every_index(dims: &[usize]) -> Vec<Vec<usize>> {
        let size: usize = dims.iter().product();
        (0..size)
            .map(|flat| {
                let mut rem = flat;
                let mut index = vec![0usize; dims.len()];
                for axis in (0..dims.len()).rev() {
                    index[axis] = rem % dims[axis];
                    rem /= dims[axis];
                }
                index
            })
            .collect()
    }

    #[test]
    fn classify_matches_the_corner_oracle() {
        let mut rng = Lcg(9);
        for dimension in 1..=3 {
            for _ in 0..40 {
                let shape = random_shape(&mut rng, dimension);
                let side = rng.pick(1, 8) as usize;
                for index in every_index(&vec![side; dimension]) {
                    assert_eq!(
                        classify(&shape, side, &index),
                        oracle(&shape, side, &index),
                        "{shape:?} side {side} at {index:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn census_and_crops_partition_the_grid() {
        let types = create(7, 3, 2, 2, 2).unwrap();
        let ball = named("ball", 2, Frac::new(1, 2)).unwrap();
        let tally = census(&ball, &types);
        assert_eq!(tally.cells.iter().sum::<usize>(), 81);
        assert_eq!(tally.filled.iter().sum::<usize>(), 64);
        let kept = crop(&types, &ball, true);
        let anti = crop(&types, &Shape::Anti(Box::new(ball.clone())), false);
        let filled = |t: &Tensor| t.bytes().iter().filter(|&&v| v != 0).count();
        assert_eq!(filled(&kept) + filled(&anti), filled(&types));
        assert_eq!(kept.get(&[0, 0]), 0);
        assert!(filled(&kept) > 0);
    }

    #[test]
    fn diamond_crop_matches_the_closed_form() {
        let half = Frac::new(1, 2);
        for m in [1usize, 2, 3, 4, 6] {
            let ones = Tensor::full(vec![2 * m, 2 * m], 1);
            let d = named("diamond", 2, half).unwrap();
            let inside = crop(&ones, &d, false);
            assert_eq!(inside.sum() as usize, 2 * m * (m - 1), "side {}", 2 * m);
        }
    }

    #[test]
    fn sponge_ball_crop_counts() {
        let sponge = create(23, 3, 3, 2, 1).unwrap();
        assert_eq!(sponge.sum(), 20);
        let ball = named("ball", 3, Frac::new(1, 2)).unwrap();
        assert_eq!(crop(&sponge, &ball, true).sum(), 20);
        assert_eq!(crop(&sponge, &ball, false).sum(), 0);
        let tally = census(&ball, &sponge);
        assert_eq!(tally.cells, [0, 26, 1]);
    }

    #[test]
    fn carpet_ball_crop_trims_the_corners() {
        let carpet = create(7, 3, 2, 2, 2).unwrap();
        let ball = named("ball", 2, Frac::new(1, 2)).unwrap();
        let kept = crop(&carpet, &ball, true);
        assert_eq!(kept.get(&[0, 0]), 0);
        assert_eq!(kept.get(&[8, 8]), 0);
        assert!(kept.sum() > 0);
        assert!(kept.sum() < carpet.sum());
    }

    #[test]
    fn refine_of_ones_is_a_crop_at_the_finer_side() {
        let ball = named("ball", 2, Frac::new(1, 2)).unwrap();
        for keep_cut in [false, true] {
            let refined = refine(&atoms::ones_2d(2), &ball, 2, 2, keep_cut).unwrap();
            let cropped = crop(&atoms::ones_2d(8), &ball, keep_cut);
            assert_eq!(refined, cropped);
        }
    }

    #[test]
    fn refine_replicates_the_parent_under_a_covering_shape() {
        let carpet = atoms::carpet_2d(3);
        let everything = named("ball", 2, Frac::whole(2)).unwrap();
        let refined = refine(&carpet, &everything, 3, 1, false).unwrap();
        assert_eq!(refined, carpet.kron(&atoms::ones_2d(3)));
    }

    #[test]
    fn refine_guards_the_cell_ceiling() {
        let ball = named("ball", 2, Frac::new(1, 2)).unwrap();
        assert!(refine(&atoms::ones_2d(10), &ball, 10, 4, false).is_err());
        assert!(refine(&atoms::ones_3d(3), &ball, 3, 9, false).is_err());
        assert!(refine(&atoms::ones_2d(3), &ball, 0, 1, false).is_err());
    }

    #[test]
    fn named_catalog_covers_its_dimensions() {
        assert_eq!(
            shapes(2),
            vec!["ball", "box", "diamond", "triangle", "octagon"]
        );
        assert_eq!(
            shapes(3),
            vec![
                "ball",
                "box",
                "diamond",
                "octahedron",
                "tetrahedron",
                "pyramid"
            ]
        );
        let r = Frac::new(1, 2);
        for dimension in 2..=3 {
            for name in shapes(dimension) {
                assert!(named(name, dimension, r).is_ok(), "{name} d{dimension}");
            }
        }
        assert!(named("hexagon", 2, r).is_err());
        assert!(named("triangle", 3, r).is_err());
        assert!(named("blob", 2, r).is_err());
    }

    #[test]
    fn named_solids_fill_a_sane_share() {
        let r = Frac::new(1, 2);
        for (dimension, side) in [(2usize, 12usize), (3, 8)] {
            let ones = Tensor::full(vec![side; dimension], 1);
            for name in shapes(dimension) {
                let shape = named(name, dimension, r).unwrap();
                let inside = crop(&ones, &shape, false).sum();
                let touched = crop(&ones, &shape, true).sum();
                assert!(inside > 0, "{name} d{dimension} inside");
                assert!(touched >= inside, "{name} d{dimension} touched");
                assert!(touched <= ones.sum(), "{name} d{dimension} bounded");
            }
        }
    }

    #[test]
    fn frac_reduces_and_computes() {
        assert_eq!(Frac::new(2, 4), Frac::new(1, 2));
        assert_eq!(Frac::new(1, -2), Frac::new(-1, 2));
        assert_eq!(Frac::new(1, 2) + Frac::new(1, 3), Frac::new(5, 6));
        assert_eq!(Frac::new(1, 2) - Frac::new(1, 2), Frac::whole(0));
        assert_eq!(Frac::new(2, 3) * Frac::new(3, 4), Frac::new(1, 2));
    }
}
