use mrlyfig::{ink, plot, save, Board, Frame};
use mrlyrs::core::errors::Result;

const TERMS: usize = 60;

fn dashed(board: &mut Board, frame: Frame, y: f64) {
    let step = frame.w / 48.0;
    let mut x = frame.x;
    while x < frame.x + frame.w {
        let to = (x + step * 0.55).min(frame.x + frame.w);
        board.segment((x, y), (to, y), 2.5, ink::dim());
        x += step;
    }
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let limit = std::f64::consts::E;
    let walk: Vec<f64> = (1..=TERMS)
        .map(|n| (1.0 + 1.0 / n as f64).powi(n as i32))
        .collect();
    let lo = walk[0] - 0.07;
    let hi = limit + 0.05;
    let slot = frame.w / TERMS as f64;
    let up = |v: f64| frame.y + frame.h * (1.0 - (v - lo) / (hi - lo));
    let pts: Vec<(f64, f64)> = walk
        .iter()
        .enumerate()
        .map(|(i, value)| (frame.x + (i as f64 + 0.5) * slot, up(*value)))
        .collect();
    plot::axis(&mut board, frame, ink::line());
    dashed(&mut board, frame, up(limit));
    board.polyline(&pts, 3.0, ink::blue());
    plot::dots(&mut board, &pts, slot * 0.22, ink::yellow());
    assert_eq!(walk.iter().filter(|value| **value < limit).count(), TERMS);
    save("wiki-eulers-number", &board)?;
    Ok(())
}
