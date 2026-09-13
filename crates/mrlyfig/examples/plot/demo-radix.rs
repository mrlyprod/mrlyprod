use mrlycore::errors::Result;
use mrlyfig::{ink, plot, save, Board, Frame};
use mrlynum::radix::flowsnake;

const LEVEL: usize = 6;
const WORDS: usize = 117649;
const DIGITS: usize = 7;
const NORM: u64 = 7;
const MARGIN: f64 = 0.08;
const DOT: f64 = 1.2;

fn main() -> Result<()> {
    let design = flowsnake();
    assert_eq!(design.size(), DIGITS);
    assert_eq!(design.base().norm(), NORM);
    assert_eq!(design.fill(LEVEL), WORDS as u128);
    assert_eq!(design.distinct(LEVEL), WORDS);

    let cloud = design.plane(LEVEL);
    assert_eq!(cloud.len(), WORDS);

    let mut board = Board::square();
    let frame = board.frame(MARGIN);
    let laid = fit(&cloud, frame);
    plot::dots(&mut board, &laid, DOT, ink::blue());
    save("demo-radix", &board)?;
    Ok(())
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
