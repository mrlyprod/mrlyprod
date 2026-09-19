use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board};
use mrlynum::zeta::Line;

const REACH: f64 = 50.0;
const STEPS: usize = 8000;

fn main() -> Result<()> {
    let line = Line::new();
    let zeros = line.zeros(10);
    assert_eq!(line.count(REACH), 10);
    assert!((zeros[0] - 14.134725).abs() < 1e-5);
    assert!(*zeros.last().unwrap() < REACH);
    let path: Vec<(f64, f64)> = (0..=STEPS)
        .map(|step| {
            let value = line.point(REACH * step as f64 / STEPS as f64).0;
            (value.re, value.im)
        })
        .collect();
    assert_eq!(path.len(), STEPS + 1);

    let mut board = Board::square();
    let frame = board.frame(0.08);
    let box_of = |pick: fn(&(f64, f64)) -> f64, fold: fn(f64, f64) -> f64| {
        path.iter().map(pick).fold(0.0, fold)
    };
    let (left, right) = (box_of(|p| p.0, f64::min), box_of(|p| p.0, f64::max));
    let (under, over) = (box_of(|p| p.1, f64::min), box_of(|p| p.1, f64::max));
    let unit = frame.w.min(frame.h) / (right - left).max(over - under);
    let (mid_x, mid_y) = frame.center();
    let cx = mid_x - 0.5 * (left + right) * unit;
    let cy = mid_y + 0.5 * (under + over) * unit;
    board.segment((frame.x, cy), (frame.x + frame.w, cy), 2.0, ink::line());
    board.segment((cx, frame.y), (cx, frame.y + frame.h), 2.0, ink::line());
    board.ring(cx, cy, unit, 2.0, ink::line());
    let plane: Vec<(f64, f64)> = path
        .iter()
        .map(|(re, im)| (cx + re * unit, cy - im * unit))
        .collect();
    board.polyline(&plane, 2.5, ink::blue());
    board.ring(cx, cy, unit * 0.07, 7.0, ink::yellow());
    save("demo-zeta", &board)?;
    Ok(())
}
