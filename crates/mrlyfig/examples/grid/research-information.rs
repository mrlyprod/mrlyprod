use mrlycore::errors::Result;
use mrlycore::Color;
use mrlyfig::board::Frame;
use mrlyfig::{ink, save, Board, Grid};
use mrlymath::two::designs;

const CODE: u128 = 495;
const SPLIT: usize = 9;
const SIDE: usize = SPLIT * SPLIT;

fn render(level: usize) -> Result<Vec<Vec<bool>>> {
    let cells = designs::create(CODE, 3, level, 0, 3)?;
    let types = cells.types();
    let side = cells.width();
    Ok((0..side)
        .map(|row| (0..side).map(|col| types.get(&[row, col]) != 0).collect())
        .collect())
}

fn rearrange(cells: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut out = vec![vec![false; SIDE]; SIDE];
    for block_row in 0..SPLIT {
        for block_col in 0..SPLIT {
            for row in 0..SPLIT {
                for col in 0..SPLIT {
                    out[block_row * SPLIT + block_col][row * SPLIT + col] =
                        cells[block_row * SPLIT + row][block_col * SPLIT + col];
                }
            }
        }
    }
    out
}

fn ones(mask: &[Vec<bool>]) -> usize {
    mask.iter().flatten().filter(|on| **on).count()
}

fn panel(board: &mut Board, frame: Frame, mask: &[Vec<bool>], color: Color) {
    Grid::new(frame, SIDE, SIDE, 0.0).carpet(board, mask, color);
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let area = board.frame(0.07);
    let gutter = area.w * 0.06;
    let cell = ((area.w - gutter) / 2.0 / SIDE as f64).floor();
    let side = cell * SIDE as f64;
    let left = area.x + (area.w - 2.0 * side - gutter) / 2.0;
    let top = area.y + (area.h - side) / 2.0;

    let whole = render(4)?;
    let factor = render(2)?;
    assert_eq!(whole.len(), SIDE);
    assert_eq!(factor.len(), SPLIT);
    assert_eq!(ones(&whole), 4096);
    assert_eq!(ones(&factor), 64);

    let rearranged = rearrange(&whole);
    assert_eq!(rearranged.len(), SIDE);
    assert_eq!(rearranged[0].len(), SIDE);
    assert_eq!(ones(&rearranged), 4096);

    let vector: Vec<bool> = factor.iter().flatten().copied().collect();
    assert_eq!(vector.len(), SIDE);
    for (row, line) in rearranged.iter().enumerate() {
        for (col, on) in line.iter().enumerate() {
            assert_eq!(*on, vector[row] && vector[col]);
        }
    }

    panel(
        &mut board,
        Frame::new(left, top, side, side),
        &whole,
        ink::blue(),
    );
    panel(
        &mut board,
        Frame::new(left + side + gutter, top, side, side),
        &rearranged,
        ink::orange(),
    );
    save("research-information", &board)?;
    Ok(())
}
