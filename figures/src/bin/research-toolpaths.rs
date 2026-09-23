use figures::{ink, save, Board};
use mrlyrs::core::error::Result;

const DIRS: [usize; 7] = [0, 5, 3, 4, 0, 0, 1];
const FLIPS: [bool; 7] = [false, true, true, false, false, false, true];
const LEVEL: usize = 4;
const STEPS: usize = 2401;
const COUNTS: [usize; 3] = [343, 1371, 686];
const MARGIN: f64 = 0.08;
const PATH: f64 = 2.2;
const DOT: f64 = 3.1;

fn expand(seq: &[(usize, bool)]) -> Vec<(usize, bool)> {
    let mut out = Vec::with_capacity(seq.len() * 7);
    for &(d, r) in seq {
        for j in 0..7 {
            let i = if r { 6 - j } else { j };
            out.push(((d + DIRS[i]) % 6, FLIPS[i] != r));
        }
    }
    out
}

fn main() -> Result<()> {
    let mut seq = vec![(0usize, false)];
    for _ in 0..LEVEL {
        seq = expand(&seq);
    }
    assert_eq!(seq.len(), STEPS);
    let unit: Vec<(f64, f64)> = (0..6)
        .map(|d| {
            let a = std::f64::consts::PI / 3.0 * d as f64;
            (a.cos(), a.sin())
        })
        .collect();
    let mut pts = vec![(0.0, 0.0)];
    for &(d, _) in &seq {
        let (x, y) = pts[pts.len() - 1];
        pts.push((x + unit[d].0, y + unit[d].1));
    }
    let mut counts = [0usize; 3];
    let mut sharp = Vec::new();
    for i in 1..seq.len() {
        let t = (seq[i].0 + 6 - seq[i - 1].0) % 6;
        let t = t.min(6 - t);
        counts[t] += 1;
        if t == 2 {
            sharp.push(pts[i]);
        }
    }
    assert_eq!(counts, COUNTS);
    let (mut x0, mut y0, mut x1, mut y1) = (f64::MAX, f64::MAX, f64::MIN, f64::MIN);
    for &(x, y) in &pts {
        x0 = x0.min(x);
        y0 = y0.min(y);
        x1 = x1.max(x);
        y1 = y1.max(y);
    }
    let mut board = Board::square();
    let area = board.frame(MARGIN);
    let (cx, cy) = area.center();
    let scale = 2.0 * area.radius() / (x1 - x0).max(y1 - y0);
    let (mx, my) = ((x0 + x1) / 2.0, (y0 + y1) / 2.0);
    let at = |(x, y): (f64, f64)| (cx + (x - mx) * scale, cy - (y - my) * scale);
    let path: Vec<(f64, f64)> = pts.iter().map(|&p| at(p)).collect();
    board.polyline(&path, PATH, ink::dim());
    for &p in &sharp {
        let (x, y) = at(p);
        board.disc(x, y, DOT, ink::orange());
    }
    save("research-toolpaths", &board)?;
    Ok(())
}
