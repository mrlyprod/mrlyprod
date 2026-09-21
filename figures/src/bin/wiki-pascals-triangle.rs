use mrlyfig::{ink, save, Board};
use mrlyrs::core::error::Result;

const ROWS: usize = 64;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.06);
    let step = frame.w / ROWS as f64;
    let (cx, _) = frame.center();
    let mut odd = 0usize;
    for n in 0..ROWS {
        let y = frame.y + (n as f64 + 0.5) * step;
        for k in 0..=n {
            let x = cx + (k as f64 - n as f64 / 2.0) * step;
            if n & k == k {
                board.disc(x, y, step * 0.42, ink::blue());
                odd += 1;
            } else {
                board.disc(x, y, step * 0.11, ink::dim());
            }
        }
    }
    assert_eq!(odd, 729);
    save("wiki-pascals-triangle", &board)?;
    Ok(())
}
