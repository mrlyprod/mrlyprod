use figures::{ink, save, Board};
use mrlyrs::core::error::Result;
use mrlyrs::num::prime::flags;
use mrlyrs::num::spiral::Lattice;

const SIDE: usize = 101;
const REACH: i64 = 50;
const PRIMES: usize = 1252;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.06);
    let unit = frame.w / SIDE as f64;
    let top = SIDE * SIDE;
    let prime = flags(top);
    let mut lit = 0usize;
    for n in 1..=top as u64 {
        if !prime[n as usize] {
            continue;
        }
        let (x, y) = Lattice::Square.xy(n);
        let px = frame.x + (x + REACH) as f64 * unit;
        let py = frame.y + (REACH - y) as f64 * unit;
        board.rect(px, py, unit * 0.86, unit * 0.86, ink::blue());
        lit += 1;
    }
    assert_eq!(lit, PRIMES);
    save("wiki-ulam-spiral", &board)?;
    Ok(())
}
