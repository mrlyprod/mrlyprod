// The level 1 sponge as a Wavefront OBJ model: corner points (v) and square faces (f).
// Only faces on the surface are written; faces between two filled cells are dropped.
// cargo run -p mrly-three --example sponge_obj

use mrly_three::{carpet, to_obj};

fn main() {
    print!("{}", to_obj(&carpet(3, 1).unwrap()));
}
