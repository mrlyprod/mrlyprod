use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board, Grid, Ramp};
use mrlynum::prime::Sieve;

const LIMIT: usize = 400;
const SIDE: usize = 20;

fn main() -> Result<()> {
    let mut board = Board::square();
    let area = board.frame(0.08);
    let mut sieve = Sieve::new(LIMIT);
    sieve.finish();
    assert_eq!(sieve.count(), 78);
    assert_eq!(sieve.rank(), 8);
    let types = sieve.types();
    let struck = (2..=LIMIT).filter(|&n| types[n] > 1).count();
    assert_eq!(struck, 321);
    let top = *types.iter().max().expect("the sieve is not empty");
    assert_eq!(top, 9);
    let ramp = Ramp::tone(ink::dim(), ink::orange());
    let grid = Grid::new(area, SIDE, SIDE, 0.10);
    for row in 0..SIDE {
        for col in 0..SIDE {
            let number = row * SIDE + col + 1;
            let tone = match types[number] {
                0 => continue,
                1 => ink::yellow(),
                mark => ramp.at((mark - 2) as f64 / (top - 2) as f64),
            };
            grid.fill(&mut board, col, row, tone);
        }
    }
    save("demo-primes", &board)?;
    Ok(())
}
