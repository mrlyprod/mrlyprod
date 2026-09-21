use figures::{ink, save, Board, Grid};
use mrlyrs::core::error::Result;
use mrlyrs::num::factor::{mobius, mobius_sieve};

const TOP: usize = 100;
const SIDE: usize = 10;

fn main() -> Result<()> {
    let mut board = Board::square();
    let area = board.frame(0.08);
    let mu = mobius_sieve(TOP);
    let grid = Grid::new(area, SIDE, SIDE, 0.10);
    let (mut plus, mut minus, mut flat) = (0usize, 0usize, 0usize);
    for row in 0..SIDE {
        for col in 0..SIDE {
            let number = row * SIDE + col + 1;
            assert_eq!(mu[number], mobius(number));
            let tone = match mu[number] {
                1 => {
                    plus += 1;
                    ink::yellow()
                }
                -1 => {
                    minus += 1;
                    ink::blue()
                }
                _ => {
                    flat += 1;
                    ink::dim()
                }
            };
            grid.fill(&mut board, col, row, tone);
        }
    }
    assert_eq!((plus, minus, flat), (31, 30, 39));
    save("wiki-mobius-function", &board)?;
    Ok(())
}
