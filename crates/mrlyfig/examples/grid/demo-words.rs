use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board};
use mrlymath::bang::{magic, MagicLayer};
use mrlymath::name::Bang;
use mrlymath::two;

const SIDE: usize = 105;
const UNIT: f64 = 8.0;

fn main() -> Result<()> {
    let mut board = Board::square();
    let letters = [
        MagicLayer::new(Bang::new(7, 2, 2), 3),
        MagicLayer::new(Bang::new(14, 2, 2), 7),
        MagicLayer::new(Bang::new(9, 2, 2), 5),
    ];
    let first = two::create(7, 3, 1, 0, 2)?;
    let pair = magic(&letters[..2])?;
    let whole = magic(&letters)?;
    assert_eq!(first.width(), 3);
    assert_eq!(pair.shape, vec![21, 21]);
    assert_eq!(whole.shape, vec![SIDE, SIDE]);
    let single = first.types();
    let mut depth = vec![0usize; SIDE * SIDE];
    let mut census = [0usize; 4];
    for row in 0..SIDE {
        for col in 0..SIDE {
            let mut deep = 0usize;
            if single.get(&[row / 35, col / 35]) != 0 {
                deep += 1;
                if pair.get(&[row / 5, col / 5]) != 0 {
                    deep += 1;
                    if whole.get(&[row, col]) != 0 {
                        deep += 1;
                    }
                }
            }
            depth[row * SIDE + col] = deep;
            census[deep] += 1;
        }
    }
    assert_eq!(census, [1225, 3200, 3168, 3432]);
    assert_eq!(census.iter().sum::<usize>(), SIDE * SIDE);
    let tones = [ink::line(), ink::dim(), ink::blue()];
    let span = SIDE as f64 * UNIT;
    let edge = (board.width as f64 - span) / 2.0;
    for row in 0..SIDE {
        for col in 0..SIDE {
            let deep = depth[row * SIDE + col];
            if deep == 0 {
                continue;
            }
            board.rect(
                edge + col as f64 * UNIT,
                edge + row as f64 * UNIT,
                UNIT,
                UNIT,
                tones[deep - 1],
            );
        }
    }
    save("demo-words", &board)?;
    Ok(())
}
