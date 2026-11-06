use mrly_error::{Result, value_error};
use mrly_num::factor::gcd;
use mrly_num::lattice::farey;
use serde::{Deserialize, Serialize};

pub const CURVATURE_CAP: i64 = 8192;
pub const CIRCLE_CAP: usize = 200_000;
pub const ORDER_CAP: usize = 64;
pub const ROOTS: [&str; 4] = ["strip", "-1,2,2,3", "-2,3,6,7", "-3,4,12,13"];
pub const STRIDE: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Circle {
    pub k: i64,
    pub x: i64,
    pub y: i64,
}

impl Circle {
    pub fn is_line(self) -> bool {
        self.k == 0
    }

    pub fn radius(self) -> Option<f64> {
        (self.k != 0).then(|| 1.0 / self.k.abs() as f64)
    }

    pub fn centre(self) -> Option<(f64, f64)> {
        (self.k != 0).then(|| (self.x as f64 / self.k as f64, self.y as f64 / self.k as f64))
    }
}

pub type Quad = [Circle; 4];

fn column(q: &Quad, at: usize) -> [i64; 4] {
    let read = |c: &Circle| match at {
        0 => c.k,
        1 => c.x,
        _ => c.y,
    };
    [read(&q[0]), read(&q[1]), read(&q[2]), read(&q[3])]
}

pub fn form(u: [i64; 4], v: [i64; 4]) -> i128 {
    let su: i128 = u.iter().map(|&a| i128::from(a)).sum();
    let sv: i128 = v.iter().map(|&a| i128::from(a)).sum();
    let dot: i128 = (0..4).map(|i| i128::from(u[i]) * i128::from(v[i])).sum();
    su * sv - 2 * dot
}

pub fn sound(q: &Quad) -> bool {
    let (k, x, y) = (column(q, 0), column(q, 1), column(q, 2));
    form(k, k) == 0
        && form(k, x) == 0
        && form(k, y) == 0
        && form(x, y) == 0
        && form(x, x) == -4
        && form(y, y) == -4
}

pub fn reflect(q: &Quad, at: usize) -> Circle {
    let (mut k, mut x, mut y) = (0i64, 0i64, 0i64);
    for (i, c) in q.iter().enumerate() {
        if i != at {
            k += c.k;
            x += c.x;
            y += c.y;
        }
    }
    Circle {
        k: 2 * k - q[at].k,
        x: 2 * x - q[at].x,
        y: 2 * y - q[at].y,
    }
}

pub fn swap(q: &Quad, at: usize) -> Quad {
    let mut out = *q;
    out[at] = reflect(q, at);
    out
}

fn circle(k: i64, x: i64, y: i64) -> Circle {
    Circle { k, x, y }
}

pub fn root(name: &str) -> Result<Quad> {
    Ok(match name {
        "strip" => [
            circle(0, 0, -1),
            circle(0, 0, 1),
            circle(2, 0, 1),
            circle(2, 2, 1),
        ],
        "-1,2,2,3" => [
            circle(-1, 0, 0),
            circle(2, 1, 0),
            circle(2, -1, 0),
            circle(3, 0, 2),
        ],
        "-2,3,6,7" => [
            circle(-2, -1, 0),
            circle(3, 1, 0),
            circle(6, 5, 0),
            circle(7, 5, 2),
        ],
        "-3,4,12,13" => [
            circle(-3, -1, 0),
            circle(4, 1, 0),
            circle(12, 7, 0),
            circle(13, 7, 2),
        ],
        _ => return value_error(format!("the root must be one of {}.", ROOTS.join(", "))),
    })
}

#[derive(Serialize, Deserialize)]
pub struct Packing {
    pub root: Quad,
    pub strip: bool,
    pub cap: i64,
    pub circles: Vec<Circle>,
    pub quads: u64,
    pub broken: u64,
    pub strayed: u64,
}

pub fn grow(name: &str, cap: i64) -> Result<Packing> {
    let seed = root(name)?;
    if !(2..=CURVATURE_CAP).contains(&cap) {
        return value_error(format!(
            "the curvature cap must be between 2 and {CURVATURE_CAP}."
        ));
    }
    let strip = seed.iter().any(|c| c.is_line());
    let mut out = Packing {
        root: seed,
        strip,
        cap,
        circles: Vec::new(),
        quads: 1,
        broken: u64::from(!sound(&seed)),
        strayed: 0,
    };
    let mut stack: Vec<(Quad, usize)> = Vec::new();
    for i in 0..4 {
        if strip && !seed[i].is_line() {
            continue;
        }
        if reflect(&seed, i).k <= cap {
            stack.push((swap(&seed, i), i));
        }
    }
    while let Some((q, last)) = stack.pop() {
        out.quads += 1;
        if !sound(&q) {
            out.broken += 1;
        }
        let made = q[last];
        if out.circles.len() >= CIRCLE_CAP {
            return value_error(format!(
                "the packing passes {CIRCLE_CAP} circles below that curvature; lower the cap."
            ));
        }
        out.circles.push(made);
        if strip && (made.x <= 0 || made.x >= made.k) {
            out.strayed += 1;
        }
        for j in 0..4 {
            if j == last {
                continue;
            }
            let next = reflect(&q, j);
            if next.k > cap || next.k <= q[j].k {
                continue;
            }
            stack.push((swap(&q, j), j));
        }
    }
    out.circles.sort_unstable();
    Ok(out)
}

pub fn frame(p: &Packing) -> [f64; 4] {
    if p.strip {
        return [0.0, 0.0, 1.0, 1.0];
    }
    let outer = p
        .root
        .iter()
        .find(|c| c.k < 0)
        .copied()
        .unwrap_or(p.root[0]);
    let (x, y) = outer.centre().unwrap_or((0.0, 0.0));
    let r = outer.radius().unwrap_or(1.0);
    [x - r, y - r, x + r, y + r]
}

pub fn flat(name: &str, cap: i64) -> Result<Vec<f64>> {
    let p = grow(name, cap)?;
    Ok(p.root
        .iter()
        .filter(|c| c.k > 0)
        .chain(&p.circles)
        .flat_map(|c| {
            let k = c.k as f64;
            [c.x as f64 / k, c.y as f64 / k, 1.0 / k, k]
        })
        .collect())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Touch {
    pub num: i64,
    pub den: i64,
    pub k: i64,
}

fn reduce(c: Circle) -> (i64, i64) {
    let g = gcd(c.x.unsigned_abs().into(), c.k.unsigned_abs().into()) as i64;
    (c.x / g, c.k / g)
}

pub fn on_line(c: Circle) -> bool {
    c.k > 0 && c.y == 1
}

pub fn is_ford(c: Circle) -> bool {
    if !on_line(c) {
        return false;
    }
    let (a, b) = reduce(c);
    c.k == 2 * b * b && c.x == 2 * a * b
}

pub fn touches(p: &Packing) -> Vec<Touch> {
    let mut out: Vec<Touch> = p
        .circles
        .iter()
        .filter(|c| on_line(**c))
        .map(|&c| {
            let (num, den) = reduce(c);
            Touch { num, den, k: c.k }
        })
        .collect();
    out.sort_unstable_by(|a, b| {
        (a.num as i128 * b.den as i128).cmp(&(b.num as i128 * a.den as i128))
    });
    out
}

#[derive(Serialize, Deserialize)]
pub struct Shadow {
    pub order: usize,
    pub reach: i64,
    pub covered: bool,
    pub nodes: usize,
    pub touched: usize,
    pub missed: usize,
    pub offford: usize,
    pub bright: u128,
    pub want: u128,
}

pub fn shadow(p: &Packing, order: usize) -> Result<Shadow> {
    if order == 0 || order > ORDER_CAP {
        return value_error(format!("the depth must be between 1 and {ORDER_CAP}."));
    }
    let reach = 2 * (order as i64) * (order as i64);
    let mut out = Shadow {
        order,
        reach,
        covered: p.strip && p.cap >= reach,
        nodes: 0,
        touched: 0,
        missed: 0,
        offford: 0,
        bright: 0,
        want: (order as u128) * (order as u128 + 1) / 2,
    };
    if !p.strip {
        return Ok(out);
    }
    out.offford = p
        .circles
        .iter()
        .filter(|c| on_line(**c) && c.k <= reach && !is_ford(**c))
        .count();
    let mut held: Vec<(i64, i64)> = touches(p)
        .into_iter()
        .filter(|t| t.den <= order as i64)
        .map(|t| (t.num, t.den))
        .collect();
    let mut lit: Vec<(i64, i64)> = farey(order)
        .iter()
        .filter(|n| n.num > 0 && n.num < n.den)
        .map(|n| (n.num as i64, n.den as i64))
        .collect();
    out.touched = held.len();
    out.nodes = lit.len();
    held.sort_unstable();
    lit.sort_unstable();
    out.missed = held
        .iter()
        .filter(|pair| lit.binary_search(pair).is_err())
        .count()
        + lit
            .iter()
            .filter(|pair| held.binary_search(pair).is_err())
            .count();
    out.bright = order as u128
        + lit
            .iter()
            .map(|&(_, den)| (order as u128) / den as u128)
            .sum::<u128>();
    Ok(out)
}
