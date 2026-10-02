use figures::board::{Board, Frame};
use figures::ink::Ramp;
use figures::{field, ink, save};
use mrlyrs::core::error::Result;
use mrlyrs::math::two::{carpet, Cell2d};

const LOW: usize = 21;
const HIGH: usize = 23;
const PANEL: usize = 410;
const EDGE: f64 = 82.0;
const STEPS: usize = 400;

fn inked(cell: &Cell2d) -> Result<Vec<Vec<bool>>> {
    let n = cell.width();
    let mut rows = vec![vec![false; n]; n];
    for (i, row) in rows.iter_mut().enumerate() {
        for (j, slot) in row.iter_mut().enumerate() {
            *slot = cell.types().get(&[i, j])? != 0;
        }
    }
    Ok(rows)
}

fn pieces(lo: f64, hi: f64) -> Vec<(f64, usize, usize)> {
    let mut cuts = vec![lo, hi];
    for n in [LOW, HIGH] {
        for k in 1..n {
            let x = k as f64 / n as f64;
            if x > lo && x < hi {
                cuts.push(x);
            }
        }
    }
    cuts.sort_by(|a, b| a.total_cmp(b));
    cuts.windows(2)
        .filter(|w| w[1] > w[0])
        .map(|w| {
            let mid = 0.5 * (w[0] + w[1]);
            let a = ((mid * LOW as f64) as usize).min(LOW - 1);
            let b = ((mid * HIGH as f64) as usize).min(HIGH - 1);
            (w[1] - w[0], a, b)
        })
        .collect()
}

fn overlay(low: &[Vec<bool>], high: &[Vec<bool>]) -> Vec<f64> {
    let side = PANEL as f64;
    let spans: Vec<Vec<(f64, usize, usize)>> = (0..PANEL)
        .map(|k| pieces(k as f64 / side, (k + 1) as f64 / side))
        .collect();
    let mut values = Vec::with_capacity(PANEL * PANEL);
    for rows in &spans {
        for cols in &spans {
            let mut area = 0.0;
            for &(ly, ra, rb) in rows {
                for &(lx, ca, cb) in cols {
                    if low[ra][ca] != high[rb][cb] {
                        area += lx * ly;
                    }
                }
            }
            values.push(area * side * side);
        }
    }
    values
}

fn limit(u: f64, v: f64) -> f64 {
    0.5 * (1.0 - (1.0 - 2.0 * u).abs() * (1.0 - 2.0 * v).abs())
}

fn fill(n: usize) -> f64 {
    let q = (n - 1) as f64 / (2 * n) as f64;
    1.0 - q * q
}

fn arcs(board: &mut Board, frame: Frame, kappa: f64, thick: f64, chords: bool) {
    for (sx, sy) in [(1.0, 1.0), (1.0, -1.0), (-1.0, 1.0), (-1.0, -1.0)] {
        if chords {
            let a = frame.at(0.5 * (1.0 - sx), 0.5 * (1.0 - sy * kappa));
            let b = frame.at(0.5 * (1.0 - sx * kappa), 0.5 * (1.0 - sy));
            board.segment(a, b, 2.0, ink::dim());
        }
        let arc: Vec<(f64, f64)> = (0..=STEPS)
            .map(|k| {
                let x = kappa + (1.0 - kappa) * k as f64 / STEPS as f64;
                frame.at(0.5 * (1.0 - sx * x), 0.5 * (1.0 - sy * kappa / x))
            })
            .collect();
        board.polyline(&arc, thick, ink::orange());
    }
}

fn main() -> Result<()> {
    let low = inked(&carpet(LOW, 1)?)?;
    let high = inked(&carpet(HIGH, 1)?)?;
    assert_eq!(low.iter().flatten().filter(|&&on| on).count(), 341);
    assert_eq!(high.iter().flatten().filter(|&&on| on).count(), 408);

    let raw = overlay(&low, &high);
    let mean = raw.iter().sum::<f64>() / raw.len() as f64;
    let (a, b) = (fill(LOW), fill(HIGH));
    assert!((mean - (a + b - 2.0 * a * b)).abs() < 1e-9);

    let side = PANEL as f64;
    let smooth: Vec<f64> = (0..PANEL * PANEL)
        .map(|k| {
            limit(
                ((k % PANEL) as f64 + 0.5) / side,
                ((k / PANEL) as f64 + 0.5) / side,
            )
        })
        .collect();
    let smean = smooth.iter().sum::<f64>() / smooth.len() as f64;
    assert!((smean - 0.375).abs() < 1e-12);

    let kappa = 2f64.sqrt() - 1.0;
    assert!((2.0 * kappa - 2f64.sqrt() * (1.0 - kappa)).abs() < 1e-12);

    let ramp = Ramp::tone(ink::panel(), ink::blue());
    let mut board = Board::square();
    let first = Frame::new(EDGE, EDGE, side, side);
    let far = board.width as f64 - EDGE - side;
    let second = Frame::new(far, far, side, side);
    field::draw_range(&mut board, first, PANEL, PANEL, &raw, (0.0, 1.0), &ramp);
    field::draw_range(&mut board, second, PANEL, PANEL, &smooth, (0.0, 0.5), &ramp);

    arcs(&mut board, second, kappa, 4.0, true);
    arcs(&mut board, first, kappa, 3.0, false);
    save("moire-local-limit", &board)?;
    Ok(())
}
