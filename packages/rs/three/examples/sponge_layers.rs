// The Menger sponge as text, level 2: a 9x9x9 cube printed as nine 9x9 layers, top to bottom.
// The 3D carpet keeps a cell when at most one of its coordinates is odd.
// cargo run -p mrly-three --example sponge_layers

use mrly_three::{carpet, text};
use std::collections::HashMap;

fn main() {
    let cell = carpet(3, 2).unwrap();
    let glyphs = HashMap::from([(0u8, ".".to_string()), (1u8, "#".to_string())]);
    for (i, row) in text(&cell, Some(&glyphs)).iter().enumerate() {
        if i > 0 && i % 9 == 0 {
            println!();
        }
        println!("{row}");
    }
}
