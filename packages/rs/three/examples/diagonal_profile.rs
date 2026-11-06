// Slice the level 3 sponge by planes x + y + z = h and count the cells in each slice.
// support() gives the first and last height that has any cells.
// cargo run -p mrly-three --example diagonal_profile

use mrly_three::{levels_code, profile, support};

fn main() {
    let sponge = levels_code(3, 2, &[0, 1]);
    let counts = profile(sponge, 3, 2, 2).unwrap();
    for (height, count) in counts.iter().enumerate() {
        println!("h={height:>2} {count}");
    }
    println!("support: {:?}", support(&counts).unwrap());
}
