use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board, Ramp};
use mrlynum::factor::gcd;

const ORDER: usize = 8;
const THICK: f64 = 2.6;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.06);
    let ramp = Ramp::tone(ink::blue(), ink::yellow());
    let base = frame.y + frame.h;
    let mut circles = 0usize;
    for b in 1..=ORDER {
        for a in 0..=b {
            if gcd(a, b) != 1 {
                continue;
            }
            let r = frame.w / (2 * b * b) as f64;
            let x = frame.x + frame.w * a as f64 / b as f64;
            let tone = ramp.at((b - 1) as f64 / (ORDER - 1) as f64);
            board.ring(x, base - r, r, THICK, tone);
            board.disc(x, base, THICK * 1.3, tone);
            circles += 1;
        }
    }
    board.segment((frame.x, base), (frame.x + frame.w, base), 1.6, ink::line());
    assert_eq!(circles, 23);
    save("wiki-ford-circles", &board)?;
    Ok(())
}
