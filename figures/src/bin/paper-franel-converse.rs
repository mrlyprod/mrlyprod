use mrlyfig::{ink, plot, save, Board, Frame};
use mrlyrs::core::errors::Result;
use mrlyrs::core::Color;
use mrlyrs::num::design::elements;
use mrlyrs::num::factor::{gcd, mobius_sieve};
use mrlyrs::num::lattice::farey;

const BASE: u64 = 3;
const DIGITS: [u64; 2] = [0, 1];
const ORDER: usize = 81;

// THE NODES

fn nodes_of(dens: &[usize]) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    for &b in dens {
        for a in 1..=b {
            if gcd(a, b) == 1 {
                out.push((a, b));
            }
        }
    }
    out.sort_by(|p, q| (p.0 * q.1).cmp(&(q.0 * p.1)));
    out
}

fn sawtooth(nodes: &[(usize, usize)]) -> Vec<f64> {
    let m = nodes.len() as f64;
    nodes
        .iter()
        .enumerate()
        .map(|(j, &(a, b))| a as f64 / b as f64 - (j + 1) as f64 / m)
        .collect()
}

fn kernel(dilates: &[i64]) -> f64 {
    let mut total = 0.0;
    for d in 1..dilates.len() {
        if dilates[d] == 0 {
            continue;
        }
        for e in 1..dilates.len() {
            if dilates[e] == 0 {
                continue;
            }
            let g = gcd(d, e) as f64;
            total += g * g / (d as f64 * e as f64) * (dilates[d] * dilates[e]) as f64;
        }
    }
    total
}

fn dilated(holds: &[bool], mu: &[i8]) -> Vec<i64> {
    let q = holds.len() - 1;
    let mut out = vec![0i64; q + 1];
    for d in 1..=q {
        for c in 1..=q / d {
            if holds[d * c] {
                out[d] += mu[c] as i64;
            }
        }
    }
    out
}

// THE MARKS

fn ticks(board: &mut Board, area: Frame, nodes: &[(usize, usize)], thick: f64, color: Color) {
    let reach = (ORDER as f64).ln();
    let foot = area.y + area.h;
    for &(a, b) in nodes {
        let x = area.x + area.w * a as f64 / b as f64;
        let h = area.h * (1.0 - 0.82 * (b as f64).ln() / reach);
        board.segment((x, foot), (x, foot - h), thick, color);
    }
}

fn trace(
    board: &mut Board,
    area: Frame,
    nodes: &[(usize, usize)],
    delta: &[f64],
    span: f64,
    thick: f64,
    color: Color,
) {
    let pts: Vec<(f64, f64)> = nodes
        .iter()
        .zip(delta)
        .map(|(&(a, b), &v)| {
            (
                area.x + area.w * a as f64 / b as f64,
                area.y + area.h * (0.5 - 0.5 * v / span),
            )
        })
        .collect();
    board.polyline(&pts, thick, color);
}

fn main() -> Result<()> {
    let design: Vec<usize> = elements(BASE, &DIGITS, 5)
        .into_iter()
        .filter(|&n| n as usize <= ORDER)
        .map(|n| n as usize)
        .collect();
    assert_eq!(design.len(), 16);
    assert_eq!(design[..5], [1, 3, 4, 9, 10]);
    let mut holds = vec![false; ORDER + 1];
    for &b in &design {
        holds[b] = true;
    }
    let thin = nodes_of(&design);
    assert_eq!(thin.len(), 241);
    let full: Vec<(usize, usize)> = farey(ORDER)
        .iter()
        .filter(|n| n.num > 0)
        .map(|n| (n.num as usize, n.den as usize))
        .collect();
    assert_eq!(full.len(), 2020);
    assert_eq!(full, nodes_of(&(1..=ORDER).collect::<Vec<usize>>()));

    let delta_thin = sawtooth(&thin);
    let delta_full = sawtooth(&full);
    let s2_thin: f64 = delta_thin.iter().map(|v| v * v).sum();
    let s2_full: f64 = delta_full.iter().map(|v| v * v).sum();
    assert!((s2_thin - 0.004_002_104_850).abs() < 1e-9);
    assert!((s2_full - 0.006_524_653_839).abs() < 1e-9);

    let mu = mobius_sieve(ORDER);
    let x_thin = dilated(&holds, &mu);
    assert_eq!(x_thin[1], -2);
    assert_eq!(x_thin.iter().map(|v| v * v).sum::<i64>(), 19);
    let g_thin = kernel(&x_thin);
    assert!((g_thin - 12.574_087).abs() < 1e-5);
    assert!((g_thin - (12.0 * 241.0 * s2_thin + 1.0)).abs() < 1e-8);
    let x_full = dilated(&[true; ORDER + 1], &mu);
    assert_eq!(x_full[1], -4);
    let g_full = kernel(&x_full);
    assert!((g_full - 159.157_609).abs() < 1e-5);
    assert!((g_full - (12.0 * 2020.0 * s2_full + 1.0)).abs() < 1e-8);

    let mut board = Board::square();
    let frame = board.frame(0.08);
    let gap = 26.0;
    let row = (frame.h - 2.0 * gap) * 0.21;
    let top = Frame::new(frame.x, frame.y, frame.w, row);
    let mid = Frame::new(frame.x, frame.y + row + gap, frame.w, row);
    let foot_y = frame.y + 2.0 * (row + gap);
    let foot = Frame::new(frame.x, foot_y, frame.w, frame.y + frame.h - foot_y);

    plot::baseline(&mut board, top, ink::line());
    plot::baseline(&mut board, mid, ink::line());
    plot::axis(&mut board, foot, ink::line());
    let zero = foot.y + foot.h / 2.0;
    board.segment((foot.x, zero), (foot.x + foot.w, zero), 1.0, ink::line());

    ticks(&mut board, top.inset(2.0), &thin, 2.2, ink::blue());
    ticks(
        &mut board,
        mid.inset(2.0),
        &full,
        1.0,
        ink::fade(ink::dim(), 0.55),
    );

    let inner = foot.inset(16.0);
    let span = delta_thin.iter().fold(0.0f64, |a, v| a.max(v.abs())) * 1.12;
    trace(
        &mut board,
        inner,
        &full,
        &delta_full,
        span,
        1.2,
        ink::fade(ink::dim(), 0.7),
    );
    trace(
        &mut board,
        inner,
        &thin,
        &delta_thin,
        span,
        2.4,
        ink::orange(),
    );
    plot::dots(
        &mut board,
        &thin
            .iter()
            .zip(&delta_thin)
            .map(|(&(a, b), &v)| {
                (
                    inner.x + inner.w * a as f64 / b as f64,
                    inner.y + inner.h * (0.5 - 0.5 * v / span),
                )
            })
            .collect::<Vec<_>>(),
        2.6,
        ink::blue(),
    );
    save("paper-franel-converse", &board)?;
    Ok(())
}
