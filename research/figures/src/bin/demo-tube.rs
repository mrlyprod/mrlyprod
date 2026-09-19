use mrlycore::errors::Result;
use mrlyfig::board::{Board, Frame};
use mrlyfig::{ink, plot, save, Grid};
use mrlymath::two;

const CODE: u128 = 495;
const SIDE: usize = 3;
const LEVEL: usize = 5;
const SPAN: usize = 243;
const EPS: f64 = 7.0;
const CELL: f64 = 2.0;
const STEPS: usize = 1200;

fn lower(f: &[f64], d: &mut [f64], hull: &mut [usize], edge: &mut [f64]) {
    let n = f.len();
    let mut k = 0;
    hull[0] = 0;
    edge[0] = f64::NEG_INFINITY;
    edge[1] = f64::INFINITY;
    for q in 1..n {
        let mut cut;
        loop {
            let p = hull[k];
            cut = ((f[q] + (q * q) as f64) - (f[p] + (p * p) as f64)) / (2 * (q - p)) as f64;
            if k > 0 && cut <= edge[k] {
                k -= 1;
            } else {
                break;
            }
        }
        k += 1;
        hull[k] = q;
        edge[k] = cut;
        edge[k + 1] = f64::INFINITY;
    }
    k = 0;
    for (q, slot) in d.iter_mut().enumerate() {
        while edge[k + 1] < q as f64 {
            k += 1;
        }
        let gap = q as f64 - hull[k] as f64;
        *slot = gap * gap + f[hull[k]];
    }
}

fn transform(types: &[u8], side: usize) -> Vec<f64> {
    let far = (4 * side * side) as f64;
    let mut square: Vec<f64> = types
        .iter()
        .map(|&b| if b != 0 { 0.0 } else { far })
        .collect();
    let mut lane = vec![0.0; side];
    let mut out = vec![0.0; side];
    let mut hull = vec![0usize; side];
    let mut edge = vec![0.0; side + 1];
    for col in 0..side {
        for row in 0..side {
            lane[row] = square[row * side + col];
        }
        lower(&lane, &mut out, &mut hull, &mut edge);
        for row in 0..side {
            square[row * side + col] = out[row];
        }
    }
    for row in 0..side {
        lane.copy_from_slice(&square[row * side..(row + 1) * side]);
        lower(&lane, &mut out, &mut hull, &mut edge);
        square[row * side..(row + 1) * side].copy_from_slice(&out);
    }
    square.iter().map(|v| v.sqrt()).collect()
}

fn limit(t: f64) -> f64 {
    let dimension = 8f64.ln() / 3f64.ln();
    let (flat, slope, bend) = if t < 0.5 {
        (1.0, 4.0 / 5.0, -4.0 / 7.0)
    } else {
        (9.0 / 8.0, 3.0 / 10.0, -1.0 / 14.0)
    };
    t.powf(dimension - 2.0) * (flat + slope * t + bend * t * t)
}

fn main() -> Result<()> {
    let cells = two::create(CODE, SIDE, LEVEL, 0, SIDE)?;
    let types = cells.types().bytes().to_vec();
    assert_eq!(types.len(), SPAN * SPAN);
    assert_eq!(types.iter().filter(|&&b| b != 0).count(), 32768);
    let dist = transform(&types, SPAN);
    assert_eq!(dist.iter().filter(|&&v| v > 0.0 && v <= EPS).count(), 20440);

    let seam = limit(1.0 / 3.0);
    assert!((seam - 379.0 / 280.0).abs() < 1e-12);
    assert!((limit(1.0) - seam).abs() < 1e-12);
    let phases: Vec<f64> = (0..=STEPS)
        .map(|i| 1.0 / 3.0 + (2.0 / 3.0) * i as f64 / STEPS as f64)
        .collect();
    let profile: Vec<f64> = phases.iter().map(|&t| limit(t)).collect();
    let high = profile.iter().copied().fold(f64::MIN, f64::max);
    let low = profile.iter().copied().fold(f64::MAX, f64::min);
    assert!(((high - low) / low - 0.003662).abs() < 1e-5);

    let mut board = Board::square();
    let margin = (board.width as f64 * 0.08).round();
    let plate = CELL * SPAN as f64;
    let sheet = Frame::new(margin, margin, plate, plate);
    let lattice = Grid::new(sheet, SPAN, SPAN, 0.0);
    for row in 0..SPAN {
        for col in 0..SPAN {
            let at = row * SPAN + col;
            if types[at] != 0 {
                lattice.fill(&mut board, col, row, ink::fg());
            } else if dist[at] <= EPS {
                lattice.fill(&mut board, col, row, ink::blue());
            }
        }
    }
    let tall = 320.0;
    let panel = Frame::new(
        board.width as f64 - margin - plate,
        board.height as f64 - margin - tall,
        plate,
        tall,
    );
    plot::axis(&mut board, panel, ink::line());
    plot::curve(
        &mut board,
        panel.inset(24.0),
        &phases,
        &profile,
        4.0,
        ink::yellow(),
    );
    save("demo-tube", &board)?;
    Ok(())
}
