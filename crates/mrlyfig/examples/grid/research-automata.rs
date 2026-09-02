use mrlycore::errors::Result;
use mrlycore::Color;
use mrlyfig::{ink, save, Board};
use mrlymath::life::elementary;

const CENSUS: usize = 16;
const STEPS: usize = 31;
const WINDOW: usize = 63;
const CROP: usize = 48;
const FAINT: f64 = 0.5;

fn tone(rule: u8) -> Color {
    if elementary::rule_degree(rule) == 1 {
        ink::GOLD
    } else if elementary::surjective(rule) {
        ink::BLUE
    } else {
        ink::fade(ink::DIM, FAINT)
    }
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let area = board.frame(0.08);
    let rows = STEPS + 1;
    let pitch = area.w / CENSUS as f64;
    let rise = (pitch - rows as f64) / 2.0;
    let left = (WINDOW - CROP) / 2;

    let mut affine = 0usize;
    let mut surjective = 0usize;
    let mut panels = 0usize;
    for rule in 0..=255u8 {
        let color = tone(rule);
        if elementary::rule_degree(rule) == 1 {
            affine += 1;
        }
        if elementary::surjective(rule) {
            surjective += 1;
        }
        let diagram = elementary::single_seed(rule, STEPS);
        assert_eq!(diagram.shape, vec![rows, WINDOW]);
        let ox = (area.x + (rule as usize % CENSUS) as f64 * pitch).round();
        let oy = (area.y + (rule as usize / CENSUS) as f64 * pitch + rise).round();
        for t in 0..rows {
            for c in 0..CROP {
                if diagram.at(t * WINDOW + left + c) != 0 {
                    board.rect(ox + c as f64, oy + t as f64, 1.0, 1.0, color);
                }
            }
        }
        panels += 1;
    }
    assert_eq!(panels, 256);
    assert_eq!(affine, 14);
    assert_eq!(surjective, 30);
    assert_eq!(surjective - affine, 16);
    save("research-automata", &board)?;
    Ok(())
}
