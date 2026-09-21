use mrlyfig::{ink, save, Board};
use mrlyrs::core::errors::Result;
use mrlyrs::num::memory::{cells, counts, Rule};

const WIDTH: usize = 3;
const CODE: u64 = 23;
const LEVELS: usize = 9;

fn main() -> Result<()> {
    let rule = Rule::new(1, WIDTH, CODE)?;
    let tally = counts(&rule, LEVELS);
    assert_eq!(tally[..8], [2, 4, 4, 6, 9, 13, 19, 28]);

    let mut board = Board::square();
    let area = board.frame(0.08);
    let band = area.h / LEVELS as f64;
    let gap = (band * 0.16).max(2.0).round();

    for depth in 1..=LEVELS {
        let word = cells(&rule, depth);
        assert_eq!(word.len() as u64, tally[depth - 1]);
        let span = area.w / (1u64 << depth) as f64;
        let y = (area.y + (depth - 1) as f64 * band).round();
        let h = (band - gap).round().max(1.0);
        let paint = if depth == LEVELS {
            ink::orange()
        } else {
            ink::yellow()
        };
        for seat in word {
            let x = (area.x + seat as f64 * span).round();
            let w = (area.x + (seat + 1) as f64 * span).round() - x;
            board.rect(x, y, w.max(1.0), h, paint);
        }
    }

    save("demo-memory", &board)?;
    Ok(())
}
