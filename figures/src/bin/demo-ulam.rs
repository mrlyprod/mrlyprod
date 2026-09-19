use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board};
use mrlynum::spiral::{diagonal, flags, Lattice};

const SIDE: usize = 201;
const REACH: i64 = 100;
const UNIT: f64 = 4.0;
const DOT: f64 = 3.0;

fn main() -> Result<()> {
    let mut board = Board::square();
    let span = SIDE as f64 * UNIT;
    let edge = (board.width as f64 - span) / 2.0;
    let corner = |(x, y): (i64, i64)| {
        (
            edge + (x + REACH) as f64 * UNIT,
            edge + (REACH - y) as f64 * UNIT,
        )
    };
    let line = diagonal(Lattice::Square, SIDE, 4, -2, 41);
    assert_eq!((line.top, line.primes), (40401, 4236));
    assert_eq!((line.values.len(), line.hits, line.streak), (101, 80, 21));
    for cell in &line.cells {
        let (x, y) = corner(*cell);
        board.rect(x - 2.0, y - 2.0, UNIT + 4.0, UNIT + 4.0, ink::orange());
    }
    let prime = flags(line.top);
    let mut lit = 0usize;
    for n in 1..=line.top as u64 {
        if !prime[n as usize] {
            continue;
        }
        let (x, y) = corner(Lattice::Square.xy(n));
        board.rect(x, y, DOT, DOT, ink::blue());
        lit += 1;
    }
    assert_eq!(lit, line.primes);
    save("demo-ulam", &board)?;
    Ok(())
}
