pub mod frames;

use mrly_error::{Result, overflow_error, value_error};
use mrly_num::factor::gcd;
use mrly_tensor::Tensor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Frac {
    pub num: i64,
    pub den: i64,
}

fn reduce(num: i128, den: i128) -> Result<Frac> {
    if den == 0 {
        return value_error("a fraction needs a nonzero denominator.");
    }
    let sign = if den < 0 { -1 } else { 1 };
    let g = (gcd(num.unsigned_abs(), den.unsigned_abs()) as i128).max(1);
    let (num, den) = (sign * num / g, sign * den / g);
    match (i64::try_from(num), i64::try_from(den)) {
        (Ok(num), Ok(den)) => Ok(Frac { num, den }),
        _ => overflow_error(format!("the fraction {num} over {den} passes i64.")),
    }
}

impl Frac {
    pub fn new(num: i64, den: i64) -> Result<Frac> {
        reduce(num as i128, den as i128)
    }
    pub fn whole(num: i64) -> Frac {
        Frac { num, den: 1 }
    }
    pub fn plus(self, other: Frac) -> Result<Frac> {
        reduce(
            self.num as i128 * other.den as i128 + other.num as i128 * self.den as i128,
            self.den as i128 * other.den as i128,
        )
    }
    pub fn minus(self, other: Frac) -> Result<Frac> {
        reduce(
            self.num as i128 * other.den as i128 - other.num as i128 * self.den as i128,
            self.den as i128 * other.den as i128,
        )
    }
    pub fn times(self, other: Frac) -> Result<Frac> {
        reduce(
            self.num as i128 * other.num as i128,
            self.den as i128 * other.den as i128,
        )
    }
}

fn lcm(a: i64, b: i64) -> Result<i64> {
    let g = gcd(a.unsigned_abs() as u128, b.unsigned_abs() as u128) as i128;
    match i64::try_from(a as i128 / g * b as i128) {
        Ok(value) => Ok(value),
        Err(_) => overflow_error(format!("the common multiple of {a} and {b} passes i64.")),
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Half {
    pub normal: Vec<i64>,
    pub offset: Frac,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Shape {
    Ball {
        center: Vec<Frac>,
        radius: Frac,
    },
    Polytope {
        walls: Vec<Half>,
    },
    Anti(Box<Shape>),
    Window {
        inner: Box<Shape>,
        low: Vec<Frac>,
        high: Vec<Frac>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Region {
    Out = 0,
    Cut = 1,
    In = 2,
}

impl Region {
    pub fn flip(self) -> Region {
        match self {
            Region::Out => Region::In,
            Region::Cut => Region::Cut,
            Region::In => Region::Out,
        }
    }
}

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

fn ball_scale(center: &[Frac], radius: Frac) -> Result<i64> {
    center.iter().try_fold(radius.den, |l, c| lcm(l, c.den))
}

fn classify_ball(center: &[Frac], radius: Frac, side: usize, index: &[usize]) -> Result<Region> {
    let l = ball_scale(center, radius)? as i128;
    let s = side as i128;
    let r = radius.num as i128 * (l / radius.den as i128) * s;
    if r < 0 {
        return Ok(Region::Out);
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
    Ok(if far <= rr {
        Region::In
    } else if near > rr {
        Region::Out
    } else {
        Region::Cut
    })
}

fn within(low: &[Frac], high: &[Frac], side: usize, index: &[usize]) -> bool {
    let s = side as i128;
    low.iter().zip(high).zip(index).all(|((lo, hi), &i)| {
        let i = i as i128;
        lo.num as i128 * s <= i * lo.den as i128 && (i + 1) * hi.den as i128 <= hi.num as i128 * s
    })
}

pub fn classify(shape: &Shape, side: usize, index: &[usize]) -> Result<Region> {
    match shape {
        Shape::Ball { center, radius } => classify_ball(center, *radius, side, index),
        Shape::Polytope { walls } => {
            let mut cut = false;
            for wall in walls {
                match classify_half(wall, side, index) {
                    Region::Out => return Ok(Region::Out),
                    Region::Cut => cut = true,
                    Region::In => {}
                }
            }
            Ok(if cut { Region::Cut } else { Region::In })
        }
        Shape::Anti(inner) => Ok(classify(inner, side, index)?.flip()),
        Shape::Window { inner, low, high } => match within(low, high, side, index) {
            true => classify(inner, side, index),
            false => Ok(Region::Out),
        },
    }
}

pub fn regions(shape: &Shape, dims: &[usize]) -> Result<Tensor> {
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
        out.put(flat, i64::from(classify(shape, side, &index)? as u8));
    }
    Ok(out)
}

pub fn shapes(dimension: usize) -> Vec<String> {
    let mut out = vec!["ball", "box", "diamond"];
    if dimension == 2 {
        out.extend(["triangle", "octagon"]);
    }
    if dimension == 3 {
        out.extend(["octahedron", "tetrahedron", "pyramid"]);
    }
    out.into_iter().map(String::from).collect()
}

fn half(normal: Vec<i64>, offset: Frac) -> Half {
    Half { normal, offset }
}

fn axis_normal(dimension: usize, axis: usize, sign: i64) -> Vec<i64> {
    let mut n = vec![0i64; dimension];
    n[axis] = sign;
    n
}

fn box_shape(dimension: usize, radius: Frac) -> Result<Shape> {
    let h = Frac::new(1, 2)?;
    let mut walls = Vec::with_capacity(2 * dimension);
    for axis in 0..dimension {
        walls.push(half(axis_normal(dimension, axis, 1), h.plus(radius)?));
        walls.push(half(axis_normal(dimension, axis, -1), radius.minus(h)?));
    }
    Ok(Shape::Polytope { walls })
}

fn diamond_walls(dimension: usize, radius: Frac) -> Result<Vec<Half>> {
    (0..1usize << dimension)
        .map(|bits| {
            let normal: Vec<i64> = (0..dimension)
                .map(|axis| if (bits >> axis) & 1 == 1 { -1 } else { 1 })
                .collect();
            let toward = Frac::new(normal.iter().sum(), 2)?;
            Ok(half(normal, toward.plus(radius)?))
        })
        .collect()
}

fn diamond(dimension: usize, radius: Frac) -> Result<Shape> {
    Ok(Shape::Polytope {
        walls: diamond_walls(dimension, radius)?,
    })
}

fn triangle(radius: Frac) -> Result<Shape> {
    let h = Frac::new(1, 2)?;
    let low = radius.minus(Frac::new(3, 2)?)?;
    Ok(Shape::Polytope {
        walls: vec![
            half(vec![1, 0], h.plus(radius)?),
            half(vec![-1, -2], low),
            half(vec![-1, 2], h.plus(radius)?),
        ],
    })
}

fn octagon(radius: Frac) -> Result<Shape> {
    let cut = radius.times(Frac::new(3, 2)?)?;
    let mut walls = match box_shape(2, radius)? {
        Shape::Polytope { walls } => walls,
        _ => unreachable!(),
    };
    walls.extend(diamond_walls(2, cut)?);
    Ok(Shape::Polytope { walls })
}

fn tetrahedron(radius: Frac) -> Result<Shape> {
    let normals = [[1, 1, 1], [-1, -1, 1], [-1, 1, -1], [1, -1, -1]];
    let walls: Result<Vec<Half>> = normals
        .iter()
        .map(|n| {
            let toward = Frac::new(n.iter().sum(), 2)?;
            Ok(half(n.to_vec(), toward.plus(radius)?))
        })
        .collect();
    Ok(Shape::Polytope { walls: walls? })
}

fn pyramid(radius: Frac) -> Result<Shape> {
    let h = Frac::new(1, 2)?;
    let low = radius.minus(Frac::new(3, 2)?)?;
    Ok(Shape::Polytope {
        walls: vec![
            half(vec![1, 0, 0], h.plus(radius)?),
            half(vec![-1, -2, 0], low),
            half(vec![-1, 2, 0], h.plus(radius)?),
            half(vec![-1, 0, -2], low),
            half(vec![-1, 0, 2], h.plus(radius)?),
        ],
    })
}

pub fn named(name: &str, dimension: usize, radius: Frac) -> Result<Shape> {
    match (name, dimension) {
        ("ball", _) => Ok(Shape::Ball {
            center: vec![Frac::new(1, 2)?; dimension],
            radius,
        }),
        ("box", _) => box_shape(dimension, radius),
        ("diamond", _) => diamond(dimension, radius),
        ("triangle", 2) => triangle(radius),
        ("octagon", 2) => octagon(radius),
        ("octahedron", 3) => diamond(3, radius),
        ("tetrahedron", 3) => tetrahedron(radius),
        ("pyramid", 3) => pyramid(radius),
        ("hexagon", 2) | ("equilateral", 2) => value_error(
            "a regular hexagon or equilateral triangle has irrational walls on the grid; no exact crop exists.",
        ),
        _ => value_error(format!("unknown shape {name} in dimension {dimension}.")),
    }
}

fn moved(shape: &Shape, offsets: &[Frac], short: Frac, unit: Frac) -> Result<Shape> {
    let at = |c: Frac, o: Frac| -> Result<Frac> { short.times(c)?.plus(o)?.times(unit) };
    let axes = |n: usize| match n == offsets.len() {
        true => Ok(()),
        false => value_error(format!(
            "a shape of dimension {n} does not fit {} extents.",
            offsets.len()
        )),
    };
    let corner = |corner: &[Frac]| -> Result<Vec<Frac>> {
        axes(corner.len())?;
        corner
            .iter()
            .zip(offsets)
            .map(|(&c, &o)| at(c, o))
            .collect()
    };
    Ok(match shape {
        Shape::Ball { center, radius } => Shape::Ball {
            center: corner(center)?,
            radius: short.times(*radius)?.times(unit)?,
        },
        Shape::Polytope { walls } => Shape::Polytope {
            walls: walls
                .iter()
                .map(|wall| {
                    axes(wall.normal.len())?;
                    let shift = wall
                        .normal
                        .iter()
                        .zip(offsets)
                        .try_fold(Frac::whole(0), |sum, (&n, &o)| {
                            sum.plus(Frac::whole(n).times(o)?)
                        })?;
                    Ok(half(wall.normal.clone(), at(wall.offset, shift)?))
                })
                .collect::<Result<_>>()?,
        },
        Shape::Anti(inner) => Shape::Anti(Box::new(moved(inner, offsets, short, unit)?)),
        Shape::Window { inner, low, high } => Shape::Window {
            inner: Box::new(moved(inner, offsets, short, unit)?),
            low: corner(low)?,
            high: corner(high)?,
        },
    })
}

pub fn fit(shape: &Shape, extents: &[usize]) -> Result<Shape> {
    let Some(&short) = extents.iter().min() else {
        return value_error("a fit needs at least one extent.");
    };
    if short == 0 {
        return value_error("a fit needs every extent at least one.");
    }
    if extents.iter().all(|&e| e == short) {
        return Ok(shape.clone());
    }
    let whole = |n: usize| match i64::try_from(n) {
        Ok(n) => Ok(Frac::whole(n)),
        Err(_) => overflow_error(format!("the extent {n} passes i64.")),
    };
    let offsets: Vec<Frac> = extents
        .iter()
        .map(|&e| whole((e - short) / 2))
        .collect::<Result<_>>()?;
    let short = whole(short)?;
    let unit = Frac::new(1, whole(extents[0])?.num)?;
    let window = Shape::Window {
        inner: Box::new(shape.clone()),
        low: vec![Frac::whole(0); extents.len()],
        high: vec![Frac::whole(1); extents.len()],
    };
    moved(&window, &offsets, short, unit)
}

pub fn reach(name: &str, dimension: usize, den: u32) -> Result<Option<u32>> {
    if den == 0 {
        return value_error("a reach needs a den of at least one.");
    }
    let corner = vec![0usize; dimension];
    let covers = |k: u32| -> Result<bool> {
        let shape = named(name, dimension, Frac::new(i64::from(k), i64::from(den))?)?;
        Ok(classify(&shape, 1, &corner)? == Region::In)
    };
    if !covers(den)? {
        return Ok(None);
    }
    let (mut lo, mut hi) = (0, den);
    while hi - lo > 1 {
        let mid = lo + (hi - lo) / 2;
        match covers(mid)? {
            true => hi = mid,
            false => lo = mid,
        }
    }
    Ok(Some(hi))
}

pub fn crop(types: &Tensor, shape: &Shape, keep_cut: bool) -> Result<Tensor> {
    let map = regions(shape, &types.shape)?;
    let mut out = types.clone();
    for flat in 0..out.size() {
        let region = map.at(flat) as u8;
        let keep = region == Region::In as u8 || (keep_cut && region == Region::Cut as u8);
        if !keep {
            out.put(flat, 0);
        }
    }
    Ok(out)
}

pub const REFINE_LIMIT: usize = 20_000_000;

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
                return value_error(format!("refine output would exceed {REFINE_LIMIT} cells."));
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
        let region = classify(shape, side, &index)?;
        let keep = region == Region::In || (keep_cut && region == Region::Cut);
        if keep {
            out.put(flat, value);
        }
    }
    Ok(out)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShapeCensus {
    pub cells: [usize; 3],
    pub filled: [usize; 3],
}

pub fn census(shape: &Shape, types: &Tensor) -> Result<ShapeCensus> {
    let map = regions(shape, &types.shape)?;
    let mut out = ShapeCensus {
        cells: [0; 3],
        filled: [0; 3],
    };
    for flat in 0..types.size() {
        let region = map.at(flat) as usize;
        out.cells[region] += 1;
        if types.at(flat) != 0 {
            out.filled[region] += 1;
        }
    }
    Ok(out)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RadialCounts {
    pub seen: u64,
    pub inside: u64,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellBox {
    pub x: u64,
    pub y: u64,
    pub seat: usize,
    pub parent: usize,
    pub live: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shell {
    pub radius: u64,
    pub number: u64,
    pub levels: Vec<Vec<ShellBox>>,
    pub orphans: usize,
}

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
