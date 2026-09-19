use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board};
use mrlynum::factor::gcd;

const ORDER: usize = 8;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let step = frame.h / ORDER as f64;
    let radius = step * 0.06;
    let mut total = 0usize;
    let mut fresh = 0usize;
    for q in 1..=ORDER {
        let y = frame.y + (q as f64 - 0.5) * step;
        board.segment((frame.x, y), (frame.x + frame.w, y), 1.5, ink::line());
        for b in 1..=q {
            for a in 0..=b {
                if gcd(a, b) != 1 {
                    continue;
                }
                let x = frame.x + frame.w * a as f64 / b as f64;
                let new = b == q;
                board.disc(x, y, radius, if new { ink::yellow() } else { ink::blue() });
                total += 1;
                fresh += new as usize;
            }
        }
    }
    assert_eq!(total, 83);
    assert_eq!(fresh, 23);
    save("wiki-farey-sequence", &board)?;
    Ok(())
}
