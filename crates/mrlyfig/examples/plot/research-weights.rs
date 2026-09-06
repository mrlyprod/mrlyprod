use mrlycore::errors::Result;
use mrlycore::Color;
use mrlyfig::{ink, plot, save, Board, Frame};

const LEVEL: u32 = 8;
const SAMPLES: usize = 720;
const CELLS: [(i64, i64); 3] = [(0, 0), (2, 0), (0, 2)];

fn root(costs: [f64; 3]) -> f64 {
    let (mut lo, mut hi) = (0.0f64, 8.0f64);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        let mass: f64 = costs.iter().map(|c| (-mid * c).exp()).sum();
        if mass > 1.0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

fn pascal(n: usize) -> Vec<Vec<u128>> {
    let mut rows: Vec<Vec<u128>> = vec![vec![1]];
    for i in 1..=n {
        let mut row = vec![1u128; i + 1];
        for k in 1..i {
            row[k] = rows[i - 1][k - 1] + rows[i - 1][k];
        }
        rows.push(row);
    }
    rows
}

fn stopping(budget: f64, cheap: f64, dear: f64, binom: &[Vec<u128>]) -> u128 {
    let mut total = 0u128;
    let mut p = 0usize;
    while p as f64 * cheap <= budget {
        let mut q = 0usize;
        while p as f64 * cheap + q as f64 * dear <= budget {
            total += binom[p + q][q] << p;
            q += 1;
        }
        p += 1;
    }
    total
}

fn mass_side(dear: f64, span: (f64, f64)) -> Vec<f64> {
    let cheap = 2.0f64.ln();
    let delta = root([cheap, cheap, dear]);
    let binom = pascal(2 + (span.1 / cheap) as usize);
    (0..SAMPLES)
        .map(|i| {
            let a = span.0 + (span.1 - span.0) * i as f64 / (SAMPLES - 1) as f64;
            (stopping(a, cheap, dear, &binom) as f64).ln() - delta * a
        })
        .collect()
}

fn gasket(nums: [u64; 3]) -> Vec<(i64, i64, u128)> {
    let mut pts = vec![(0i64, 0i64, 1u128)];
    for _ in 0..LEVEL {
        let mut next = Vec::with_capacity(pts.len() * 3);
        for &(x, y, m) in &pts {
            for (k, &(dx, dy)) in CELLS.iter().enumerate() {
                next.push((x * 3 + dx, y * 3 + dy, m * nums[k] as u128));
            }
        }
        pts = next;
    }
    pts
}

fn length_side(nums: [u64; 3], den: u64) -> (f64, Vec<f64>) {
    let pts = gasket(nums);
    assert_eq!(pts.len(), 3usize.pow(LEVEL));
    let whole: u128 = pts.iter().map(|p| p.2).sum();
    assert_eq!(whole, (den as u128).pow(LEVEL));
    let mut rows: Vec<(u128, u128)> = pts
        .iter()
        .map(|&(x, y, m)| ((x * x + y * y) as u128, m))
        .collect();
    rows.sort_by_key(|row| row.0);
    let mut running = 0u128;
    let shells: Vec<(f64, f64)> = rows
        .iter()
        .map(|&(square, m)| {
            running += m;
            (square as f64, running as f64)
        })
        .collect();
    let base = 3.0f64.ln();
    let alpha = -(nums[0] as f64 / den as f64).ln() / base;
    let ys = (0..SAMPLES)
        .map(|i| {
            let u = 4.0 + 4.0 * i as f64 / (SAMPLES - 1) as f64;
            let radius = (u * base).exp();
            let reach = radius * radius;
            let cut = shells.partition_point(|shell| shell.0 <= reach);
            let held = if cut == 0 { 0.0 } else { shells[cut - 1].1 };
            held.ln() - LEVEL as f64 * (den as f64).ln() - alpha * u * base
        })
        .collect();
    (alpha, ys)
}

fn center(ys: &[f64]) -> Vec<f64> {
    let mean = ys.iter().sum::<f64>() / ys.len() as f64;
    ys.iter().map(|y| y - mean).collect()
}

fn reach(first: &[f64], second: &[f64]) -> f64 {
    first
        .iter()
        .chain(second.iter())
        .fold(0.0f64, |a, y| a.max(y.abs()))
        * 1.14
}

fn trace(board: &mut Board, area: Frame, ys: &[f64], span: f64, thick: f64, color: Color) {
    let last = (ys.len() - 1) as f64;
    let pts: Vec<(f64, f64)> = ys
        .iter()
        .enumerate()
        .map(|(i, y)| {
            (
                area.x + area.w * i as f64 / last,
                area.y + area.h * (0.5 - 0.5 * y / span),
            )
        })
        .collect();
    board.polyline(&pts, thick, color);
}

fn panel(board: &mut Board, area: Frame, first: &[f64], second: &[f64]) {
    plot::axis(board, area, ink::LINE);
    let inner = area.inset(18.0);
    let mid = inner.y + inner.h / 2.0;
    board.segment((inner.x, mid), (inner.x + inner.w, mid), 1.0, ink::LINE);
    let span = reach(first, second);
    trace(board, inner, second, span, 3.4, ink::ORANGE);
    trace(board, inner, first, span, 2.1, ink::BLUE);
}

fn main() -> Result<()> {
    let window = (48.0, 48.0 + 6.0 * 2.0f64.ln());
    let lattice = mass_side(4.0f64.ln(), window);
    let smooth = mass_side(3.0f64.ln(), window);
    assert!((root([2.0f64.ln(), 2.0f64.ln(), 4.0f64.ln()]) - 1.2715533032).abs() < 1e-9);
    assert!((root([2.0f64.ln(), 2.0f64.ln(), 3.0f64.ln()]) - 1.3646005647).abs() < 1e-9);

    let (thin, ring_thin) = length_side([2, 2, 1], 5);
    let (fat, ring_fat) = length_side([3, 3, 2], 8);
    assert!((thin - 0.834043767).abs() < 1e-9);
    assert!((fat - 0.892789261).abs() < 1e-9);

    let mut board = Board::square();
    let frame = board.frame(0.08);
    let half = frame.h / 2.0;
    let top = Frame::new(frame.x, frame.y, frame.w, half).inset(14.0);
    let foot = Frame::new(frame.x, frame.y + half, frame.w, half).inset(14.0);
    panel(&mut board, top, &center(&lattice), &center(&smooth));
    panel(&mut board, foot, &center(&ring_thin), &center(&ring_fat));
    save("research-weights", &board)?;
    Ok(())
}
