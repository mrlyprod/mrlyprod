use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board};
use mrlymath::bang;

const CENSUS: usize = 16;
const STAMP_COLS: usize = 4;
const STAMP_ROWS: usize = 2;

fn main() -> Result<()> {
    let mut board = Board::square();
    let area = board.frame(0.08);
    let universe = bang::bang(3);
    assert_eq!(universe.total, 256);
    assert_eq!(universe.distinct(), 22);
    assert_eq!(bang::counting::distinct_designs(3)?, 22);
    assert_eq!(bang::corners(3).len(), 8);

    let pitch = area.w / CENSUS as f64;
    let bit = pitch * 0.84 / STAMP_COLS as f64;
    let band = bit * (STAMP_ROWS as f64 + 1.0);
    let top = area.y + (area.h - band * CENSUS as f64) / 2.0;
    let pad = bit * 0.10;
    let mut gold = 0usize;
    for code in 0..256u128 {
        let design = universe.design(code);
        let color = if design.canonical {
            gold += 1;
            ink::GOLD
        } else {
            ink::BLUE
        };
        let ox = area.x + (code as usize % CENSUS) as f64 * pitch + (pitch - bit * 4.0) / 2.0;
        let oy = top + (code as usize / CENSUS) as f64 * band + bit / 2.0;
        for slot in 0..8usize {
            if (code >> slot) & 1 == 0 {
                continue;
            }
            let x = ox + (slot % STAMP_COLS) as f64 * bit;
            let y = oy + (slot / STAMP_COLS) as f64 * bit;
            board.rect(x + pad, y + pad, bit - 2.0 * pad, bit - 2.0 * pad, color);
        }
    }
    assert_eq!(gold, 22);
    save("research-bijection", &board)?;
    Ok(())
}
