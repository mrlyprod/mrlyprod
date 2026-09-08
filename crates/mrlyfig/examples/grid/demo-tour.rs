use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board, Grid};
use mrlymath::two;
use mrlymath::two::census;

const CODE: u128 = 7;
const NUMBER: usize = 3;
const BASE: usize = 2;
const LEVEL: usize = 4;
const SIDE: usize = 81;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let carpet = two::create(CODE, NUMBER, LEVEL, 0, BASE)?;
    assert_eq!((carpet.width(), census::fills(&carpet)), (SIDE, 4096));
    assert_eq!(census::perimeter(&carpet), 3536);
    let types = carpet.types();
    let on = |row: i64, col: i64| {
        (0..SIDE as i64).contains(&row)
            && (0..SIDE as i64).contains(&col)
            && types.get(&[row as usize, col as usize]) != 0
    };
    let grid = Grid::new(frame, SIDE, SIDE, 0.0);
    let body = ink::mix(ink::line(), ink::dim(), 0.35);
    for row in 0..SIDE {
        for col in 0..SIDE {
            if on(row as i64, col as i64) {
                grid.fill(&mut board, col, row, body);
            }
        }
    }
    let thick = frame.cell(SIDE) * 0.22;
    let mut edges = 0usize;
    for row in 0..SIDE {
        for col in 0..SIDE {
            if !on(row as i64, col as i64) {
                continue;
            }
            let (x, y, w, h) = grid.cell(col, row);
            let (r, c) = (row as i64, col as i64);
            let sides = [
                (!on(r - 1, c), (x, y), (x + w, y)),
                (!on(r + 1, c), (x, y + h), (x + w, y + h)),
                (!on(r, c - 1), (x, y), (x, y + h)),
                (!on(r, c + 1), (x + w, y), (x + w, y + h)),
            ];
            for (open, a, b) in sides {
                if open {
                    board.segment(a, b, thick, ink::yellow());
                    edges += 1;
                }
            }
        }
    }
    assert_eq!(edges as u128, census::perimeter(&carpet));
    save("demo-tour", &board)?;
    Ok(())
}
