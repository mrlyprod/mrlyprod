use figures::{ink, save, Board, Grid, Ramp};
use mrlyrs::core::error::Result;
use mrlyrs::num::factor::gcd;
use mrlyrs::num::lattice::coprime_pairs;

const SIDE: usize = 24;

fn main() -> Result<()> {
    let mut board = Board::square();
    let area = board.frame(0.08);
    let grid = Grid::new(area, SIDE, SIDE, 0.10);
    let ramp = Ramp::tone(ink::dim(), ink::blue());
    let mut lit = 0u64;
    for row in 0..SIDE {
        for col in 0..SIDE {
            let a = col + 1;
            let b = SIDE - row;
            let share = gcd(a, b);
            let tone = if share == 1 {
                lit += 1;
                ink::yellow()
            } else {
                ramp.at((share - 2) as f64 / (SIDE - 2) as f64)
            };
            grid.fill(&mut board, col, row, tone);
        }
    }
    assert_eq!(lit, coprime_pairs(SIDE));
    assert_eq!(lit, 359);
    save("wiki-greatest-common-divisor", &board)?;
    Ok(())
}
