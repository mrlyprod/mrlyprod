use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board, Grid};
use mrlymath::two::designs;
use mrlynum::classics::gcd;

fn main() -> Result<()> {
    let side = 32;
    let gasket = designs::from_corners(&[vec![0, 0], vec![1, 0], vec![0, 1]], 2, 5, 0, 2)?;
    let types = gasket.types();
    let mut board = Board::square();
    let grid = Grid::new(board.frame(0.08), side, side, 0.10);
    let mut filled = 0usize;
    let mut visible = 0usize;
    for y in 0..side {
        for x in 0..side {
            if types.get(&[y, x]) == 0 {
                continue;
            }
            filled += 1;
            let coprime = gcd(x as u128, y as u128) == 1;
            if coprime {
                visible += 1;
            }
            let tone = if coprime { ink::GOLD } else { ink::DIM };
            grid.fill(&mut board, x, side - 1 - y, tone);
        }
    }
    assert_eq!((filled, visible), (243, 122));
    save("paper-coprime-density-above-dimension-one", &board)?;
    Ok(())
}
