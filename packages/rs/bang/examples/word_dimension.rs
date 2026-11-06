// A word stacks designs one inside another. Its fractal dimension is log(fill) / log(side).
// Carpet repeated: 8 of 9 cells filled per layer gives log 8 / log 3 = 1.8928.
// cargo run -p mrly-bang --example word_dimension

use mrly_bang::word::{dimension, fill, side};
use mrly_math::factory::MagicLayer;
use mrly_math::name::Bang;

fn main() {
    let carpet = MagicLayer::new(Bang::new(7, 2, 2), 3);
    for depth in 1..=4 {
        let word = vec![carpet.clone(); depth];
        println!(
            "depth {depth}: side {:>3} fill {:>5} dimension {:.4}",
            side(&word).unwrap(),
            fill(&word).unwrap(),
            dimension(&word).unwrap()
        );
    }
}
