use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board, Color, Ramp};
use mrlynum::factor::gcd;
use mrlynum::lattice::totients;

const TOP: i64 = 2048;
const DEEP: usize = 32;
const CIRCLES: usize = 2448;
const RESTING: usize = 323;
const THICK: f64 = 1.7;
const HAIR: f64 = 1.2;
const RULE: f64 = 2.0;
const OVER: f64 = 0.035;

#[derive(Clone, Copy)]
struct Circle {
    k: i64,
    x: i64,
    y: i64,
}

type Quad = [Circle; 4];

fn root() -> Quad {
    [
        Circle { k: 0, x: 0, y: -1 },
        Circle { k: 0, x: 0, y: 1 },
        Circle { k: 2, x: 0, y: 1 },
        Circle { k: 2, x: 2, y: 1 },
    ]
}

fn descartes(q: &Quad) -> bool {
    let sum: i64 = q.iter().map(|c| c.k).sum();
    let squares: i64 = q.iter().map(|c| c.k * c.k).sum();
    sum * sum == 2 * squares
}

fn reflect(q: &Quad, i: usize) -> Circle {
    let (mut k, mut x, mut y) = (0i64, 0i64, 0i64);
    for (j, c) in q.iter().enumerate() {
        if j != i {
            k += c.k;
            x += c.x;
            y += c.y;
        }
    }
    Circle {
        k: 2 * k - q[i].k,
        x: 2 * x - q[i].x,
        y: 2 * y - q[i].y,
    }
}

fn swap(q: &Quad, i: usize) -> Quad {
    let mut out = *q;
    out[i] = reflect(q, i);
    out
}

fn grow(cap: i64) -> Vec<Circle> {
    let seed = root();
    let mut out = Vec::new();
    let mut stack: Vec<(Quad, usize)> = Vec::new();
    assert!(descartes(&seed));
    for i in 0..2 {
        if reflect(&seed, i).k <= cap {
            stack.push((swap(&seed, i), i));
        }
    }
    while let Some((q, last)) = stack.pop() {
        assert!(descartes(&q));
        out.push(q[last]);
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
    out
}

fn fraction(c: Circle) -> (i64, i64) {
    let b = ((c.k / 2) as f64).sqrt().round() as i64;
    if b <= 0 {
        return (0, 0);
    }
    (c.x / (2 * b), b)
}

fn mark(board: &mut Board, at: (f64, f64), r: f64, thick: f64, span: (f64, f64), color: Color) {
    let half = thick / 2.0;
    let reach = r + half + 1.0;
    let x0 = (at.0 - reach).max(span.0 - 1.0).max(0.0).floor() as usize;
    let x1 = (at.0 + reach).min(span.1 + 1.0).max(0.0).ceil() as usize;
    let y0 = (at.1 - reach).max(0.0).floor() as usize;
    let y1 = (at.1 + reach).max(0.0).ceil() as usize;
    for py in y0..y1.min(board.height) {
        for px in x0..x1.min(board.width) {
            let (fx, fy) = (px as f64 + 0.5, py as f64 + 0.5);
            let ring = (((fx - at.0).powi(2) + (fy - at.1).powi(2)).sqrt() - r).abs() - half;
            let cut = (span.0 - fx).max(fx - span.1);
            board.blend(px, py, color, 0.5 - ring.max(cut));
        }
    }
}

fn main() -> Result<()> {
    let seed = root();
    let grown = grow(TOP);
    let phi = totients(DEEP);
    let nodes = phi[1..=DEEP].iter().sum::<u64>() as usize - 1;
    let resting: Vec<Circle> = grown.iter().copied().filter(|c| c.y == 1).collect();

    assert_eq!(grown.len(), CIRCLES);
    assert_eq!(resting.len(), RESTING);
    assert_eq!(resting.len(), nodes);
    assert!(grown.iter().all(|c| c.k > 0 && c.x > 0 && c.x < c.k));
    let mut deepest = 1;
    for c in &resting {
        let (a, b) = fraction(*c);
        assert_eq!((c.k, c.x, c.y), (2 * b * b, 2 * a * b, 1));
        assert_eq!(gcd(a as usize, b as usize), 1);
        assert!(0 < a && a < b && b <= DEEP as i64);
        deepest = deepest.max(b);
    }
    assert_eq!(deepest, DEEP as i64);
    assert_eq!(fraction(seed[2]), (0, 1));
    assert_eq!(fraction(seed[3]), (1, 1));

    let mut board = Board::square();
    let frame = board.frame(0.08);
    let span = (frame.x, frame.x + frame.w);
    let over = frame.w * OVER;
    let place = |c: &Circle| {
        (
            (
                frame.x + frame.w * c.x as f64 / c.k as f64,
                frame.y + frame.h * (1.0 - c.y as f64 / c.k as f64),
            ),
            frame.w / c.k as f64,
        )
    };
    for foot in [frame.y, frame.y + frame.h] {
        board.segment(
            (span.0 - over, foot),
            (span.1 + over, foot),
            RULE,
            ink::dim(),
        );
    }
    let rest = ink::fade(ink::dim(), 0.75);
    for c in grown.iter().filter(|c| c.y != 1) {
        let (at, r) = place(c);
        mark(&mut board, at, r, HAIR, span, rest);
    }
    let ramp = Ramp::tone(ink::blue(), ink::fg());
    let reach = (DEEP as f64).ln();
    for c in resting.iter().chain([seed[2], seed[3]].iter()) {
        let (at, r) = place(c);
        let (_, b) = fraction(*c);
        mark(
            &mut board,
            at,
            r,
            THICK,
            span,
            ramp.at((b as f64).ln() / reach),
        );
    }
    save("research-apollonian", &board)?;
    Ok(())
}
