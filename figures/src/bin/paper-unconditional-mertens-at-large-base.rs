use figures::{ink, save, Board, Color};
use mrlyrs::core::error::Result;
use std::f64::consts::TAU;

const NAME: &str = "paper-unconditional-mertens-at-large-base";
const BASE: i64 = 10;
const LEVEL: u32 = 3;
const Z: i64 = 8;
const COUNTS: [usize; 4] = [732, 202, 26, 40];

// DISSECTION

fn fraction(a: i64, y: i64, cap: i64) -> (i64, i64) {
    let (mut p0, mut q0, mut p1, mut q1) = (0i64, 1i64, 1i64, 0i64);
    let (mut n, mut d) = (a, y);
    let mut best = (0, 1);
    while d > 0 {
        let c = n / d;
        let (p2, q2) = (c * p1 + p0, c * q1 + q0);
        if q2 > cap {
            break;
        }
        (p0, q0, p1, q1) = (p1, q1, p2, q2);
        best = (p2, q2);
        (n, d) = (d, n - c * d);
    }
    best
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

fn smooth(mut d: i64) -> bool {
    loop {
        let g = gcd(d, BASE);
        if g == 1 {
            return d == 1;
        }
        while d % g == 0 {
            d /= g;
        }
    }
}

fn region(a: i64, y: i64) -> usize {
    let cap = (y as f64).powf(0.6).floor() as i64;
    let low = (y as f64).powf(0.4);
    let (l, d) = fraction(a, y, cap);
    let h = (a * d - l * y).abs();
    assert!(d <= cap && h * cap <= y && gcd(l, d) == 1);
    if d as f64 >= low {
        0
    } else if d < Z && h < Z {
        if smooth(d) {
            3
        } else {
            2
        }
    } else {
        1
    }
}

// DRAWING

fn band(
    board: &mut Board,
    center: (f64, f64),
    radii: (f64, f64),
    angles: (f64, f64),
    color: Color,
) {
    let at = |r: f64, t: f64| (center.0 + r * t.sin(), center.1 - r * t.cos());
    let steps = (((angles.1 - angles.0) / 0.004).ceil() as usize).max(1);
    let mut pts = Vec::with_capacity(2 * steps + 2);
    for i in 0..=steps {
        pts.push(at(
            radii.1,
            angles.0 + (angles.1 - angles.0) * i as f64 / steps as f64,
        ));
    }
    for i in (0..=steps).rev() {
        pts.push(at(
            radii.0,
            angles.0 + (angles.1 - angles.0) * i as f64 / steps as f64,
        ));
    }
    board.polygon(&pts, color);
}

fn runs(regions: &[usize], pass: usize) -> Vec<(usize, usize)> {
    let n = regions.len();
    let start = (0..n).find(|a| regions[*a] != pass).unwrap_or(0);
    let mut out = Vec::new();
    let mut open: Option<usize> = None;
    for step in 1..=n {
        let a = start + step;
        let inside = regions[a % n] == pass;
        match (open, inside) {
            (None, true) => open = Some(a),
            (Some(first), false) => {
                out.push((first, a));
                open = None;
            }
            _ => {}
        }
    }
    if let Some(first) = open {
        out.push((first, start + n + 1));
    }
    out
}

fn main() -> Result<()> {
    let y = BASE.pow(LEVEL);
    let regions: Vec<usize> = (0..y).map(|a| region(a, y)).collect();
    let mut counts = [0usize; 4];
    for r in &regions {
        counts[*r] += 1;
    }
    assert_eq!(counts, COUNTS);
    assert_eq!(counts.iter().sum::<usize>(), y as usize);

    let mut board = Board::square();
    let frame = board.frame(0.08);
    let center = frame.center();
    let outer = frame.radius();
    let slot = TAU / y as f64;
    let width = 0.12 * outer;
    let tracks = [0.28, 0.48, 0.68, 0.88];
    let inks = [ink::blue(), ink::dim(), ink::orange(), ink::yellow()];
    for pass in 0..4 {
        let mid = tracks[pass] * outer + width / 2.0;
        board.ring(center.0, center.1, mid, 1.0, ink::line());
        let mut held = 0;
        for (first, end) in runs(&regions, pass) {
            held += end - first;
            let angles = ((first as f64 - 0.5) * slot, (end as f64 - 0.5) * slot);
            band(
                &mut board,
                center,
                (mid - width / 2.0, mid + width / 2.0),
                angles,
                inks[pass],
            );
        }
        assert_eq!(held, counts[pass]);
    }
    save(NAME, &board)?;
    Ok(())
}
