use mrlyfig::{ink, plot, save, Board, Frame};
use mrlyrs::core::error::Result;
use mrlyrs::num::lattice::totients;
use mrlyrs::num::zeta::{novelty_main, novelty_wave, smoothed_novelty, Line};

const LOW: f64 = 8.0;
const HIGH: f64 = 14.0;
const PER_OCTAVE: usize = 16;
const ONE: usize = 1;
const MANY: usize = 30;
const CURVE: usize = 2400;

fn trace(area: Frame, span: f64, values: &[f64]) -> Vec<(f64, f64)> {
    let last = (values.len() - 1) as f64;
    values
        .iter()
        .enumerate()
        .map(|(i, v)| {
            (
                area.x + area.w * i as f64 / last,
                area.y + area.h * (0.5 - 0.5 * v / span),
            )
        })
        .collect()
}

fn main() -> Result<()> {
    let reach = 2.0f64.powf(HIGH + 1.0) as usize;
    let phi = totients(reach);
    assert_eq!(reach, 32_768);
    assert_eq!(&phi[1..=10], &[1, 1, 2, 2, 4, 2, 6, 4, 6, 4]);
    let main = novelty_main();
    let samples = ((HIGH - LOW) * PER_OCTAVE as f64).round() as usize + 1;
    assert_eq!(samples, 97);
    let js: Vec<f64> = (0..samples)
        .map(|k| LOW + k as f64 / PER_OCTAVE as f64)
        .collect();
    let dots: Vec<f64> = js
        .iter()
        .map(|&j| {
            let y = 2.0f64.powf(-j);
            smoothed_novelty(&phi, y, main) / y.powf(1.5)
        })
        .collect();
    let line = Line::new();
    let gammas = line.zeros(MANY);
    assert!((gammas[0] - 14.134_725).abs() < 1e-6);
    let coef = line.novelty_coefficients(&gammas);
    assert!((coef[0].abs() - 0.1879).abs() < 5e-4);
    let wave =
        |count: usize, j: f64| novelty_wave(&gammas[..count], &coef[..count], -j * 2.0f64.ln());
    let peak = dots.iter().fold(0.0f64, |a, v| a.max(v.abs()));
    let miss = |count: usize| {
        js.iter()
            .zip(&dots)
            .map(|(&j, &v)| (v - wave(count, j)).abs())
            .fold(0.0f64, f64::max)
            / peak
    };
    assert!(miss(MANY) < 1e-2);
    assert!(miss(ONE) > 0.3 && miss(ONE) < 0.7);
    let sweep = |count: usize| -> Vec<f64> {
        (0..=CURVE)
            .map(|k| wave(count, LOW + (HIGH - LOW) * k as f64 / CURVE as f64))
            .collect()
    };
    let (one, many) = (sweep(ONE), sweep(MANY));

    let mut board = Board::square();
    let frame = board.frame(0.08);
    plot::axis(&mut board, frame, ink::line());
    let mid = frame.y + frame.h / 2.0;
    board.segment((frame.x, mid), (frame.x + frame.w, mid), 1.5, ink::line());
    let inner = frame.inset(22.0);
    let span = peak * 1.12;
    board.polyline(&trace(inner, span, &one), 4.0, ink::dim());
    board.polyline(&trace(inner, span, &many), 2.0, ink::orange());
    plot::dots(&mut board, &trace(inner, span, &dots), 5.0, ink::blue());
    save("demo-novelty", &board)?;
    Ok(())
}
