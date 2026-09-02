use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board};
use mrlymath::life::elementary;

const RULE: u8 = 110;
const STEPS: usize = 256;

fn main() -> Result<()> {
    let mut board = Board::square();
    let area = board.frame(0.08);
    let rows = STEPS + 1;
    let window = 2 * STEPS + 1;
    let diagram = elementary::single_seed(RULE, STEPS);
    assert_eq!(diagram.shape, vec![rows, window]);

    let mut lo = window;
    let mut hi = 0usize;
    let mut live = 0usize;
    for t in 0..rows {
        for c in 0..window {
            if diagram.at(t * window + c) != 0 {
                lo = lo.min(c);
                hi = hi.max(c);
                live += 1;
            }
        }
    }
    assert_eq!(lo, 0);
    assert_eq!(hi, STEPS);
    assert!(live > 0);

    let span = hi - lo + 1;
    let scale = (area.w / span as f64)
        .min(area.h / rows as f64)
        .floor()
        .max(1.0);
    let ox = ((board.width as f64 - span as f64 * scale) / 2.0).round();
    let oy = ((board.height as f64 - rows as f64 * scale) / 2.0).round();
    for t in 0..rows {
        for c in lo..=hi {
            if diagram.at(t * window + c) != 0 {
                let x = ox + (c - lo) as f64 * scale;
                let y = oy + t as f64 * scale;
                board.rect(x, y, scale, scale, ink::FG);
            }
        }
    }
    save("demo-wolfram", &board)?;
    Ok(())
}
