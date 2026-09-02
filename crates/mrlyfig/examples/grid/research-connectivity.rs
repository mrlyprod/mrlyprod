use mrlycore::errors::Result;
use mrlycore::Color;
use mrlycore::Rng;
use mrlyfig::board::{Board, Frame};
use mrlyfig::{ink, save, Grid};
use mrlymath::two::designs;

const SIDE: usize = 64;
const CELLS: usize = 729;

fn scatter(seed: u64, count: usize) -> Vec<bool> {
    let mut rng = Rng::new(seed);
    let mut slots: Vec<usize> = (0..SIDE * SIDE).collect();
    let mut mask = vec![false; SIDE * SIDE];
    for pick in 0..count {
        let swap = pick + rng.below(slots.len() - pick);
        slots.swap(pick, swap);
        mask[slots[pick]] = true;
    }
    assert_eq!(mask.iter().filter(|on| **on).count(), count);
    mask
}

fn plate(board: &mut Board, frame: Frame, mask: &[bool], color: Color, rule: f64) {
    let edge = frame.inset(-rule * 2.0);
    board.rect(edge.x, edge.y, edge.w, rule, ink::LINE);
    board.rect(edge.x, edge.y + edge.h - rule, edge.w, rule, ink::LINE);
    board.rect(edge.x, edge.y, rule, edge.h, ink::LINE);
    board.rect(edge.x + edge.w - rule, edge.y, rule, edge.h, ink::LINE);
    let grid = Grid::new(frame, SIDE, SIDE, 0.0);
    for row in 0..SIDE {
        for col in 0..SIDE {
            if mask[row * SIDE + col] {
                grid.fill(board, col, row, color);
            }
        }
    }
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let rule = board.width as f64 / 320.0;
    let lay = board.frame(0.08).inset(rule * 2.0);
    let gap = rule * 4.0;
    let side = (lay.w - gap) / 2.0;

    let cells = designs::create(7, 2, 6, 0, 2)?;
    assert_eq!(cells.width(), SIDE);
    let types = cells.types();
    let gasket: Vec<bool> = (0..SIDE * SIDE).map(|flat| types.at(flat) != 0).collect();
    assert_eq!(gasket.iter().filter(|on| **on).count(), CELLS);

    plate(
        &mut board,
        Frame::new(lay.x, lay.y, side, side),
        &gasket,
        ink::GREEN,
        rule,
    );
    plate(
        &mut board,
        Frame::new(lay.x + side + gap, lay.y + side + gap, side, side),
        &scatter(20260902, CELLS),
        ink::PINK,
        rule,
    );
    save("research-connectivity", &board)?;
    Ok(())
}
