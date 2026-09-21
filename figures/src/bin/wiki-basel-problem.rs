use mrlyfig::{ink, plot, save, Board, Frame};
use mrlyrs::core::errors::Result;

const SQUARES: usize = 12;
const TERMS: usize = 40;

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
    let top = Frame::new(frame.x, frame.y, frame.w, frame.h * 0.38);
    let chart = Frame::new(frame.x, frame.y + frame.h * 0.46, frame.w, frame.h * 0.54);
    let reach: f64 = (1..=SQUARES).map(|k| 1.0 / k as f64).sum();
    let unit = top.w / reach;
    let base = top.y + top.h;
    let mut x = top.x;
    let mut laid = 0usize;
    for k in 1..=SQUARES {
        let side = unit / k as f64;
        board.rect(x, base - side, side - 2.0, side, ink::blue());
        x += side;
        laid += 1;
    }
    plot::baseline(
        &mut board,
        Frame::new(top.x, top.y, top.w, base - top.y),
        ink::line(),
    );
    let limit = std::f64::consts::PI * std::f64::consts::PI / 6.0;
    let mut walk = Vec::with_capacity(TERMS);
    let mut run = 0.0;
    for k in 1..=TERMS {
        run += 1.0 / (k * k) as f64;
        walk.push(run);
    }
    let lo = walk[0] - 0.14;
    let hi = limit + 0.12;
    let foot = chart.y + chart.h;
    let up = |v: f64| foot - chart.h * (v - lo) / (hi - lo);
    let slot = chart.w / TERMS as f64;
    let pad = slot * 0.16;
    for (i, value) in walk.iter().enumerate() {
        let y = up(*value);
        board.rect(
            chart.x + i as f64 * slot + pad,
            y,
            slot - 2.0 * pad,
            foot - y,
            ink::yellow(),
        );
    }
    dashed(&mut board, chart, up(limit));
    plot::axis(&mut board, chart, ink::line());
    assert_eq!(laid, SQUARES);
    assert_eq!(walk.iter().filter(|value| **value < limit).count(), TERMS);
    save("wiki-basel-problem", &board)?;
    Ok(())
}
