// merge() lists the frames of letters sliding in from both ends to meet in the middle.
// Printed: every frame for "ABC", one cell of padding.
// cargo run -p mrly-font --example merge
use mrly_font::{merge, raster};

fn main() {
    let (text, pad) = ("ABC", 1);
    let frames = merge(text, pad);
    let flat = raster(text);
    let (rows, cols) = (flat.len() + 2 * pad, flat[0].len() + 2 * pad);
    println!("{} frames", frames.len());
    println!();
    for row in 0..rows {
        let line: Vec<String> = frames
            .iter()
            .map(|f| {
                (0..cols)
                    .map(|col| {
                        if f.contains(&(row * cols + col)) {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect()
            })
            .collect();
        println!("{}", line.join("  "));
    }
}
