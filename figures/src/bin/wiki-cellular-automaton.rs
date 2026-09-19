use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board};
use mrlymath::life::elementary;

const RULE: u8 = 90;
const STEPS: usize = 63;

fn main() -> Result<()> {
    let mut board = Board::square();
    let area = board.frame(0.08);
    let rows = STEPS + 1;
    let window = 2 * STEPS + 1;
    let diagram = elementary::single_seed(RULE, STEPS);
    assert_eq!(diagram.shape, vec![rows, window]);

    let mut live = 0usize;
    for t in 0..rows {
        for c in 0..window {
            if diagram.at(t * window + c) != 0 {
                live += 1;
            }
        }
    }
    assert_eq!(live, 729);

    let scale = (area.w / window as f64).min(area.h / rows as f64);
    let ox = (board.width as f64 - window as f64 * scale) / 2.0;
    let oy = (board.height as f64 - rows as f64 * scale) / 2.0;
    let gap = scale * 0.12;
    for t in 0..rows {
        for c in 0..window {
            if diagram.at(t * window + c) != 0 {
                board.rect(
                    ox + c as f64 * scale + gap,
                    oy + t as f64 * scale + gap,
                    scale - 2.0 * gap,
                    scale - 2.0 * gap,
                    ink::yellow(),
                );
            }
        }
    }
    save("wiki-cellular-automaton", &board)?;
    Ok(())
}
