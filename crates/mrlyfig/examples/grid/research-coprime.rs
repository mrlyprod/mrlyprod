use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board, Grid};
use mrlymath::two::designs;
use mrlynum::classics::gcd;

fn main() -> Result<()> {
    let mut board = Board::square();
    let area = board.frame(0.08);
    let cells = designs::create(495, 3, 4, 0, 3)?;
    let side = cells.width();
    assert_eq!(side, 81);
    let types = cells.types();
    assert_eq!(types.sum(), 4096);
    let grid = Grid::new(area, side, side, 0.10);
    let mut lit = 0usize;
    for row in 0..side {
        for col in 0..side {
            if types.get(&[row, col]) == 0 {
                continue;
            }
            let x = (col + 1) as u128;
            let y = (side - row) as u128;
            let coprime = gcd(x, y) == 1;
            if coprime {
                lit += 1;
            }
            grid.fill(
                &mut board,
                col,
                row,
                if coprime { ink::GOLD } else { ink::DIM },
            );
        }
    }
    assert!(lit > 0 && lit < 4096);
    save("research-coprime", &board)?;
    Ok(())
}
