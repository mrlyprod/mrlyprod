use std::collections::BTreeSet;

use figures::{ink, save, Board, Color, Ramp};
use mrlyrs::core::error::Result;
use mrlyrs::num::factor::gcd;

const HEIGHT: f64 = 0.01;
const ROOF: f64 = 1.6;
const SAMPLES: usize = 20000;
const THICK: f64 = 1.5;
const EDGE: f64 = 2.6;
const STEP: f64 = 1.5;

struct Canvas {
    left: f64,
    bottom: f64,
    scale: f64,
    floor: f64,
    top: f64,
}

impl Canvas {
    fn at(&self, x: f64, y: f64) -> (f64, f64) {
        (
            self.left + x * self.scale,
            self.bottom - (y - self.floor) * self.scale,
        )
    }

    fn inside(&self, x: f64, y: f64) -> bool {
        x.abs() <= 0.5 && x * x + y * y >= 1.0 && y <= self.top
    }
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let floor = 3f64.sqrt() / 2.0;
    let scale = frame.h / ROOF;
    let canvas = Canvas {
        left: frame.x + frame.w / 2.0,
        bottom: frame.y + frame.h,
        scale,
        floor,
        top: floor + ROOF,
    };
    let cusps = cusps();
    let deepest = cusps.iter().map(|&(_, c)| c).max().unwrap_or(1);
    let ramp = Ramp::tone(ink::blue(), ink::pink());
    let mut drawn = 0usize;
    for &(a, c) in &cusps {
        let tone = ramp.at((c - 1) as f64 / (deepest - 1).max(1) as f64);
        drawn += trace(&mut board, &canvas, a, c, tone);
    }
    outline(&mut board, &canvas);
    let hits = fold(&cusps);
    assert_eq!((cusps.len(), drawn, hits), (243, 110, 239));
    save("wiki-horocycle-flow", &board)?;
    Ok(())
}

fn cusps() -> Vec<(i64, i64)> {
    let floor = 3f64.sqrt() / 2.0;
    let mut out = Vec::new();
    let mut c = 1i64;
    while 1.0 / ((c * c) as f64 * HEIGHT) >= floor {
        let radius = 1.0 / (2.0 * (c * c) as f64 * HEIGHT);
        let reach = ((radius + 0.5) * c as f64).floor() as i64;
        for a in -reach..=reach {
            if gcd(a.unsigned_abs() as u128, c as u128) == 1 {
                out.push((a, c));
            }
        }
        c += 1;
    }
    out
}

fn trace(board: &mut Board, canvas: &Canvas, a: i64, c: i64, tone: Color) -> usize {
    let centre = a as f64 / c as f64;
    let radius = 1.0 / (2.0 * (c * c) as f64 * HEIGHT);
    let dt = STEP / (radius * canvas.scale);
    let steps = (std::f64::consts::TAU / dt).ceil() as usize;
    let mut run: Vec<(f64, f64)> = Vec::new();
    let mut runs = 0usize;
    for k in 0..=steps {
        let t = k as f64 * std::f64::consts::TAU / steps as f64 - std::f64::consts::FRAC_PI_2;
        let x = centre + radius * t.cos();
        let y = radius + radius * t.sin();
        if canvas.inside(x, y) {
            run.push(canvas.at(x, y));
        } else if !run.is_empty() {
            board.polyline(&run, THICK, tone);
            run.clear();
            runs += 1;
        }
    }
    if !run.is_empty() {
        board.polyline(&run, THICK, tone);
        runs += 1;
    }
    runs
}

fn outline(board: &mut Board, canvas: &Canvas) {
    let tone = ink::fg();
    let floor = canvas.floor;
    let top = canvas.top;
    board.segment(canvas.at(-0.5, floor), canvas.at(-0.5, top), EDGE, tone);
    board.segment(canvas.at(0.5, floor), canvas.at(0.5, top), EDGE, tone);
    let pts: Vec<(f64, f64)> = (0..=240)
        .map(|k| {
            let t = std::f64::consts::PI * (1.0 / 3.0 + k as f64 / 720.0);
            canvas.at(t.cos(), t.sin())
        })
        .collect();
    board.polyline(&pts, EDGE, tone);
}

fn fold(cusps: &[(i64, i64)]) -> usize {
    let known: BTreeSet<(i64, i64)> = cusps.iter().copied().collect();
    let mut hit = BTreeSet::new();
    for k in 0..SAMPLES {
        let (mut x, mut y) = ((k as f64 + 0.5) / SAMPLES as f64, HEIGHT);
        let mut m = [[1i64, 0], [0, 1]];
        loop {
            let n = x.round();
            x -= n;
            m = [
                [m[0][0] - n as i64 * m[1][0], m[0][1] - n as i64 * m[1][1]],
                m[1],
            ];
            let norm = x * x + y * y;
            if norm >= 1.0 {
                break;
            }
            (x, y) = (-x / norm, y / norm);
            m = [[-m[1][0], -m[1][1]], m[0]];
        }
        let (mut a, mut c) = (m[0][0], m[1][0]);
        if c < 0 {
            (a, c) = (-a, -c);
        }
        let centre = a as f64 / c as f64;
        let radius = 1.0 / (2.0 * (c * c) as f64 * HEIGHT);
        let off = ((x - centre).powi(2) + (y - radius).powi(2)).sqrt() - radius;
        assert!(off.abs() < 1e-6 * radius);
        assert!(known.contains(&(a, c)));
        hit.insert((a, c));
    }
    hit.len()
}
