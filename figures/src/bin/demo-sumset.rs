use figures::{ink, save, Board};
use mrlyrs::core::error::Result;
use mrlyrs::num::sumset::Sumset;

const LEVEL: u32 = 16;
const ROWS: usize = 21;
const CELLS: usize = 860;

fn main() -> Result<()> {
    let sumset = Sumset::new(LEVEL)?;
    assert_eq!(sumset.count(3u64.pow(10)), Some(45968));
    assert_eq!(sumset.count(14348906), Some(10953840));

    let mut board = Board::square();
    let area = board.frame(0.08);
    let band = area.h / ROWS as f64;
    let gap = (band * 0.22).round().max(2.0);

    for row in 0..ROWS {
        let x = 3f64.powf(6.0 + row as f64 / 2.0).round() as u64;
        let cells = CELLS.min(x as usize + 1);
        let fills = sumset.fills(0, x + 1, cells)?;
        assert_eq!(fills.len(), cells);
        let y = (area.y + row as f64 * band).round();
        let h = (band - gap).round();
        let step = area.w / cells as f64;
        for (i, &share) in fills.iter().enumerate() {
            if share == 0.0 {
                continue;
            }
            let left = (area.x + i as f64 * step).round();
            let right = (area.x + (i + 1) as f64 * step).round();
            board.rect(
                left,
                y,
                (right - left).max(1.0),
                h,
                ink::mix(ink::ground(), ink::blue(), share),
            );
        }
    }

    save("demo-sumset", &board)?;
    Ok(())
}
