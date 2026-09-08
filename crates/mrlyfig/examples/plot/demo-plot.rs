use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board};
use mrlylab::ledger::{terms, Axis, Key, Measure, BUDGET};
use mrlynum::blend::delta;

const ROWS: usize = 8;

fn main() -> Result<()> {
    let key = Key::new(23, 3, 2, Measure::Fills, Axis::Side);
    let (first, capped) = terms(&key, ROWS, BUDGET)?;
    assert_eq!(key.name(), "mrly_bang_d3_23.fills.side");
    assert!(!capped);
    assert_eq!(first, vec![20, 81, 208, 425, 756, 1225, 1856, 2673]);
    let mut rows = vec![first];
    while rows.len() < ROWS {
        rows.push(delta(rows.last().unwrap()));
    }
    let cells: usize = rows.iter().map(|row| row.len()).sum();
    assert_eq!(cells, 36);
    assert_eq!(rows[3], vec![24; 5]);
    assert!(rows[4..].iter().all(|row| row.iter().all(|v| *v == 0)));

    let mut board = Board::square();
    let frame = board.frame(0.08);
    let step = frame.w / ROWS as f64;
    let top = rows[0].iter().copied().max().unwrap_or(1) as f64;
    let scale = (1.0 + top).ln();
    for (depth, row) in rows.iter().enumerate() {
        let y = frame.y + (depth as f64 + 0.5) * step;
        for (place, value) in row.iter().enumerate() {
            let x = frame.x + (place as f64 + 0.5 + depth as f64 / 2.0) * step;
            if *value == 0 {
                board.ring(x, y, step * 0.14, step * 0.030, ink::dim());
                continue;
            }
            let weight = (1.0 + value.unsigned_abs() as f64).ln() / scale;
            let color = if depth == 3 {
                ink::yellow()
            } else {
                ink::blue()
            };
            board.disc(x, y, step * (0.07 + 0.33 * weight), color);
        }
    }
    save("demo-plot", &board)?;
    Ok(())
}
