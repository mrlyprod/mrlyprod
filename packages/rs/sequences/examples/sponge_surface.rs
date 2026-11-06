// Exposed faces of the 3D carpet (the Menger sponge) at each level. Level 1 is 72, level 2 is 1056.
// The same numbers come from counting a built sponge in the three crate.
// cargo run -p mrly-sequences --example sponge_surface

use mrly_math::Code;
use mrly_sequences::counts::surface;

fn main() {
    let sponge = Code::from(23u64);
    for level in 1..=8 {
        println!("level {level}: {}", surface(sponge, 3, level, 2).unwrap());
    }
}
