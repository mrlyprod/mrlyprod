// Every glyph has a pen: the order in which a hand would draw it, as strokes of cells.
// Each lit cell shows its step, counting from 0 and using 0-9 then a-z; a new stroke lifts the pen.
// cargo run -p mrly-font --example pen
use mrly_font::strokes;

fn main() {
    for c in ['A', 'S'] {
        let strokes = strokes(c);
        let cells: usize = strokes.iter().map(Vec::len).sum();
        println!("{c}: {} strokes, {cells} cells", strokes.len());
        let mut grid = vec![vec!['.'; 5]; 5];
        let mut step = 0;
        for stroke in &strokes {
            for &(r, col) in stroke {
                grid[r][col] = std::char::from_digit(step, 36).unwrap();
                step += 1;
            }
        }
        for row in grid {
            println!("  {}", row.into_iter().collect::<String>());
        }
        println!();
    }
}
