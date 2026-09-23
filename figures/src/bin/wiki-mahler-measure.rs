use figures::{ink, save, Board};
use mrlyrs::core::error::Result;
use std::f64::consts::TAU;

const LEHMER: [f64; 11] = [1.0, 1.0, 0.0, -1.0, -1.0, -1.0, -1.0, -1.0, 0.0, 1.0, 1.0];
const SALEM: f64 = 1.176_280_818_259_917_5;
const SAMPLES: usize = 4096;
const FINE: usize = 1 << 16;
const STRETCH: f64 = 0.34;
const FLOOR: f64 = 2.0;

type Z = (f64, f64);

fn mul(a: Z, b: Z) -> Z {
    (a.0 * b.0 - a.1 * b.1, a.0 * b.1 + a.1 * b.0)
}

fn div(a: Z, b: Z) -> Z {
    let d = b.0 * b.0 + b.1 * b.1;
    ((a.0 * b.0 + a.1 * b.1) / d, (a.1 * b.0 - a.0 * b.1) / d)
}

fn norm(z: Z) -> f64 {
    z.0.hypot(z.1)
}

fn eval(z: Z) -> Z {
    LEHMER.iter().rev().fold((0.0, 0.0), |acc, c| {
        let m = mul(acc, z);
        (m.0 + c, m.1)
    })
}

fn height(t: f64) -> f64 {
    norm(eval((t.cos(), t.sin()))).ln()
}

fn roots() -> Vec<Z> {
    let degree = LEHMER.len() - 1;
    let seed = (0.4, 0.9);
    let mut zs: Vec<Z> = Vec::with_capacity(degree);
    let mut z = (1.0, 0.0);
    for _ in 0..degree {
        zs.push(z);
        z = mul(z, seed);
    }
    for _ in 0..500 {
        for k in 0..degree {
            let mut den = (1.0, 0.0);
            for j in 0..degree {
                if j != k {
                    den = mul(den, (zs[k].0 - zs[j].0, zs[k].1 - zs[j].1));
                }
            }
            let step = div(eval(zs[k]), den);
            zs[k] = (zs[k].0 - step.0, zs[k].1 - step.1);
        }
    }
    zs
}

fn squeeze(v: f64) -> f64 {
    if v >= 0.0 {
        v
    } else {
        -FLOOR * (1.0 - (v / FLOOR).exp())
    }
}

fn main() -> Result<()> {
    let zs = roots();
    let on: Vec<Z> = zs
        .iter()
        .copied()
        .filter(|z| (norm(*z) - 1.0).abs() < 1e-9)
        .collect();
    let off: Vec<Z> = zs
        .iter()
        .copied()
        .filter(|z| (norm(*z) - 1.0).abs() >= 1e-9)
        .collect();
    assert_eq!(on.len(), 8);
    assert_eq!(off.len(), 2);
    let outside: f64 = off.iter().map(|z| norm(*z).max(1.0)).product();
    assert!((outside - SALEM).abs() < 1e-12);
    assert!(off.iter().all(|z| z.1.abs() < 1e-12 && z.0 > 0.0));
    let mean = (0..FINE)
        .map(|i| height((i as f64 + 0.5) / FINE as f64 * TAU))
        .sum::<f64>()
        / FINE as f64;
    assert!((mean - SALEM.ln()).abs() < 1e-4);

    let reach: Vec<f64> = (0..SAMPLES)
        .map(|i| 1.0 + STRETCH * squeeze(height(i as f64 / SAMPLES as f64 * TAU)))
        .collect();
    let mut angles: Vec<f64> = (0..SAMPLES)
        .map(|i| i as f64 / SAMPLES as f64 * TAU)
        .collect();
    angles.extend(on.iter().map(|z| z.1.atan2(z.0).rem_euclid(TAU)));
    angles.sort_by(|a, b| a.total_cmp(b));
    assert_eq!(angles.len(), SAMPLES + 8);
    let levels: Vec<f64> = angles.iter().map(|t| height(*t)).collect();
    let plane: Vec<(f64, f64)> = angles
        .iter()
        .zip(&levels)
        .map(|(t, v)| {
            let r = 1.0 + STRETCH * squeeze(*v);
            (r * t.cos(), r * t.sin())
        })
        .collect();
    let (mut x0, mut x1, mut y0, mut y1) = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
    for p in &plane {
        x0 = x0.min(p.0);
        x1 = x1.max(p.0);
        y0 = y0.min(p.1);
        y1 = y1.max(p.1);
    }

    let mut board = Board::square();
    let frame = board.frame(0.08);
    let scale = frame.w / (x1 - x0).max(y1 - y0);
    let (fx, fy) = frame.center();
    let cx = fx - scale * (x0 + x1) / 2.0;
    let cy = fy + scale * (y0 + y1) / 2.0;
    let at = |x: f64, y: f64| (cx + scale * x, cy - scale * y);

    let out = ink::fade(ink::blue(), 0.22);
    let inn = ink::fade(ink::orange(), 0.22);
    let (bx0, by0) = at(x0, y1);
    let (bx1, by1) = at(x1, y0);
    for py in (by0.floor() as usize)..(by1.ceil() as usize).min(board.height) {
        for px in (bx0.floor() as usize)..(bx1.ceil() as usize).min(board.width) {
            let x = (px as f64 + 0.5 - cx) / scale;
            let y = (cy - py as f64 - 0.5) / scale;
            let rho = x.hypot(y);
            let u = y.atan2(x).rem_euclid(TAU) / TAU * SAMPLES as f64;
            let i = (u.floor() as usize) % SAMPLES;
            let w = u - u.floor();
            let r = reach[i] * (1.0 - w) + reach[(i + 1) % SAMPLES] * w;
            if rho > 1.0 && rho < r {
                board.blend(px, py, out, 1.0);
            } else if rho < 1.0 && rho > r {
                board.blend(px, py, inn, 1.0);
            }
        }
    }

    board.ring(cx, cy, scale, 3.0, ink::dim());
    let mut run: Vec<(f64, f64)> = Vec::new();
    let mut up = levels[0] >= 0.0;
    for i in 0..=angles.len() {
        let k = i % angles.len();
        let p = at(plane[k].0, plane[k].1);
        let side = levels[k] >= 0.0;
        if side != up {
            run.push(p);
            board.polyline(&run, 5.0, if up { ink::blue() } else { ink::orange() });
            run.clear();
            up = side;
        }
        run.push(p);
    }
    board.polyline(&run, 5.0, if up { ink::blue() } else { ink::orange() });

    for z in &on {
        let (px, py) = at(z.0, z.1);
        board.disc(px, py, 8.0, ink::fg());
    }
    for z in &off {
        let (px, py) = at(z.0, z.1);
        board.disc(px, py, 13.0, ink::yellow());
    }
    save("wiki-mahler-measure", &board)?;
    Ok(())
}
