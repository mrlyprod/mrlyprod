use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board};
use mrlynum::factor::gcd;
use mrlynum::lattice::coprime_pairs;

const N: usize = 100;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let step = frame.w / N as f64;
    let lit = step * 0.52;
    let hid = step * 0.24;
    let mut visible = 0u64;
    for a in 1..=N {
        for b in 1..=N {
            let x = frame.x + (a as f64 - 0.5) * step;
            let y = frame.y + frame.h - (b as f64 - 0.5) * step;
            if gcd(a, b) == 1 {
                visible += 1;
                board.rect(x - lit / 2.0, y - lit / 2.0, lit, lit, ink::GOLD);
            } else {
                board.rect(
                    x - hid / 2.0,
                    y - hid / 2.0,
                    hid,
                    hid,
                    ink::fade(ink::DIM, 0.9),
                );
            }
        }
    }
    assert_eq!(visible, coprime_pairs(N));
    save("research-pi", &board)?;
    Ok(())
}
