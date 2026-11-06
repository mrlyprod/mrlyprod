// Distinct shapes in one dimension, for a ring of q cells (base q).
// These are binary bracelets: 2, 3, 4, 6, 8, 13, 18, 30.
// cargo run -p mrly-bang --example shapes_by_base

use mrly_bang::baseq::bracelets;

fn main() {
    for (i, count) in bracelets(8).unwrap().iter().enumerate() {
        println!("base {} {count}", i + 1);
    }
}
