// Count filled and empty cells of the carpet as the level rises.
// Filled cells multiply by 8 each level, so the filled share (8/9)^level goes to zero.
// cargo run -p mrly-math --example carpet_counts

use mrly_math::atoms::carpet_2d;
use mrly_math::cell::census::{fills, voids};
use mrly_math::cell::designs::grow;
use mrly_math::cell::models::Cell2d;

fn main() {
    println!("level filled empty");
    for level in 1..=5 {
        let cell: Cell2d = grow(carpet_2d(3), level).unwrap();
        println!("{level:>5} {:>6} {:>5}", fills(&cell), voids(&cell));
    }
}
