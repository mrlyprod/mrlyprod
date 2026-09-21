use mrlyfig::{ink, save, Board, Frame, Grid};
use mrlyrs::core::error::Result;
use mrlyrs::math::two::designs;

const CODES: [u128; 7] = [7, 14, 3, 5, 9, 6, 15];
const SIDE: usize = 9;
const GUTTER: f64 = 130.0;
const EDGE: f64 = 3.0;

fn place(index: usize) -> (usize, usize) {
    match index {
        0 => (0, 1),
        n if n < 4 => (1, n - 1),
        n => (2, n - 4),
    }
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let area = board.frame(0.08);
    let tile = (area.w - 2.0 * GUTTER) / 3.0;
    let step = tile + GUTTER;
    let corner = |index: usize| {
        let (row, col) = place(index);
        (area.x + col as f64 * step, area.y + row as f64 * step)
    };
    let foot = |index: usize| {
        let (x, y) = corner(index);
        (x + tile / 2.0, y + tile)
    };
    let head = |index: usize| {
        let (x, y) = corner(index);
        (x + tile / 2.0, y)
    };
    for child in 1..CODES.len() {
        let parent = if child < 4 { 0 } else { child - 3 };
        board.segment(foot(parent), head(child), EDGE, ink::line());
    }
    let mut cells = 0u64;
    for (index, code) in CODES.iter().enumerate() {
        let design = designs::create(*code, 3, 2, 0, 2)?;
        assert_eq!(design.width(), SIDE);
        let tone = if index == 0 {
            ink::yellow()
        } else {
            ink::blue()
        };
        let (x, y) = corner(index);
        Grid::new(Frame::new(x, y, tile, tile), SIDE, SIDE, 0.0).paint(
            &mut board,
            &design,
            |kind| (kind != 0).then_some(tone),
        );
        cells += design.types().sum();
    }
    assert_eq!(cells, 283);
    save("site-wiki", &board)?;
    Ok(())
}
