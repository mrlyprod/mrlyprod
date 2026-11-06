// Random cells from a fixed seed: the same seed always gives the same picture.
// Each cell is filled with chance 0.4.
// cargo run -p mrly-two --example seeded_noise

use mrly_rand::Rng;
use mrly_two::{noise, text};
use std::collections::HashMap;

fn main() {
    let glyphs = HashMap::from([(0u8, ".".to_string()), (1u8, "#".to_string())]);
    let cell = noise(16, 1, 0.4, &mut Rng::new(1)).unwrap();
    for row in text(&cell, Some(&glyphs)) {
        println!("{row}");
    }
}
