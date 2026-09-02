use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board, Grid};
use mrlymath::bang::{word, MagicLayer};
use mrlymath::name::Bang;

const CODES: usize = 15;
const LENGTH: usize = 32;
const SLACK: f64 = 0.25;

fn alternating(a: u128, b: u128) -> Vec<MagicLayer> {
    (0..LENGTH)
        .map(|place| {
            let code = if place % 2 == 0 { a } else { b };
            MagicLayer::new(Bang::new(code, 2, 2), 2)
        })
        .collect()
}

fn verdict(a: u128, b: u128) -> Result<(bool, bool)> {
    let letters = alternating(a, b);
    let rates = word::rates(&letters)?;
    let (component, fill) = rates[rates.len() - 1];
    let constant = word::constant_functional(&letters)?;
    Ok((
        (component - fill).abs() < SLACK,
        (component - constant).abs() < SLACK,
    ))
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let grid = Grid::new(board.frame(0.08), CODES, CODES, 0.12);
    let mut alphabets = 0usize;
    let mut ceiling = 0usize;
    let mut exact = 0usize;
    for row in 0..CODES {
        for col in (row + 1)..CODES {
            let (meets, constant) = verdict(row as u128 + 1, col as u128 + 1)?;
            alphabets += 1;
            let (x, y, w, h) = grid.cell(col, row);
            if meets {
                ceiling += 1;
                board.rect(x, y, w, h, ink::GREEN);
            } else {
                board.rect(x, y, w, h, ink::ORANGE);
            }
            if constant {
                exact += 1;
                let pip = w * 0.34;
                board.rect(
                    x + (w - pip) / 2.0,
                    y + (h - pip) / 2.0,
                    pip,
                    pip,
                    ink::GOLD,
                );
            }
        }
    }
    assert_eq!((alphabets, ceiling, exact), (105, 89, 27));
    save("paper-component-exponent-of-kronecker-words", &board)?;
    Ok(())
}
