use mrly_color::Color;
use mrly_color::{BLUE, BROWN, CYAN, GREEN, INDIGO, MINT, ORANGE, PINK, PURPLE, RED, TEAL, YELLOW};
use mrly_error::{Result, value_error};
use mrly_math::code::Code;
use mrly_math::factory;
use mrly_sequences::counts::profile_of_tile;

const PALETTE: [Color; 12] = [
    RED, ORANGE, YELLOW, GREEN, MINT, TEAL, CYAN, BLUE, INDIGO, PURPLE, PINK, BROWN,
];

use mrly_sequences::counts::diagonal::WIDEST;

struct Solid {
    filled: Vec<bool>,
    ranks: Vec<usize>,
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
        let filled: Vec<bool> = pattern.bytes()?.iter().map(|&byte| byte != 0).collect();
        let mut ranks = vec![0usize; filled.len()];
        let mut count = 0;
        for (flat, &live) in filled.iter().enumerate() {
            if !live {
                continue;
            }
            ranks[flat] = count;
            count += 1;
        }
        Ok(Solid {
            filled,
            ranks,
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

pub fn profile(code: Code, number: usize, level: usize, base: usize) -> Result<Vec<u128>> {
    let tile = factory::create(code, number, 3, base, 1)?;
    profile_of_tile(&tile, level as u32)
}

pub fn support(counts: &[u128]) -> Option<(usize, usize)> {
    let low = counts.iter().position(|&count| count > 0)?;
    let high = counts.iter().rposition(|&count| count > 0)?;
    Some((low, high))
}

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

pub fn project(point: [u32; 3]) -> (f64, f64) {
    let (x, y, z) = (point[0] as f64, point[1] as f64, point[2] as f64);
    ((x - y) / 2f64.sqrt(), (x + y - 2.0 * z) / 6f64.sqrt())
}

pub fn shadow(point: [u32; 3]) -> (i64, i64) {
    let (x, y, z) = (point[0] as i64, point[1] as i64, point[2] as i64);
    (x - y, x + y - 2 * z)
}

pub fn svg(
    code: Code,
    number: usize,
    level: usize,
    base: usize,
    heights: &[usize],
    scale: usize,
) -> Result<String> {
    let solid = Solid::new(code, number, level, base)?;
    let mut circles: Vec<(f64, f64, Color)> = Vec::new();
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
            "<circle cx=\"{cx:.2}\" cy=\"{cy:.2}\" r=\"{radius:.2}\" fill=\"{}\"/>",
            fill.to_hex()
        ));
    }
    out.push("</svg>".to_string());
    Ok(out.join("\n"))
}
