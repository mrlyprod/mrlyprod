use mrlycore::errors::Result;
use mrlycore::Color;
use mrlyfig::{ink, save, Board, Frame, Grid};
use mrlymath::two::designs;

const BOXES: usize = 3;
const GUTTER: f64 = 52.0;
const SAMPLES: usize = 1000;

fn outline(board: &mut Board, frame: Frame, thick: f64, color: Color) {
    board.rect(frame.x, frame.y, frame.w, thick, color);
    board.rect(frame.x, frame.y + frame.h - thick, frame.w, thick, color);
    board.rect(frame.x, frame.y, thick, frame.h, color);
    board.rect(frame.x + frame.w - thick, frame.y, thick, frame.h, color);
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let area = board.frame(0.08);
    let tile = (area.w - 2.0 * GUTTER) / 3.0;
    let top = area.y + (area.h - tile) / 2.0;
    let carpet = designs::create(495, 3, 3, 0, 3)?;
    let side = carpet.width();
    assert_eq!(side, 27);

    let mut lit = [0usize; 3];
    for panel in 0..3usize {
        let x = area.x + panel as f64 * (tile + GUTTER);
        let frame = Frame::new(x, top, tile, tile);
        let mut on = [[false; BOXES]; BOXES];
        match panel {
            0 => {
                board.rect(frame.x, frame.y, frame.w, frame.h, ink::blue());
                on = [[true; BOXES]; BOXES];
            }
            1 => {
                Grid::new(frame, side, side, 0.0).paint(&mut board, &carpet, |kind| {
                    (kind != 0).then_some(ink::blue())
                });
                let block = side / BOXES;
                for row in 0..BOXES {
                    for col in 0..BOXES {
                        for r in row * block..(row + 1) * block {
                            for c in col * block..(col + 1) * block {
                                if carpet.types().get(&[r, c]) != 0 {
                                    on[row][col] = true;
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                board.segment(
                    (frame.x, frame.y + frame.h),
                    (frame.x + frame.w, frame.y),
                    tile * 0.03,
                    ink::blue(),
                );
                for i in 0..SAMPLES {
                    let u = (i as f64 + 0.5) / SAMPLES as f64;
                    let col = (u * BOXES as f64) as usize;
                    let row = ((1.0 - u) * BOXES as f64) as usize;
                    on[row.min(BOXES - 1)][col.min(BOXES - 1)] = true;
                }
            }
        }
        let step = tile / BOXES as f64;
        for row in 0..BOXES {
            for col in 0..BOXES {
                let cell = Frame::new(
                    frame.x + col as f64 * step,
                    frame.y + row as f64 * step,
                    step,
                    step,
                );
                outline(&mut board, cell, 1.5, ink::line());
                if on[row][col] {
                    outline(&mut board, cell.inset(step * 0.07), 6.0, ink::yellow());
                    lit[panel] += 1;
                }
            }
        }
    }
    assert_eq!(lit, [9, 8, 3]);
    save("wiki-fractal-dimension", &board)?;
    Ok(())
}
