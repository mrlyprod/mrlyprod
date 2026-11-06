// The symmetries of a grid: shifts and mirrors on each axis, plus swapping axes.
// The measured group size matches the formula for every base and dimension shown.
// cargo run -p mrly-bang --example symmetry_group

use mrly_bang::baseq::{group_order, predicted_group_order};

fn main() {
    println!("base dim measured formula");
    for base in 2..=4 {
        for dimension in 1..=3 {
            println!(
                "{base:>4} {dimension:>3} {:>8} {:>7}",
                group_order(base, dimension).unwrap(),
                predicted_group_order(base, dimension).unwrap()
            );
        }
    }
}
