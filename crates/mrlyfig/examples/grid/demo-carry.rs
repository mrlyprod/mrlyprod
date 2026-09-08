use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board, Frame, Grid, Ramp};
use mrlymath::dim::carry;

const BASES: [usize; 2] = [3, 5];
const DIMS: [usize; 8] = [2, 3, 4, 5, 6, 7, 8, 9];

fn main() -> Result<()> {
    let mut board = Board::square();
    let area = board.frame(0.08);
    let panels = Grid::new(area, 4, 4, 0.05);
    assert_eq!(carry::cap(3)?, 15);
    assert_eq!(carry::cap(5)?, 11);
    assert_eq!(carry::even_block(3, 3)?, vec![vec![6, 6], vec![1, 3]]);
    let mut drawn = 0usize;
    for (turn, base) in BASES.iter().enumerate() {
        let ramp = Ramp::tone(
            ink::line(),
            if *base == 3 {
                ink::yellow()
            } else {
                ink::blue()
            },
        );
        for (step, dimension) in DIMS.iter().enumerate() {
            let block = carry::even_block(*base, *dimension)?;
            let width = block.len();
            assert_eq!(width, dimension.div_ceil(2));
            let slot = turn * DIMS.len() + step;
            let (x, y, w, h) = panels.cell(slot % 4, slot / 4);
            let cells = Grid::new(Frame::new(x, y, w, h), width, width, 0.06);
            let peak = block
                .iter()
                .flatten()
                .fold(0.0f64, |top, &value| top.max(value as f64));
            for (row, line) in block.iter().enumerate() {
                for (col, &value) in line.iter().enumerate() {
                    let t = (1.0 + value as f64).ln() / (1.0 + peak).ln();
                    cells.fill(&mut board, col, row, ramp.at(t));
                    drawn += 1;
                }
            }
        }
    }
    assert_eq!(drawn, 168);
    save("demo-carry", &board)?;
    Ok(())
}
