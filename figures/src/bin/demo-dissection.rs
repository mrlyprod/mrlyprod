use figures::{ink, save, Board};
use mrlyrs::core::error::Result;
use mrlyrs::num::dissection::{regions, weights, Region};
use std::f64::consts::TAU;

const NAME: &str = "demo-dissection";
const BASE: u64 = 10;
const MISSING: u64 = 7;
const LEVEL: u32 = 3;
const Z: u64 = 8;
const COUNTS: [usize; 4] = [732, 202, 26, 40];
const DECADES: f64 = 4.0;

fn slot(region: Region) -> usize {
    match region {
        Region::A => 0,
        Region::B => 1,
        Region::C1 => 2,
        Region::C2 => 3,
    }
}

fn main() -> Result<()> {
    let digits: Vec<u64> = (0..BASE).filter(|&d| d != MISSING).collect();
    let cut = regions(BASE, LEVEL, Z);
    let weight = weights(BASE, &digits, LEVEL);
    let y = cut.len();
    let scale = (digits.len() as f64).powi(LEVEL as i32);
    let mut counts = [0usize; 4];
    for r in &cut {
        counts[slot(*r)] += 1;
    }
    assert_eq!(counts, COUNTS);
    assert_eq!(y, 1000);
    assert!((weight[0] / scale - 1.0).abs() < 1e-12);

    let mut board = Board::square();
    let frame = board.frame(0.08);
    let (cx, cy) = frame.center();
    let outer = frame.radius();
    let inner = 0.36 * outer;
    let inks = [ink::blue(), ink::dim(), ink::orange(), ink::yellow()];
    let at = |r: f64, t: f64| (cx + r * t.sin(), cy - r * t.cos());
    let step = TAU / y as f64;
    board.ring(cx, cy, inner - 6.0, 1.0, ink::line());
    for pass in [1usize, 0, 2, 3] {
        for a in (0..y).filter(|&a| slot(cut[a]) == pass) {
            let w = weight[a] / scale;
            let reach = (1.0 + w.max(1e-12).log10() / DECADES).clamp(0.015, 1.0);
            let top = inner + (outer - inner) * reach;
            let mid = a as f64 * step;
            let half = 0.34 * step;
            let pts = [
                at(inner, mid - half),
                at(top, mid - half),
                at(top, mid + half),
                at(inner, mid + half),
            ];
            board.polygon(&pts, inks[pass]);
        }
    }
    save(NAME, &board)?;
    Ok(())
}
