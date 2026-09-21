use crate::core::error::{value_error, Result};
use crate::num::factor::gcd;
use crate::num::lattice::farey;

/// The largest curvature a packing is grown to.
pub const CURVATURE_CAP: i64 = 8192;
/// The most circles one growth makes before it gives up.
pub const CIRCLE_CAP: usize = 200_000;
/// The deepest the Farey stack is read against a packing.
pub const ORDER_CAP: usize = 64;
/// The root quadruples on offer: the strip first, then the bounded packings named by their curvatures.
pub const ROOTS: [&str; 4] = ["strip", "-1,2,2,3", "-2,3,6,7", "-3,4,12,13"];

// THE OBJECTS

/// A circle in the integer coordinates `(k, k x, k y)`: a line is `k = 0` with `(k x, k y)` its outward unit normal, and the curvature is negative on the circle that contains a bounded packing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Circle {
    /// The curvature.
    pub k: i64,
    /// The curvature times the centre's abscissa.
    pub x: i64,
    /// The curvature times the centre's ordinate.
    pub y: i64,
}

impl Circle {
    /// Whether the circle is a line.
    pub fn is_line(self) -> bool {
        self.k == 0
    }

    /// The radius, none on a line.
    pub fn radius(self) -> Option<f64> {
        (self.k != 0).then(|| 1.0 / self.k.abs() as f64)
    }

    /// The centre, none on a line.
    pub fn centre(self) -> Option<(f64, f64)> {
        (self.k != 0).then(|| (self.x as f64 / self.k as f64, self.y as f64 / self.k as f64))
    }
}

/// Four mutually tangent circles.
pub type Quad = [Circle; 4];

fn column(q: &Quad, at: usize) -> [i64; 4] {
    let read = |c: &Circle| match at {
        0 => c.k,
        1 => c.x,
        _ => c.y,
    };
    [read(&q[0]), read(&q[1]), read(&q[2]), read(&q[3])]
}

/// The bilinear form `B(u, v) = (sum u)(sum v) - 2 sum u v` that the reflection preserves.
///
/// ```
/// use mrlyrs::num::apollonian::form;
/// assert_eq!(form([-1, 2, 2, 3], [-1, 2, 2, 3]), 0);
/// assert_eq!(form([0, 1, -1, 0], [0, 1, -1, 0]), -4);
/// ```
pub fn form(u: [i64; 4], v: [i64; 4]) -> i128 {
    let su: i128 = u.iter().map(|&a| i128::from(a)).sum();
    let sv: i128 = v.iter().map(|&a| i128::from(a)).sum();
    let dot: i128 = (0..4).map(|i| i128::from(u[i]) * i128::from(v[i])).sum();
    su * sv - 2 * dot
}

/// Whether the quadruple carries all six exact invariants: Descartes `B(k, k) = 0`, the position half `B(k, kx) = B(k, ky) = B(kx, ky) = 0`, and the frame `B(kx, kx) = B(ky, ky) = -4`.
pub fn sound(q: &Quad) -> bool {
    let (k, x, y) = (column(q, 0), column(q, 1), column(q, 2));
    form(k, k) == 0
        && form(k, x) == 0
        && form(k, y) == 0
        && form(x, y) == 0
        && form(x, x) == -4
        && form(y, y) == -4
}

/// Reflects the circle at the seat through the other three, `v' = 2(v_1 + v_2 + v_3) - v` on all three coordinates at once, which is the second root of the Descartes quadratic and needs no square root.
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

/// The quadruple with the circle at the seat replaced by its reflection.
pub fn swap(q: &Quad, at: usize) -> Quad {
    let mut out = *q;
    out[at] = reflect(q, at);
    out
}

fn circle(k: i64, x: i64, y: i64) -> Circle {
    Circle { k, x, y }
}

/// The named root quadruple: `strip` is the two lines a unit apart holding the circles at `0` and `1`, and the rest are bounded packings named by their four curvatures.
///
/// ```
/// use mrlyrs::num::apollonian::{root, sound};
/// for name in mrlyrs::num::apollonian::ROOTS {
///     assert!(sound(&root(name).unwrap()));
/// }
/// ```
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

// THE GROWTH

/// A packing grown from its root in exact integers.
pub struct Packing {
    /// The root quadruple the growth started from.
    pub root: Quad,
    /// Whether the root carries a line, so the packing is the strip and the growth keeps one period.
    pub strip: bool,
    /// The curvature the growth stopped at.
    pub cap: i64,
    /// The circles the growth made, the root excluded, in curvature order.
    pub circles: Vec<Circle>,
    /// The quadruples the growth made, the root counted.
    pub quads: u64,
    /// The quadruples that failed one of the six invariants.
    pub broken: u64,
    /// The circles centred outside the open period, which the strip must have none of.
    pub strayed: u64,
}

/// Grows the named packing to the curvature cap, one circle per node of the reflection tree and the root quadruple excluded, so `circles.len()` is the census `N(T)`. On the strip only the two root swaps that replace a line are taken, which are exactly the two that stay inside one period.
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

/// The box the packing is drawn in: one period of the strip, or the box of the circle that contains a bounded packing.
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

// THE FORD CIRCLES

/// A tangency point on the line `y = 0`: the reduced fraction the circle rests at and the curvature it carries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Touch {
    /// The numerator of the reduced fraction.
    pub num: i64,
    /// The denominator of the reduced fraction.
    pub den: i64,
    /// The curvature of the circle resting there, which the Ford identification forces to be `2 den^2`.
    pub k: i64,
}

fn reduce(c: Circle) -> (i64, i64) {
    let g = gcd(c.x.unsigned_abs().into(), c.k.unsigned_abs().into()) as i64;
    (c.x / g, c.k / g)
}

/// Whether the circle has positive curvature and is tangent to the line `y = 0`, which in these coordinates reads `k > 0` and `k y = 1`: the curvature guard is what excludes the line `y = 1`, which is `(0, 0, 1)`.
pub fn on_line(c: Circle) -> bool {
    c.k > 0 && c.y == 1
}

/// Whether the circle is the Ford circle over its own tangency point: curvature `2 b^2` and abscissa `2 a b` at the reduced `a/b`.
pub fn is_ford(c: Circle) -> bool {
    if !on_line(c) {
        return false;
    }
    let (a, b) = reduce(c);
    c.k == 2 * b * b && c.x == 2 * a * b
}

/// The tangency points on the line `y = 0`, ascending: one per circle of the packing with `k y = 1`, the root excluded. Empty off the strip.
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

// THE SHADOW

/// The Farey stack read against the packing's tangency points.
pub struct Shadow {
    /// The depth the stack is read at.
    pub order: usize,
    /// The curvature a circle of denominator the order carries, twice the order squared.
    pub reach: i64,
    /// Whether the packing was grown far enough to carry every node of that depth.
    pub covered: bool,
    /// The stack's nodes inside the open period.
    pub nodes: usize,
    /// The packing's tangency points of denominator at most the order.
    pub touched: usize,
    /// The nodes no tangency point rests on, plus the tangency points no node lights.
    pub missed: usize,
    /// The circles below the reach tangent to the line that are not Ford circles.
    pub offford: usize,
    /// The brightness of the period summed node by node, the node `0/1` on the period's edge counted.
    pub bright: u128,
    /// The closed form that brightness lands on, `Q(Q + 1)/2`.
    pub want: u128,
}

/// Reads the Farey stack of the order against the packing: the nodes lit inside the open period against the tangency points of the line-tangent circles of curvature at most `2 Q^2`, and the brightness `floor(Q/b)` summed on the nodes against `Q(Q + 1)/2`. Off the strip there is no line and every count is zero.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_reflection_keeps_the_six_invariants_on_every_quadruple() {
        for name in ROOTS {
            let p = grow(name, 512).unwrap();
            assert_eq!(p.broken, 0);
            assert_eq!(p.strayed, 0);
            assert!(p.quads > p.circles.len() as u64);
        }
        let seed = root("strip").unwrap();
        assert_eq!(reflect(&seed, 0), Circle { k: 8, x: 4, y: 7 });
        assert_eq!(reflect(&seed, 1), Circle { k: 8, x: 4, y: 1 });
        assert_eq!(reflect(&seed, 2), Circle { k: 2, x: 4, y: 1 });
    }

    #[test]
    fn the_census_lands_on_the_counts_the_generator_prints() {
        assert_eq!(grow("strip", 1000).unwrap().circles.len(), 950);
        assert_eq!(grow("-1,2,2,3", 1000).unwrap().circles.len(), 3325);
        assert_eq!(grow("strip", 2048).unwrap().circles.len(), 2448);
    }

    #[test]
    fn every_line_tangent_circle_is_the_ford_circle_over_its_own_fraction() {
        let p = grow("strip", 2048).unwrap();
        let marks = touches(&p);
        assert_eq!(marks.len(), 323);
        assert!(p
            .circles
            .iter()
            .filter(|c| on_line(**c))
            .all(|&c| is_ford(c)));
        for mark in &marks {
            assert_eq!(mark.k, 2 * mark.den * mark.den);
            assert_eq!(gcd(mark.num as u128, mark.den as u128), 1);
        }
        assert!(marks
            .windows(2)
            .all(|pair| { pair[0].num * pair[1].den < pair[1].num * pair[0].den }));
        assert!(touches(&grow("-1,2,2,3", 2048).unwrap()).is_empty());
    }

    #[test]
    fn the_stack_is_the_shadow_of_the_line_tangent_circles() {
        let p = grow("strip", 2048).unwrap();
        let read = shadow(&p, 32).unwrap();
        assert!(read.covered);
        assert_eq!(read.reach, 2048);
        assert_eq!((read.nodes, read.touched, read.missed), (323, 323, 0));
        assert_eq!(read.offford, 0);
        assert_eq!((read.bright, read.want), (528, 528));
        let shallow = shadow(&p, 16).unwrap();
        assert_eq!(
            (
                shallow.nodes,
                shallow.touched,
                shallow.missed,
                shallow.bright
            ),
            (79, 79, 0, 136)
        );
        assert!(!shadow(&grow("strip", 512).unwrap(), 32).unwrap().covered);
        assert!(shadow(&p, ORDER_CAP + 1).is_err());
    }

    #[test]
    fn a_bounded_root_frames_its_own_outer_circle() {
        assert_eq!(frame(&grow("strip", 512).unwrap()), [0.0, 0.0, 1.0, 1.0]);
        assert_eq!(
            frame(&grow("-1,2,2,3", 512).unwrap()),
            [-1.0, -1.0, 1.0, 1.0]
        );
        assert!(grow("gasket", 512).is_err());
        assert!(grow("strip", CURVATURE_CAP + 1).is_err());
    }
}
