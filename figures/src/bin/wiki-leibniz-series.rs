use mrlyfig::{ink, plot, save, Board, Frame};
use mrlyrs::core::error::Result;

const TERMS: usize = 30;

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
    let limit = std::f64::consts::PI / 4.0;
    let mut walk = Vec::with_capacity(TERMS);
    let mut run = 0.0;
    for k in 0..TERMS {
        run += if k % 2 == 0 { 1.0 } else { -1.0 } / (2 * k + 1) as f64;
        walk.push(run);
    }
    let lo = walk.iter().copied().fold(f64::MAX, f64::min) - 0.05;
    let hi = walk.iter().copied().fold(f64::MIN, f64::max) + 0.05;
    let slot = frame.w / TERMS as f64;
    let up = |v: f64| frame.y + frame.h * (1.0 - (v - lo) / (hi - lo));
    let at = |i: usize| (frame.x + (i as f64 + 0.5) * slot, up(walk[i]));
    let pts: Vec<(f64, f64)> = (0..TERMS).map(at).collect();
    plot::axis(&mut board, frame, ink::line());
    dashed(&mut board, frame, up(limit));
    board.polyline(&pts, 2.0, ink::line());
    let mut over = 0usize;
    let mut under = 0usize;
    for (i, point) in pts.iter().enumerate() {
        let high = walk[i] > limit;
        board.disc(
            point.0,
            point.1,
            slot * 0.34,
            if high { ink::blue() } else { ink::orange() },
        );
        over += high as usize;
        under += !high as usize;
    }
    assert_eq!(over, 15);
    assert_eq!(under, 15);
    save("wiki-leibniz-series", &board)?;
    Ok(())
}
