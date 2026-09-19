use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board};
use mrlylab::ledger::{designs, terms, Axis, Key, Measure, BUDGET};
use mrlynum::blend::delta;

const COUNT: usize = 8;

fn main() -> Result<()> {
    assert_eq!(designs(1, 2)?.len(), 3);
    assert_eq!(designs(2, 2)?.len(), 6);
    assert_eq!(designs(3, 2)?.len(), 22);
    let mut rows: Vec<(usize, Vec<i128>)> = Vec::new();
    for dimension in 1..=3usize {
        for &code in designs(dimension, 2)? {
            let key = Key::new(code, dimension, 2, Measure::Fills, Axis::Side);
            let (values, capped) = terms(&key, COUNT, BUDGET)?;
            assert!(!capped);
            let mut fold = values.clone();
            for _ in 0..dimension {
                fold = delta(&fold);
            }
            assert!(fold.iter().all(|value| *value == fold[0]));
            assert!(delta(&fold).iter().all(|value| *value == 0));
            if values[0] > 0 {
                rows.push((dimension, values));
            }
        }
    }
    assert_eq!(rows.len(), 28);

    let mut board = Board::square();
    let frame = board.frame(0.08);
    let sides: Vec<f64> = (0..COUNT).map(|i| ((2 * i + 3) as f64).ln()).collect();
    let logs: Vec<f64> = rows
        .iter()
        .flat_map(|(_, values)| values.iter().map(|value| (*value as f64).ln()))
        .collect();
    let low = logs.iter().copied().fold(f64::MAX, f64::min);
    let high = logs.iter().copied().fold(f64::MIN, f64::max);
    let at = |index: usize, value: i128| {
        (
            frame.x + frame.w * (sides[index] - sides[0]) / (sides[COUNT - 1] - sides[0]),
            frame.y + frame.h * (1.0 - ((value as f64).ln() - low) / (high - low)),
        )
    };
    for dimension in 1..=3usize {
        let color = match dimension {
            1 => ink::fade(ink::dim(), 0.85),
            2 => ink::yellow(),
            _ => ink::blue(),
        };
        for (_, values) in rows.iter().filter(|(d, _)| *d == dimension) {
            let path: Vec<(f64, f64)> = values
                .iter()
                .enumerate()
                .map(|(index, value)| at(index, *value))
                .collect();
            board.polyline(&path, 2.5, color);
            for (x, y) in &path {
                board.disc(*x, *y, 5.5, color);
            }
        }
    }
    save("demo-sequences", &board)?;
    Ok(())
}
