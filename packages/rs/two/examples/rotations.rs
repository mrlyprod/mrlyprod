// A design that is not symmetric, turned a quarter turn at a time.
// Code 11 on a 4x4 tile never matches itself when turned, so each turn gives a new picture. Rotation k turns it k times.
// cargo run -p mrly-two --example rotations

use mrly_math::Code;
use mrly_two::{create, text};
use std::collections::HashMap;

fn main() {
    let glyphs = HashMap::from([(0u8, ".".to_string()), (1u8, "#".to_string())]);
    for rotation in 0..4 {
        let cell = create(Code::from(11u64), 4, 1, rotation, 2).unwrap();
        println!("rotation {rotation}");
        for row in text(&cell, Some(&glyphs)) {
            println!("{row}");
        }
        println!();
    }
}
