use mrlyfig::{ink, plot, save, Board, Frame, Grid};
use mrlyrs::core::error::Result;
use mrlyrs::math::three::{carpet, slice};
use std::f64::consts::PI;

const LEVEL: usize = 4;
const SIDE: usize = 81;
const EPS: f64 = 1.0 / 36.0;
const LEVELS: usize = 200;
const DEPTH: usize = 40;
const DIGITS: usize = 40;
const CELLW: [f64; 3] = [3.0, 2.0, 3.0];
const SUMW: [f64; 3] = [0.0, 3.0, 5.0];
const BANDS: [(f64, f64, f64); 3] = [
    (1.0 / 12.0, 2.122718, 2.122723),
    (1.0 / 8.0, 2.134668, 2.135742),
    (1.0 / 6.0, 2.135019, 2.136794),
];
const TUBES: [(f64, f64, f64); 2] = [
    (1.0 / 12.0, 0.180947086, 0.180947093),
    (1.0 / 8.0, 0.234186414, 0.234701259),
];

// THE HOLE INTEGRALS

fn a0(t: f64, d: f64) -> f64 {
    if t >= d {
        return PI * d * d / 4.0;
    }
    t / 2.0 * (d * d - t * t).sqrt() + d * d / 2.0 * (t / d).asin()
}

fn a1(t: f64, d: f64) -> f64 {
    if t >= d {
        return d * d * d / 3.0;
    }
    let y = d * d - t * t;
    t * t * (3.0 * d.powi(4) - 3.0 * d * d * t * t + t.powi(4)) / (3.0 * (d.powi(3) + y * y.sqrt()))
}

fn seg(t0: f64, t1: f64, alpha: f64, beta: f64, d: f64) -> f64 {
    if t1 <= t0 {
        return 0.0;
    }
    alpha * (a0(t1, d) - a0(t0, d)) - beta * (a1(t1, d) - a1(t0, d))
}

fn hole(s: f64, d: f64) -> f64 {
    4.0 * seg(0.0, s / 2.0, s, 2.0, d)
}

fn partial(s: f64, c: f64, d: f64) -> f64 {
    let left = seg(0.0, (s / 2.0).min(c), s, 2.0, d);
    let right = seg((s - c).max(0.0), s / 2.0, s, 2.0, d);
    let bottom = if c <= s / 2.0 {
        seg(0.0, c, c, 1.0, d)
    } else {
        seg(0.0, s - c, c, 1.0, d) + seg(s - c, s / 2.0, s, 2.0, d)
    };
    left + right + 2.0 * bottom
}

fn digit(q: f64) -> usize {
    ((q - 3.0 * (q / 3.0).floor()) as usize).min(2)
}

fn weight(mut i: f64, digits: usize) -> f64 {
    let mut w = 1.0;
    for _ in 0..digits {
        w *= CELLW[digit(i)];
        i = (i / 3.0).floor();
    }
    w
}

fn below(count: f64, digits: usize) -> f64 {
    if count >= 3f64.powi(digits as i32) {
        return 8f64.powi(digits as i32);
    }
    let (mut total, mut prefix) = (0.0, 1.0);
    for d in (0..digits).rev() {
        let k = digit((count / 3f64.powi(d as i32)).floor());
        total += prefix * SUMW[k] * 8f64.powi(d as i32);
        prefix *= CELLW[k];
    }
    total
}

fn wall_half(delta: f64) -> f64 {
    let mut total = 0.0;
    for m in 1..=LEVELS {
        total += 8f64.powi(m as i32 - 1) * hole(3f64.powi(-(m as i32) - 1), delta);
    }
    total / 2.0
}

fn strip(delta: f64) -> f64 {
    let mut total = 0.0;
    for m in 1..=LEVELS {
        let s = 3f64.powi(-(m as i32) - 1);
        let ratio = delta / s;
        let columns = 3f64.powi(m as i32 - 1);
        let full = ((ratio - 2.0) / 3.0).floor() + 1.0;
        if full > 0.0 {
            total += below(full.min(columns), m - 1) * hole(s, delta);
        }
        let cut = ((ratio - 1.0) / 3.0).floor();
        if cut >= 0.0
            && cut < columns
            && (3.0 * cut + 1.0) * s < delta
            && delta < (3.0 * cut + 2.0) * s
        {
            total += weight(cut, m - 1) * partial(s, delta - (3.0 * cut + 1.0) * s, delta);
        }
    }
    total
}

fn tube(delta: f64) -> f64 {
    (PI + 8.0) * delta * delta - 8.0 * 2f64.sqrt() * delta.powi(3)
        + 48.0 * (wall_half(delta) - strip(delta))
}

fn periodic(eps: f64) -> f64 {
    let dim = 20f64.ln() / 3f64.ln();
    let mut total = 20.0 / 27.0;
    for l in 0..=DEPTH {
        total += (27f64 / 20.0).powi(l as i32) * tube(eps / 3f64.powi(l as i32));
    }
    eps.powf(dim - 3.0) * total
}

// THE DISTANCE IN THE MIDPLANE

fn split(x: f64) -> (usize, f64) {
    let d = (3.0 * x).floor().min(2.0);
    (d as usize, 3.0 * x - d)
}

fn carpet_dist(mut a: f64, mut b: f64) -> f64 {
    let mut s = 1.0 / 3.0;
    for _ in 0..DIGITS {
        let ((da, fa), (db, fb)) = (split(a), split(b));
        if da == 1 && db == 1 {
            return s * fa.min(1.0 - fa).min(fb).min(1.0 - fb);
        }
        a = fa;
        b = fb;
        s /= 3.0;
    }
    0.0
}

fn wall(u: f64, along: f64, across: f64) -> f64 {
    let d = carpet_dist(3.0 * along, 3.0 * across) / 3.0;
    (u * u + d * d).sqrt()
}

fn arm_dist(along: f64, u: f64) -> f64 {
    let side = wall(u, along, 1.0 / 6.0).min(wall(1.0 / 3.0 - u, along, 1.0 / 6.0));
    let far = wall(1.0 / 6.0, along, u);
    side.min(far)
}

fn plus_dist(x: f64, y: f64) -> f64 {
    let (third, two) = (1.0 / 3.0, 2.0 / 3.0);
    let mid = |v: f64| (third..=two).contains(&v);
    if mid(x) && mid(y) {
        let mut best = f64::MAX;
        for cx in [third, two] {
            for cy in [third, two] {
                best = best.min(((x - cx).powi(2) + (y - cy).powi(2)).sqrt());
            }
        }
        let edge = (x - third)
            .abs()
            .min((x - two).abs())
            .min((y - third).abs())
            .min((y - two).abs());
        return best.min((1.0 / 36.0 + edge * edge).sqrt());
    }
    if mid(y) {
        let along = if x < third { x } else { 1.0 - x };
        return arm_dist(along, y - third);
    }
    let along = if y < third { y } else { 1.0 - y };
    arm_dist(along, x - third)
}

fn dist(mut x: f64, mut y: f64) -> f64 {
    let mut scale = 1.0;
    for _ in 0..DIGITS {
        let ((dx, fx), (dy, fy)) = (split(x), split(y));
        if dx == 1 || dy == 1 {
            return scale * plus_dist(x, y);
        }
        x = fx;
        y = fy;
        scale /= 3.0;
    }
    0.0
}

// THE MARKS

fn bands(board: &mut Board, frame: Frame) {
    plot::axis(board, frame, ink::line());
    let area = frame.inset(24.0);
    let (lo, hi) = (2.120, 2.140);
    let y_of = |v: f64| area.y + area.h * (hi - v) / (hi - lo);
    let width = area.w * 0.3;
    let (a, b) = (BANDS[0], BANDS[2]);
    for (k, band) in [a, b].iter().enumerate() {
        let x = area.x + area.w * (0.25 + 0.5 * k as f64) - width / 2.0;
        let (top, foot) = (y_of(band.2), y_of(band.1));
        let h = (foot - top).max(4.0);
        board.rect(x, top, width, h, ink::orange());
    }
    let (cx, gap_lo, gap_hi) = (area.x + area.w / 2.0, y_of(a.2) - 2.0, y_of(b.1) + 2.0);
    board.segment((cx, gap_lo), (cx, gap_hi), 2.0, ink::dim());
    board.segment((cx - 10.0, gap_lo), (cx + 10.0, gap_lo), 2.0, ink::dim());
    board.segment((cx - 10.0, gap_hi), (cx + 10.0, gap_hi), 2.0, ink::dim());
}

fn main() -> Result<()> {
    let sponge = carpet(3, LEVEL)?;
    assert_eq!(sponge.types().sum(), 160000);
    let dust = slice(&sponge, 2, (SIDE - 1) / 2)?;
    let cells = dust.types().bytes().to_vec();
    assert_eq!(cells.len(), SIDE * SIDE);
    assert_eq!(cells.iter().filter(|&&b| b != 0).count(), 256);

    let ident = (PI + 8.0) / 36.0 - 2f64.sqrt() / 27.0;
    assert!((tube(1.0 / 6.0) - ident).abs() < 1e-9);
    for (delta, lo, hi) in TUBES {
        let t = tube(delta);
        assert!(lo <= t && t <= hi);
    }
    let mut values = [0.0; 3];
    for (k, (eps, lo, hi)) in BANDS.iter().enumerate() {
        values[k] = periodic(*eps);
        assert!(*lo <= values[k] && values[k] <= *hi);
    }
    assert!(values[2] - values[0] >= 0.012296);
    assert!((dist(0.5, 0.5) - 2f64.sqrt() / 6.0).abs() < 1e-15);
    assert!((dist(1.0 / 6.0, 1.0 / 6.0) - 2f64.sqrt() / 18.0).abs() < 1e-15);

    let mut board = Board::square();
    let margin = (board.width as f64 * 0.08).round();
    let plate = 600.0;
    let sheet = Frame::new(margin, margin, plate, plate);
    board.rect(sheet.x, sheet.y, sheet.w, sheet.h, ink::panel());
    let n = plate as usize;
    let blue = ink::blue();
    for j in 0..n {
        for i in 0..n {
            let (x, y) = ((i as f64 + 0.5) / plate, (j as f64 + 0.5) / plate);
            let cover = (0.5 + (EPS - dist(x, y)) * plate).clamp(0.0, 1.0);
            if cover > 0.0 {
                board.blend(sheet.x as usize + i, sheet.y as usize + j, blue, cover);
            }
        }
    }
    let lattice = Grid::new(sheet, SIDE, SIDE, 0.0);
    for row in 0..SIDE {
        for col in 0..SIDE {
            if cells[row * SIDE + col] != 0 {
                lattice.fill(&mut board, col, row, ink::fg());
            }
        }
    }
    let panel = Frame::new(
        board.width as f64 - margin - 240.0,
        board.height as f64 - margin - 240.0,
        240.0,
        240.0,
    );
    bands(&mut board, panel);
    save("paper-sponge-measurability", &board)?;
    Ok(())
}
