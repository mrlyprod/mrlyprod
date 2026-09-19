use mrlycore::errors::Result;
use mrlycore::Color;
use mrlyfig::{ink, plot, save, Board, Frame};
use mrlynum::formulas;
use mrlynum::series::EULER;
use std::f64::consts::{E, PI};

const TOP: usize = 2000;
const FLOOR: usize = 2;
const SAMPLES: usize = 240;
const DECADES: f64 = 4.5;
const LEFT: f64 = FLOOR as f64;
const RIGHT: f64 = TOP as f64;

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

fn place(panel: Frame, at: f64, gauge: f64) -> Option<(f64, f64)> {
    if gauge <= 0.0 {
        return None;
    }
    let across = (at.log10() - LEFT.log10()) / (RIGHT.log10() - LEFT.log10());
    let drop = (-gauge.log10()).clamp(0.0, DECADES) / DECADES;
    Some((panel.x + panel.w * across, panel.y + panel.h * drop))
}

fn trace(panel: Frame, rungs: &[usize], gauge: impl Fn(usize) -> f64) -> Vec<(f64, f64)> {
    rungs
        .iter()
        .filter_map(|&m| place(panel, m as f64, gauge(m)))
        .collect()
}

fn chaser(board: &mut Board, panel: Frame, rungs: &[usize], partial: fn(usize) -> f64, limit: f64) {
    let path = trace(panel, rungs, |m| (partial(m) - limit).abs() / limit);
    board.polyline(&path, 2.6, ink::blue());
}

fn stage(board: &mut Board, panel: Frame, path: &[(f64, f64)], color: Color) {
    board.polyline(path, 2.6, color);
    plot::axis(board, panel, ink::line());
}

fn main() -> Result<()> {
    let rungs = ladder();
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let mut panels: Vec<Frame> = Vec::new();
    for row in frame.rows(2) {
        for cell in row.cols(4) {
            panels.push(Frame::new(
                cell.x + cell.w * 0.04,
                cell.y + cell.h * 0.09,
                cell.w * 0.92,
                cell.h * 0.82,
            ));
        }
    }
    assert_eq!(panels.len(), 8);

    chaser(&mut board, panels[0], &rungs, formulas::wallis, PI / 2.0);
    chaser(&mut board, panels[1], &rungs, formulas::leibniz, PI / 4.0);
    chaser(&mut board, panels[2], &rungs, formulas::basel, PI * PI / 6.0);
    chaser(&mut board, panels[3], &rungs, formulas::e_partial, E);
    chaser(
        &mut board,
        panels[4],
        &rungs,
        formulas::euler_gamma_partial,
        EULER,
    );
    for panel in &panels[..5] {
        plot::axis(&mut board, *panel, ink::line());
    }

    let counted = trace(panels[5], &rungs, |m| {
        let li = formulas::li(m as f64);
        (formulas::prime_count(m) as f64 - li).abs() / li
    });
    stage(&mut board, panels[5], &counted, ink::yellow());

    let comet = trace(panels[6], &rungs, |m| 1.0 / formulas::goldbach(2 * m) as f64);
    stage(&mut board, panels[6], &comet, ink::yellow());

    let meter = trace(panels[7], &rungs, |m| {
        formulas::mertens(m).unsigned_abs() as f64 / (m as f64).sqrt()
    });
    plot::dots(&mut board, &meter, 2.4, ink::orange());
    plot::axis(&mut board, panels[7], ink::line());

    assert_eq!(rungs.first().copied(), Some(FLOOR));
    assert_eq!(rungs.last().copied(), Some(TOP));
    assert_eq!(counted.len(), rungs.len());
    assert_eq!(comet.len(), rungs.len());
    assert!(meter.len() < rungs.len());
    save("wiki-famous-formulas", &board)?;
    Ok(())
}
