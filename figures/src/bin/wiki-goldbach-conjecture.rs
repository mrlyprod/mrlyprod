use mrlyfig::{ink, plot, save, Board};
use mrlyrs::core::errors::Result;
use mrlyrs::num::formulas;

const TOP: usize = 400;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let record = formulas::goldbach_record(TOP);
    let peak = record.iter().copied().max().unwrap_or(1) as f64;
    let slot = frame.w / record.len() as f64;
    let pad = slot * 0.16;
    let mut sixes = 0usize;
    for (i, &pairs) in record.iter().enumerate() {
        let even = 2 * (i + 2);
        let h = frame.h * pairs as f64 / peak;
        let hue = if even.is_multiple_of(6) {
            sixes += 1;
            ink::yellow()
        } else {
            ink::blue()
        };
        board.rect(
            frame.x + i as f64 * slot + pad,
            frame.y + frame.h - h,
            slot - 2.0 * pad,
            h,
            hue,
        );
    }

    plot::baseline(&mut board, frame, ink::line());
    assert_eq!(record.len(), 199);
    assert_eq!(record.iter().copied().min(), Some(1));
    assert_eq!(peak as usize, 27);
    assert_eq!(sixes, 66);
    save("wiki-goldbach-conjecture", &board)?;
    Ok(())
}
