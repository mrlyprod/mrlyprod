use mrlycore::errors::Result;
use mrlyfig::{ink, plot, save, Board, Frame};

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
    let limit = std::f64::consts::PI / 2.0;
    let mut walk = Vec::with_capacity(TERMS);
    let mut run = 1.0;
    for k in 1..=TERMS {
        let k = k as f64;
        run *= 4.0 * k * k / (4.0 * k * k - 1.0);
        walk.push(run);
    }
    let lo = walk[0] - 0.06;
    let hi = limit + 0.04;
    let foot = frame.y + frame.h;
    let up = |v: f64| foot - frame.h * (v - lo) / (hi - lo);
    let slot = frame.w / TERMS as f64;
    let pad = slot * 0.16;
    for (i, value) in walk.iter().enumerate() {
        let y = up(*value);
        board.rect(
            frame.x + i as f64 * slot + pad,
            y,
            slot - 2.0 * pad,
            foot - y,
            ink::blue(),
        );
    }
    dashed(&mut board, frame, up(limit));
    plot::axis(&mut board, frame, ink::line());
    assert_eq!(walk.iter().filter(|value| **value < limit).count(), TERMS);
    save("wiki-wallis-product", &board)?;
    Ok(())
}
