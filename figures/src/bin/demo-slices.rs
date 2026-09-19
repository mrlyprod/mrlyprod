use mrlycore::errors::Result;
use mrlyfig::board::{Board, Frame};
use mrlyfig::{hex, ink, save};
use mrlymath::six;
use mrlymath::six::Cell6d;

const CODE: u128 = 23;
const WIDE: usize = 4;

fn at(cell: &Cell6d, side: usize, row: usize, col: usize) -> u8 {
    let offset = (cell.width() - hex::row_len(side, row)) / 2;
    cell.cell.types().get(&[row, col + offset])
}

fn panels(frame: Frame, gap: f64) -> Vec<Frame> {
    frame
        .rows(WIDE)
        .iter()
        .flat_map(|row| row.cols(WIDE))
        .map(|cell| cell.inset(gap))
        .collect()
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let stage = panels(frame, frame.w / WIDE as f64 * 0.05);
    let mut triangles = 0usize;
    for k in 1..=WIDE * WIDE {
        let side = 2 * k - 1;
        let slice = six::cut_design(CODE, side, 1, 2)?;
        let pieces = six::components(&slice)?;
        let holes = six::holes(&slice)?;
        if k % 2 == 0 {
            assert_eq!(pieces, 1);
            assert!(holes > 0);
        } else if k > 1 {
            assert!(pieces > 1);
        }
        let color = if pieces > 1 {
            ink::blue()
        } else {
            ink::orange()
        };
        let panel = stage[k - 1];
        let edge = panel.w / (2 * side) as f64;
        let painted = std::cell::Cell::new(0usize);
        let offered = std::cell::Cell::new(0usize);
        hex::hexagon(&mut board, panel, side, edge * 0.14, |row, col, _| {
            offered.set(offered.get() + 1);
            if at(&slice, side, row, col) != six::FILL {
                return None;
            }
            painted.set(painted.get() + 1);
            Some(color)
        });
        assert_eq!(offered.get(), hex::count(side));
        assert_eq!(painted.get(), six::fills(&slice));
        triangles += offered.get();
    }
    assert_eq!(triangles, 32736);
    save("demo-slices", &board)?;
    Ok(())
}
