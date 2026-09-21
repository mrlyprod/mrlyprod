use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board};
use mrlynum::morse::{digits, lift, Lift};

const STRIP: usize = 64;
const SIDE: usize = 32;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let word = digits(STRIP);
    assert_eq!(word.iter().filter(|&&bit| bit == 1).count(), STRIP / 2);
    let signs = lift(Lift::Parity, SIDE);
    assert_eq!(lift(Lift::Xor, SIDE), signs);
    assert_eq!(
        signs.iter().filter(|&&bit| bit == 1).count(),
        SIDE * SIDE / 2
    );
    let tone = |bit: u8| if bit == 0 { ink::orange() } else { ink::blue() };
    let cell = (frame.h / (2.0 * SIDE as f64 + 5.0)).floor();
    let block = 2.0 * cell;
    let band = 3.0 * cell;
    let gap = 2.0 * cell;
    let run = block * SIDE as f64;
    let left = ((board.width as f64 - run) / 2.0).round();
    let top = ((board.height as f64 - run - band - gap) / 2.0).round();
    for (place, &bit) in word.iter().enumerate() {
        board.rect(left + place as f64 * cell, top, cell, band, tone(bit));
    }
    let head = top + band + gap;
    for row in 0..SIDE {
        for col in 0..SIDE {
            board.rect(
                left + col as f64 * block,
                head + row as f64 * block,
                block,
                block,
                tone(signs[row * SIDE + col]),
            );
        }
    }
    save("wiki-thue-morse-sequence", &board)?;
    Ok(())
}
