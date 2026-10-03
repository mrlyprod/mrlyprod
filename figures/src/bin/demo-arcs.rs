use figures::{ink, save, Board, Grid};
use mrlyrs::core::error::Result;
use mrlyrs::math::arcs;
use mrlyrs::math::bang::Code;
use std::f64::consts::{FRAC_PI_2, PI};

const LEVEL: u32 = 5;
const SIDE: usize = 32;

type Sweep = ((f64, f64), (f64, f64));

fn sweeps(filled: bool, left: f64, top: f64, cell: f64) -> [Sweep; 2] {
    if filled {
        [
            ((left, top + cell), (-FRAC_PI_2, 0.0)),
            ((left + cell, top), (FRAC_PI_2, PI)),
        ]
    } else {
        [
            ((left + cell, top + cell), (PI, PI + FRAC_PI_2)),
            ((left, top), (0.0, FRAC_PI_2)),
        ]
    }
}

fn main() -> Result<()> {
    let drawn = arcs::draw(Code::from(7u128), 2, LEVEL as usize)?;
    assert_eq!(drawn.side, SIDE);
    assert_eq!(drawn.loops, 3u64.pow(LEVEL - 1) - 2u64.pow(LEVEL) + 1);
    assert_eq!(drawn.loops, 50);
    assert_eq!(drawn.strands, 2 * SIDE as u64);
    assert_eq!(drawn.cells.iter().filter(|&&b| b & 1 == 1).count(), 243);

    let mut board = Board::square();
    let frame = board.frame(0.08);
    let cell = frame.w / SIDE as f64;
    let grid = Grid::new(frame, SIDE, SIDE, 0.0);
    for y in 0..SIDE {
        for x in 0..SIDE {
            if drawn.cells[y * SIDE + x] & 1 == 1 {
                grid.fill(&mut board, x, SIDE - 1 - y, ink::line());
            }
        }
    }
    for y in 0..SIDE {
        for x in 0..SIDE {
            let byte = drawn.cells[y * SIDE + x];
            let left = frame.x + x as f64 * cell;
            let top = frame.y + (SIDE - 1 - y) as f64 * cell;
            for ((centre, angles), bit) in sweeps(byte & 1 == 1, left, top, cell)
                .into_iter()
                .zip([2, 4])
            {
                let color = if byte & bit == 0 {
                    ink::blue()
                } else {
                    ink::orange()
                };
                board.arc(centre, cell / 2.0, angles, cell * 0.18, color);
            }
        }
    }
    save("demo-arcs", &board)?;
    Ok(())
}
