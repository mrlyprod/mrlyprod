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

// RADIAL

/// The tallies of one design against a single integer radius about a centre.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RadialCounts {
    /// The filled cells whose own centre lies within the radius.
    pub seen: u64,
    /// The filled cells lying wholly within the radius.
    pub inside: u64,
    /// The filled cells the sphere of the radius crosses.
    pub cut: u64,
}

fn root_up(square: u64) -> u64 {
    if square == 0 {
        return 0;
    }
    let mut root = (square as f64).sqrt() as u64;
    while root.saturating_mul(root) < square {
        root += 1;
    }
    while root > 0 && (root - 1) * (root - 1) >= square {
        root -= 1;
    }
    root
}

fn shell(square: u64) -> u64 {
    root_up(square).div_ceil(2)
}

/// Counts a design's filled cells against every integer radius about one centre, in exact integer arithmetic.
///
/// Cell centres sit at the index plus one half, so every length is doubled and the centre is given in those doubled units: the lattice corner is zero on each axis and the grid centre is the side. Entry r of the result holds the filled cells whose centre lies within radius r, the filled cells whose whole cell lies within it, and the filled cells the sphere of radius r crosses; squared distances are compared as integers and never as floats.
///
/// The tallies are prefix sums over one pass of the grid: each cell lands in the smallest radius that sees it, the smallest that swallows it and the smallest that clears it, and the columns are summed afterwards.
pub fn radial_census(types: &Tensor, centre: &[i64], r_max: u64) -> Vec<RadialCounts> {
    let rank = types.shape.len();
    let width = r_max as usize + 1;
    let mut seen = vec![0u64; width];
    let mut inside = vec![0u64; width];
    let mut touch = vec![0u64; width];
    let mut index = vec![0i64; rank];
    for flat in 0..types.size() {
        if types.at(flat) != 0 {
            let (mut near, mut mid, mut far) = (0u64, 0u64, 0u64);
            for (axis, coordinate) in index.iter().enumerate() {
                let gap = (2 * coordinate + 1 - centre[axis]).unsigned_abs();
                mid += gap * gap;
                far += (gap + 1) * (gap + 1);
                near += gap.saturating_sub(1) * gap.saturating_sub(1);
            }
            for (column, square) in [(&mut seen, mid), (&mut inside, far), (&mut touch, near)] {
                let at = shell(square);
                if at < width as u64 {
                    column[at as usize] += 1;
                }
            }
        }
        for axis in (0..rank).rev() {
            index[axis] += 1;
            if (index[axis] as usize) < types.shape[axis] {
                break;
            }
            index[axis] = 0;
        }
    }
    for column in [&mut seen, &mut inside, &mut touch] {
        for r in 1..width {
            column[r] += column[r - 1];
        }
    }
    (0..width)
        .map(|r| RadialCounts {
            seen: seen[r],
            inside: inside[r],
            cut: touch[r] - inside[r],
        })
        .collect()
}

// CROSSING SHELL

/// One box of a crossing shell: where it sits, the seat it takes in its parent and whether the design keeps its path.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShellBox {
    /// The box's first coordinate in its own level's grid.
    pub x: u64,
    /// The box's second coordinate in its own level's grid.
    pub y: u64,
    /// The seat the box takes in its parent, row-major over the side, and the side squared at the root.
    pub seat: usize,
    /// The parent's place in the level above, and `usize::MAX` at the root or when the level above holds no such box.
    pub parent: usize,
    /// Whether every seat from the root down to this box is one the design keeps.
    pub live: bool,
}

/// The rooted tree of the boxes one circle crosses, level by level.
#[derive(Clone, Debug)]
pub struct Shell {
    /// The radius in cells.
    pub radius: u64,
    /// The side of a box measured in the boxes one level below it.
    pub number: u64,
    /// The boxes of level `j`, the crossed cells at `0` and the single root last, each level in the arc's own order.
    pub levels: Vec<Vec<ShellBox>>,
    /// The boxes whose parent is missing from the level above, which the crossing identity forbids.
    pub orphans: usize,
}

/// Lists the level-`level` boxes the circle of radius `radius` crosses, in the arc's own order.
///
/// A box `X` of side `number^level` is crossed when its near corner lies in the closed disc and its far corner outside it, `|X| <= radius / number^level < |X + 1|`, so the level is the whole grid's crossing shell read at that real radius. The columns telescope, the low row of one column being the high row of the next, so the walk is one pass over `floor(radius / number^level) + 1` columns and every comparison is on integers. The order runs along the arc, the first coordinate rising while the second falls, which is why a parent's children are contiguous among its own.
pub fn crossing_shell(radius: u64, number: u64, level: u32) -> Vec<(u64, u64)> {
    let scale = match number.checked_pow(level) {
        Some(scale) if scale <= radius => scale,
        _ => return vec![(0, 0)],
    };
    let square = radius * radius;
    let top = radius / scale;
    let high: Vec<u64> = (0..=top + 1)
        .map(|i| {
            let reach = scale * i;
            if reach > radius {
                0
            } else {
                (square - reach * reach).isqrt() / scale
            }
        })
        .collect();
    let mut out = Vec::with_capacity(2 * top as usize + 1);
    for i in 0..=top as usize {
        for y in (high[i + 1]..=high[i]).rev() {
            out.push((i as u64, y));
        }
    }
    out
}

/// Builds the whole crossing tree of one radius, pruned by the seats the design keeps.
///
/// The depth is the least level whose grid holds the circle inside one box, so the tree is rooted there and its leaves are the `2 * radius + 1` crossed cells. `keep` reads the design's level-one tile row-major, one flag per seat, and a box is live when every seat from the root down to it is kept, so the live leaves are exactly the crossed cells the design fills.
pub fn crossing_tree(radius: u64, number: u64, keep: &[bool]) -> Shell {
    let mut depth = 0u32;
    while number
        .checked_pow(depth)
        .is_some_and(|scale| scale <= radius)
    {
        depth += 1;
    }
    let seats = (number * number) as usize;
    let mut levels: Vec<Vec<ShellBox>> = (0..=depth)
        .map(|level| {
            crossing_shell(radius, number, level)
                .into_iter()
                .map(|(x, y)| ShellBox {
                    x,
                    y,
                    seat: seats,
                    parent: usize::MAX,
                    live: true,
                })
                .collect()
        })
        .collect();
    let mut orphans = 0;
    for level in (0..depth as usize).rev() {
        let mut start = 0usize;
        for k in 0..levels[level].len() {
            let here = levels[level][k];
            let (px, py) = (here.x / number, here.y / number);
            let mut at = start;
            while at < levels[level + 1].len()
                && (levels[level + 1][at].x, levels[level + 1][at].y) != (px, py)
            {
                at += 1;
            }
            if at == levels[level + 1].len() {
                orphans += 1;
                levels[level][k].live = false;
                continue;
            }
            start = at;
            let seat = ((here.x % number) * number + here.y % number) as usize;
            let live = levels[level + 1][at].live && keep.get(seat).copied().unwrap_or(false);
            let cell = &mut levels[level][k];
            cell.seat = seat;
            cell.parent = at;
            cell.live = live;
        }
    }
    Shell {
        radius,
        number,
        levels,
        orphans,
    }
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

    fn corner_seen(code: u128, dimension: usize, level: usize, r_max: u64) -> Vec<u64> {
        let types = create(code, 3, dimension, 2, level).unwrap();
        let table = radial_census(&types, &vec![0; dimension], r_max);
        (1..=r_max as usize).map(|r| table[r].seen).collect()
    }

    #[test]
    fn radial_census_counts_the_corner_disc() {
        assert_eq!(
            corner_seen(7, 2, 6, 12),
            [1, 3, 7, 12, 16, 22, 30, 38, 48, 63, 77, 91]
        );
        assert_eq!(corner_seen(23, 3, 4, 8), [1, 4, 13, 28, 47, 65, 95, 137]);
    }

    #[test]
    fn radial_census_does_not_depend_on_the_level() {
        let shallow = create(7, 3, 2, 2, 4).unwrap();
        let deep = create(7, 3, 2, 2, 5).unwrap();
        let reach = 80;
        let near = radial_census(&shallow, &[0, 0], reach);
        let far = radial_census(&deep, &[0, 0], reach);
        assert_eq!(near, far);
        assert!(near[reach as usize].seen > 0);
    }

    #[test]
    fn radial_census_matches_the_ball_census() {
        for (code, dimension, level) in [(7u128, 2usize, 4usize), (23, 3, 3)] {
            let types = create(code, 3, dimension, 2, level).unwrap();
            let side = types.shape[0];
            let table = radial_census(&types, &vec![0; dimension], side as u64 - 1);
            for r in [1usize, 5, 17, side - 1] {
                let ball = Shape::Ball {
                    center: vec![Frac::whole(0); dimension],
                    radius: Frac::new(r as i64, side as i64),
                };
                let tally = census(&ball, &types);
                assert_eq!(tally.filled[2] as u64, table[r].inside, "in r={r}");
                assert_eq!(tally.filled[1] as u64, table[r].cut, "cut r={r}");
                assert!(table[r].inside <= table[r].seen);
                assert!(table[r].seen <= table[r].inside + table[r].cut);
            }
        }
    }

    fn brute_shell(radius: u64, scale: u64) -> Vec<(u64, u64)> {
        let mut out = Vec::new();
        let reach = radius / scale + 1;
        for x in 0..=reach {
            for y in 0..=reach {
                let near = (scale * x).pow(2) + (scale * y).pow(2);
                let far = (scale * (x + 1)).pow(2) + (scale * (y + 1)).pow(2);
                if near <= radius * radius && radius * radius < far {
                    out.push((x, y));
                }
            }
        }
        out.sort_by_key(|&(x, y)| (x, std::cmp::Reverse(y)));
        out
    }

    #[test]
    fn crossing_shell_matches_the_brute_sweep() {
        for radius in 1..=90u64 {
            for level in 0..5u32 {
                let scale = 3u64.pow(level);
                assert_eq!(
                    crossing_shell(radius, 3, level),
                    brute_shell(radius, scale),
                    "r={radius} j={level}"
                );
            }
        }
    }

    #[test]
    fn crossing_shell_is_two_floor_plus_one() {
        for radius in 1..=400u64 {
            for level in 0..8u32 {
                let want = 2 * (radius / 3u64.pow(level)) + 1;
                assert_eq!(crossing_shell(radius, 3, level).len() as u64, want);
                let five = 2 * (radius / 5u64.pow(level.min(5))) + 1;
                assert_eq!(crossing_shell(radius, 5, level.min(5)).len() as u64, five);
            }
        }
    }

    #[test]
    fn crossing_tree_hangs_every_box_on_a_crossed_parent() {
        let tile = create(7, 3, 2, 2, 1).unwrap();
        let keep: Vec<bool> = tile.bytes().iter().map(|&b| b != 0).collect();
        for radius in 1..=120u64 {
            let tree = crossing_tree(radius, 3, &keep);
            assert_eq!(tree.orphans, 0, "r={radius}");
            assert_eq!(tree.levels.last().unwrap().len(), 1);
            assert_eq!(tree.levels[0].len() as u64, 2 * radius + 1);
            for level in 0..tree.levels.len() - 1 {
                for cell in &tree.levels[level] {
                    let parent = tree.levels[level + 1][cell.parent];
                    assert_eq!((parent.x, parent.y), (cell.x / 3, cell.y / 3));
                    assert_eq!(cell.seat, ((cell.x % 3) * 3 + cell.y % 3) as usize);
                    assert!(!cell.live || parent.live);
                }
            }
        }
    }

    #[test]
    fn the_live_leaves_are_the_designs_crossed_cells() {
        for code in [7u128, 5, 11, 15] {
            let tile = create(code, 3, 2, 2, 1).unwrap();
            let keep: Vec<bool> = tile.bytes().iter().map(|&b| b != 0).collect();
            for depth in 1..=4usize {
                let grid = create(code, 3, 2, 2, depth).unwrap();
                let side = 3u64.pow(depth as u32);
                let table = radial_census(&grid, &[0, 0], side - 1);
                for radius in side / 3..side {
                    let tree = crossing_tree(radius.max(1), 3, &keep);
                    assert_eq!(tree.levels.len(), depth + 1, "code={code} r={radius}");
                    let live = tree.levels[0].iter().filter(|cell| cell.live).count() as u64;
                    assert_eq!(live, table[radius as usize].cut, "code={code} r={radius}");
                }
            }
        }
    }
}
