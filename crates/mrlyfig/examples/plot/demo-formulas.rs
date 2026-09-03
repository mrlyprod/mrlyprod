use mrlycore::errors::Result;
use mrlycore::Color;
use mrlyfig::{ink, plot, save, Board, Frame};
use mrlynum::formulas;
use mrlynum::series::EULER;
use std::f64::consts::{E, PI};

const TOP: usize = 2000;
const FLOOR: usize = 2;
const SAMPLES: usize = 600;
const DECADES: f64 = 4.5;
const LEFT: f64 = FLOOR as f64;
const RIGHT: f64 = TOP as f64;

type Chaser = (fn(usize) -> f64, f64);

fn ladder() -> Vec<usize> {
    let (lo, hi) = ((FLOOR as f64).ln(), (TOP as f64).ln());
    let mut out: Vec<usize> = Vec::with_capacity(SAMPLES);
    for k in 0..SAMPLES {
        let step = lo + (hi - lo) * k as f64 / (SAMPLES - 1) as f64;
        let at = step.exp().round() as usize;
        if out.last() != Some(&at) {
            out.push(at);
        }
    }
    out
}

fn place(frame: Frame, at: f64, gauge: f64) -> Option<(f64, f64)> {
    if gauge <= 0.0 {
        return None;
    }
    let across = (at.log10() - LEFT.log10()) / (RIGHT.log10() - LEFT.log10());
    let drop = (-gauge.log10()).clamp(0.0, DECADES) / DECADES;
    Some((frame.x + frame.w * across, frame.y + frame.h * drop))
}

fn trace(frame: Frame, rungs: &[usize], gauge: impl Fn(usize) -> f64) -> Vec<(f64, f64)> {
    rungs
        .iter()
        .filter_map(|&m| place(frame, m as f64, gauge(m)))
        .collect()
}

fn stroke(board: &mut Board, pts: &[(f64, f64)], thick: f64, color: Color) {
    for pair in pts.windows(2) {
        board.segment(pair[0], pair[1], thick, color);
    }
}

fn rules(board: &mut Board, frame: Frame) {
    let faint = ink::fade(ink::DIM, 0.22);
    for step in 1..=DECADES as usize {
        let y = frame.y + frame.h * step as f64 / DECADES;
        board.segment((frame.x, y), (frame.x + frame.w, y), 1.2, faint);
    }
    for decade in [10.0f64, 100.0, 1000.0] {
        let across = (decade.log10() - LEFT.log10()) / (RIGHT.log10() - LEFT.log10());
        let x = frame.x + frame.w * across;
        board.segment((x, frame.y), (x, frame.y + frame.h), 1.2, faint);
    }
}

fn main() -> Result<()> {
    assert_eq!(formulas::prime_count(1000), 168);
    assert_eq!(formulas::goldbach(1000), 28);

    let rungs = ladder();
    assert_eq!(rungs.first().copied(), Some(FLOOR));
    assert_eq!(rungs.last().copied(), Some(TOP));

    let mut board = Board::square();
    let frame = board.frame(0.08);
    rules(&mut board, frame);
    plot::axis(&mut board, frame, ink::LINE);

    let chasers: [Chaser; 5] = [
        (formulas::wallis, PI / 2.0),
        (formulas::leibniz, PI / 4.0),
        (formulas::basel, PI * PI / 6.0),
        (formulas::euler_gamma_partial, EULER),
        (formulas::e_partial, E),
    ];
    let mut curves = 0;
    for (partial, limit) in chasers {
        let path = trace(frame, &rungs, |m| (partial(m) - limit).abs() / limit);
        assert_eq!(path.len(), rungs.len());
        stroke(&mut board, &path, 2.2, ink::BLUE);
        curves += 1;
    }

    let counted = trace(frame, &rungs, |m| {
        let li = formulas::li(m as f64);
        (formulas::prime_count(m) as f64 - li).abs() / li
    });
    assert_eq!(counted.len(), rungs.len());
    stroke(&mut board, &counted, 3.0, ink::GOLD);
    curves += 1;

    let comet = trace(frame, &rungs, |m| 1.0 / formulas::goldbach(2 * m) as f64);
    assert_eq!(comet.len(), rungs.len());
    plot::dots(&mut board, &comet, 2.0, ink::GOLD);
    curves += 1;

    let meter = trace(frame, &rungs, |m| {
        formulas::mertens(m).unsigned_abs() as f64 / (m as f64).sqrt()
    });
    assert!(meter.len() < rungs.len());
    plot::dots(&mut board, &meter, 1.9, ink::ORANGE);
    curves += 1;

    assert_eq!(curves, 8);
    save("demo-formulas", &board)?;
    Ok(())
}
