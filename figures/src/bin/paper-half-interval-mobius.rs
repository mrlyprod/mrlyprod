use figures::{ink, save, Board};
use mrlyrs::core::error::Result;
use std::f64::consts::PI;

const NAME: &str = "paper-half-interval-mobius";
const GAMMA: f64 = 0.577_215_664_901_532_9;
const LOW: f64 = 2.0;
const HIGH: f64 = 7.0;
const SAMPLES: usize = 1600;
const FIFTH: u64 = 94939;
const QUARTER: u64 = 3789;

// CERTIFICATE

fn rate(base: u64) -> f64 {
    let b = base as f64;
    let fill = (b + 1.0) / 2.0;
    let cp = 2f64.sqrt() - 4.0 / PI;
    let tan = (3.0 * PI / 8.0 + PI / (4.0 * b)).tan();
    let x0 = (b / PI) * ((b + 3.0).ln() + GAMMA + tan.ln()) + cp * (b + 1.0).powi(2) / (8.0 * b);
    (fill + x0 + 0.5 / (PI / (2.0 * b)).sin()) / fill
}

fn margin(base: u64, bar: f64) -> f64 {
    bar - rate(base).ln() / (base as f64).ln()
}

fn odd(x: f64) -> u64 {
    let n = x.round() as u64;
    if n % 2 == 1 {
        n
    } else {
        n + 1
    }
}

// FIGURE

fn main() -> Result<()> {
    assert!(margin(FIFTH - 2, 0.2) < 0.0 && margin(FIFTH, 0.2) > 0.0);
    assert!(margin(QUARTER - 2, 0.25) < 0.0 && margin(QUARTER, 0.25) > 0.0);
    assert!((1.3678e-8..1.3679e-8).contains(&margin(FIFTH, 0.2)));
    assert!((7.9625e-6..7.9626e-6).contains(&margin(QUARTER, 0.25)));
    assert!((0.199_387_1..=0.199_387_2).contains(&(0.2 - margin(100_003, 0.2))));

    let mut bases: Vec<u64> = (0..=SAMPLES)
        .map(|i| odd(10f64.powf(LOW + (HIGH - LOW) * i as f64 / SAMPLES as f64)))
        .collect();
    bases.extend([QUARTER - 2, QUARTER, FIFTH - 2, FIFTH]);
    bases.sort_unstable();
    bases.dedup();
    for &b in &bases {
        assert_eq!(margin(b, 0.2) > 0.0, b >= FIFTH);
        assert_eq!(margin(b, 0.25) > 0.0, b >= QUARTER);
    }

    let mut board = Board::square();
    let frame = board.frame(0.08);
    let top = margin(*bases.last().unwrap(), 0.25);
    let bottom = margin(bases[0], 0.2);
    let pad = 0.04 * (top - bottom);
    let (hi, lo) = (top + pad, bottom - pad);
    let at = |b: u64, m: f64| {
        let u = ((b as f64).log10() - LOW) / (HIGH - LOW);
        (
            frame.x + frame.w * u,
            frame.y + frame.h * (hi - m) / (hi - lo),
        )
    };

    for decade in 3..7 {
        let x = frame.x + frame.w * (decade as f64 - LOW) / (HIGH - LOW);
        board.segment((x, frame.y), (x, frame.y + frame.h), 1.4, ink::line());
    }
    let zero = at(bases[0], 0.0).1;
    board.segment((frame.x, zero), (frame.x + frame.w, zero), 2.2, ink::dim());

    for (bar, wall, color) in [(0.25, QUARTER, ink::orange()), (0.2, FIFTH, ink::blue())] {
        let pts: Vec<(f64, f64)> = bases.iter().map(|&b| at(b, margin(b, bar))).collect();
        board.polyline(&pts, 5.0, color);
        let (x, y) = at(wall, 0.0);
        board.disc(x, y, 15.0, ink::ground());
        board.disc(x, y, 9.5, color);
    }
    save(NAME, &board)?;
    Ok(())
}
