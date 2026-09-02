use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board};

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let dims: Vec<usize> = (2..=14).collect();
    let free: Vec<usize> = dims.iter().map(|d| 2 * d + 1).collect();
    let proved: Vec<usize> = dims.iter().map(|d| d.div_ceil(2)).collect();
    assert_eq!(free.first(), Some(&5));
    assert_eq!(free.last(), Some(&29));
    assert_eq!(proved, vec![1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7]);

    let peak = *free.last().unwrap() as f64;
    let slot = frame.w / dims.len() as f64;
    let foot = frame.y + frame.h;
    let unit = frame.h / peak;
    for (index, order) in free.iter().enumerate() {
        let wide = slot * 0.62;
        let tall = *order as f64 * unit;
        board.rect(
            frame.x + index as f64 * slot + (slot - wide) / 2.0,
            foot - tall,
            wide,
            tall,
            ink::fade(ink::DIM, 0.45),
        );
    }
    for (index, order) in proved.iter().enumerate() {
        let wide = slot * 0.26;
        let tall = *order as f64 * unit;
        board.rect(
            frame.x + index as f64 * slot + (slot - wide) / 2.0,
            foot - tall,
            wide,
            tall,
            ink::BLUE,
        );
    }
    board.rect(frame.x, foot, frame.w, 2.0, ink::LINE);
    save("paper-slice-recurrence-order", &board)?;
    Ok(())
}
