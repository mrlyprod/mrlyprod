use figures::{ink, plot, save, Board, Frame};
use mrlyrs::core::error::Result;
use mrlyrs::core::Color;
use mrlyrs::num::lattice::totients;
use mrlyrs::num::zeta::{novelty_main, novelty_wave, sharp_novelty, smoothed_novelty, Line};
use std::f64::consts::PI;

const REACH: usize = 3_000_000;
const LOW: f64 = 8.0;
const HIGH: f64 = 20.5;
const PER_OCTAVE: usize = 16;
const ZEROS: usize = 29;
const CURVE: usize = 2400;

fn beads(board: &mut Board, area: Frame, values: &[f64], span: f64, r: f64, color: Color) {
    let last = (values.len() - 1) as f64;
    let pts: Vec<(f64, f64)> = values
        .iter()
        .enumerate()
        .map(|(i, v)| {
            (
                area.x + area.w * i as f64 / last,
                area.y + area.h * (0.5 - 0.5 * v / span),
            )
        })
        .collect();
    plot::dots(board, &pts, r, color);
}

fn reach(values: &[f64]) -> f64 {
    values.iter().fold(0.0f64, |a, v| a.max(v.abs())) * 1.12
}

fn main() -> Result<()> {
    let phi = totients(REACH);
    assert_eq!(&phi[1..=10], &[1, 1, 2, 2, 4, 2, 6, 4, 6, 4]);
    let mut prefix = vec![0u64; REACH + 1];
    for n in 1..=REACH {
        prefix[n] = prefix[n - 1] + phi[n];
    }
    let total = prefix[REACH] as f64;
    let expect = 3.0 * (REACH as f64).powi(2) / (PI * PI);
    assert!((total - expect).abs() < REACH as f64 * (REACH as f64).ln());

    let main_bump = novelty_main();
    assert!((main_bump - 6.0 / (PI * PI) * 0.575_725_895_994).abs() < 1e-9);

    let samples = ((HIGH - LOW) * PER_OCTAVE as f64).round() as usize + 1;
    assert_eq!(samples, 201);
    let js: Vec<f64> = (0..samples)
        .map(|k| LOW + k as f64 / PER_OCTAVE as f64)
        .collect();
    let smooth: Vec<f64> = js
        .iter()
        .map(|&j| {
            let y = 2.0f64.powf(-j);
            smoothed_novelty(&phi, y, main_bump) / y.powf(1.5)
        })
        .collect();
    let rough: Vec<f64> = js
        .iter()
        .map(|&j| {
            let y = 2.0f64.powf(-j);
            sharp_novelty(&prefix, y) / y
        })
        .collect();

    let line = Line::new();
    let gammas = line.zeros(ZEROS);
    assert!((gammas[0] - 14.134_725).abs() < 1e-6);
    assert!((gammas[ZEROS - 1] - 98.831_194).abs() < 1e-5);
    let coef = line.novelty_coefficients(&gammas);
    assert!((coef[0].abs() - 0.1879).abs() < 5e-4);
    assert!((coef[9].abs() - 4.286e-3).abs() < 5e-6);
    let wave = |j: f64| novelty_wave(&gammas, &coef, -j * 2.0f64.ln());
    let peak = smooth.iter().fold(0.0f64, |a, v| a.max(v.abs()));
    let miss = js
        .iter()
        .zip(&smooth)
        .map(|(&j, &v)| (v - wave(j)).abs())
        .fold(0.0f64, f64::max);
    assert!(miss / peak < 1e-2);

    let mut board = Board::square();
    let frame = board.frame(0.08);
    let gap = 28.0;
    let top_h = frame.h * 0.62 - gap / 2.0;
    let top = Frame::new(frame.x, frame.y, frame.w, top_h);
    let foot = Frame::new(
        frame.x,
        frame.y + top_h + gap,
        frame.w,
        frame.h - top_h - gap,
    );
    for area in [top, foot] {
        plot::axis(&mut board, area, ink::line());
        let mid = area.y + area.h / 2.0;
        board.segment((area.x, mid), (area.x + area.w, mid), 1.0, ink::line());
    }
    let inner_top = top.inset(16.0);
    let inner_foot = foot.inset(16.0);
    let span_top = reach(&smooth);
    let span_foot = reach(&rough);
    let curve: Vec<(f64, f64)> = (0..=CURVE)
        .map(|k| {
            let t = k as f64 / CURVE as f64;
            let v = wave(LOW + (HIGH - LOW) * t);
            (
                inner_top.x + inner_top.w * t,
                inner_top.y + inner_top.h * (0.5 - 0.5 * v / span_top),
            )
        })
        .collect();
    board.polyline(&curve, 2.0, ink::orange());
    beads(&mut board, inner_top, &smooth, span_top, 3.4, ink::blue());
    beads(&mut board, inner_foot, &rough, span_foot, 3.4, ink::blue());
    save("paper-novelty-meter", &board)?;
    Ok(())
}
