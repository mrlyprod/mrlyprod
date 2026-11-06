// Build a design by choosing which corner sums are filled, instead of a code.
// Levels count the odd coordinates: level 0 is the (0,0) corner, level 1 are (0,1) and (1,0), level 2 is (1,1).
// Here levels 0 and 1 are filled, which is the carpet.
// cargo run -p mrly-two --example level_set_design

use mrly_two::{level_set, text};
use std::collections::HashMap;

fn main() {
    let glyphs = HashMap::from([(0u8, ".".to_string()), (1u8, "#".to_string())]);
    let cell = level_set(3, &[0, 1], 2, 0, 2).unwrap();
    for row in text(&cell, Some(&glyphs)) {
        println!("{row}");
    }
}
