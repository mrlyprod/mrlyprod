use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board, Grid};
use mrlymath::shape::{self, Frac, Region};
use mrlymath::two::designs;

fn main() -> Result<()> {
    let mut board = Board::square();
    let area = board.frame(0.08);
    let cells = designs::create(7, 3, 4, 0, 2)?;
    let side = cells.width();
    assert_eq!(side, 81);
    let types = cells.types();
    assert_eq!(types.sum(), 4096);
    let ball = shape::named("ball", 2, Frac::new(1, 2))?;
    let tally = shape::census(&ball, types);
    assert_eq!(tally.filled[Region::In as usize], 2908);
    assert_eq!(tally.filled[Region::Cut as usize], 204);
    let map = shape::regions(&ball, &[side, side]);
    let grid = Grid::new(area, side, side, 0.10);
    for row in 0..side {
        for col in 0..side {
            if types.get(&[row, col]) == 0 {
                continue;
            }
            let tone = match map.get(&[row, col]) {
                2 => ink::GOLD,
                1 => ink::ORANGE,
                _ => continue,
            };
            grid.fill(&mut board, col, row, tone);
        }
    }
    save("research-crop", &board)?;
    Ok(())
}
