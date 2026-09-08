use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board};

const WORDMARK: &str = "MRLYPROD";

fn main() -> Result<()> {
    let mut board = Board::square();
    let area = board.frame(0.08);
    let grid = mrlyfont::raster(WORDMARK);
    let rows = grid.len();
    let cols = grid[0].len();
    assert_eq!((rows, cols), (5, 47));
    let scale = (area.w / cols as f64).floor().max(1.0);
    let ox = ((board.width as f64 - cols as f64 * scale) / 2.0).round();
    let oy = ((board.height as f64 - rows as f64 * scale) / 2.0).round();
    let mut lit = 0;
    for (row, line) in grid.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            if cell != 1 {
                continue;
            }
            lit += 1;
            let x = ox + col as f64 * scale;
            let y = oy + row as f64 * scale;
            board.rect(x, y, scale, scale, ink::fg());
        }
    }
    assert_eq!(lit, 103);
    save("demo-font", &board)?;
    Ok(())
}
