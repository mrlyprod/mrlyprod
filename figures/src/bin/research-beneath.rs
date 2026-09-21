use mrlyfig::{ink, save, Board, Frame};
use mrlyrs::core::errors::Result;
use mrlyrs::num::radix::koch;

const LEVEL: usize = 6;
const POINTS: usize = 4096;
const DIGITS: usize = 4;
const NORM: u64 = 9;
const PROBE: usize = 2;
const STEP: f64 = 1.0 / 9.0;
const TOL: f64 = 1e-12;
const MARGIN: f64 = 0.08;
const THIN: f64 = 1.4;

fn main() -> Result<()> {
    let design = koch();
    assert_eq!(design.size(), DIGITS);
    assert_eq!(design.base().norm(), NORM);
    assert_eq!(design.fill(LEVEL), POINTS as u128);

    let probe = design.plane(PROBE);
    assert_eq!(probe.len(), DIGITS * DIGITS);
    assert!(steps(&probe).iter().all(|s| (s - STEP).abs() < TOL));

    let curve = design.plane(LEVEL);
    assert_eq!(curve.len(), POINTS);

    let mut board = Board::square();
    let frame = board.frame(MARGIN);
    let laid = fit(&curve, frame);
    board.polyline(&laid, THIN, ink::blue());
    save("research-beneath", &board)?;
    Ok(())
}

// THE STEP

fn steps(pts: &[(f64, f64)]) -> Vec<f64> {
    pts.windows(2)
        .map(|w| ((w[1].0 - w[0].0).powi(2) + (w[1].1 - w[0].1).powi(2)).sqrt())
        .collect()
}

// THE LAYOUT

fn fit(pts: &[(f64, f64)], frame: Frame) -> Vec<(f64, f64)> {
    let (mut low, mut high) = ((f64::MAX, f64::MAX), (f64::MIN, f64::MIN));
    for &(x, y) in pts {
        low = (low.0.min(x), low.1.min(y));
        high = (high.0.max(x), high.1.max(y));
    }
    let span = (high.0 - low.0, high.1 - low.1);
    let scale = (frame.w / span.0).min(frame.h / span.1);
    let mid = ((low.0 + high.0) / 2.0, (low.1 + high.1) / 2.0);
    let (cx, cy) = frame.center();
    pts.iter()
        .map(|&(x, y)| (cx + (x - mid.0) * scale, cy - (y - mid.1) * scale))
        .collect()
}
