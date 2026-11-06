// Filled and empty cells of the 2D carpet (code 7, 3x3 tile) at each level, with the filled share.
// Filled cells multiply by 8, grid cells by 9, so the share (8/9)^level shrinks toward 0.
// cargo run -p mrly-sequences --example fill_per_level

use mrly_math::Code;
use mrly_sequences::counts::{fill, rational, void};

fn main() {
    let carpet = Code::from(7u64);
    println!("level filled empty share");
    for level in 1..=6 {
        let (top, bottom) = rational(carpet, 3, 2, level, 2).unwrap();
        println!(
            "{level:>5} {:>6} {:>5} {top}/{bottom}",
            fill(carpet, 3, 2, level, 2).unwrap(),
            void(carpet, 3, 2, level, 2).unwrap()
        );
    }
}
