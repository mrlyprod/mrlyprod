use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board, Grid};
use mrlymath::two;
use mrlymath::two::census;

const CODE: u128 = 495;
const NUMBER: usize = 3;
const BASE: usize = 3;
const LEVEL: usize = 2;
const COPIES: usize = 5;
const TILE: usize = 9;
const SIDE: usize = 45;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let tile = two::create(CODE, NUMBER, LEVEL, 0, BASE)?;
    assert_eq!((tile.width(), census::fills(&tile)), (TILE, 64));
    assert_eq!(census::perimeter(&tile), 80);
    let sheet = tile.tile(COPIES, COPIES);
    assert_eq!((sheet.width(), census::fills(&sheet)), (SIDE, 1600));
    assert_eq!(census::perimeter(&sheet), 1280);
    let buried = 25 * 80 - census::perimeter(&sheet);
    assert_eq!(buried, 720);
    let types = sheet.types();
    let on = |row: usize, col: usize| types.get(&[row, col]) != 0;
    let grid = Grid::new(frame, SIDE, SIDE, 0.0);
    for row in 0..SIDE {
        for col in 0..SIDE {
            if on(row, col) {
                grid.fill(&mut board, col, row, ink::blue());
            }
        }
    }
    let thick = frame.cell(SIDE) * 0.26;
    let mut seams = 0usize;
    for row in 0..SIDE {
        for col in 0..SIDE {
            if !on(row, col) {
                continue;
            }
            let (x, y, w, h) = grid.cell(col, row);
            if col % TILE == TILE - 1 && col + 1 < SIDE && on(row, col + 1) {
                board.segment((x + w, y), (x + w, y + h), thick, ink::orange());
                seams += 1;
            }
            if row % TILE == TILE - 1 && row + 1 < SIDE && on(row + 1, col) {
                board.segment((x, y + h), (x + w, y + h), thick, ink::orange());
                seams += 1;
            }
        }
    }
    assert_eq!(seams * 2, buried as usize);
    assert_eq!(seams, 360);
    save("demo-tile", &board)?;
    Ok(())
}
