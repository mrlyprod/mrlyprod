// The Sierpinski carpet as text: 3x3 tile, three levels, 27x27 cells.
// text() turns each cell value into a glyph; here 1 is a block and 0 is a dot.
// cargo run -p mrly-two --example carpet_text

use mrly_two::{carpet, text};
use std::collections::HashMap;

fn main() {
    let cell = carpet(3, 3).unwrap();
    let glyphs = HashMap::from([(0u8, ".".to_string()), (1u8, "#".to_string())]);
    for row in text(&cell, Some(&glyphs)) {
        println!("{row}");
    }
}
