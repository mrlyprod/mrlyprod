// Filled cells in each named 3D design on a 3x3x3 tile.
// Carpet (the sponge) fills 20 of 27. Dust, point and star are the sparse ones.
// cargo run -p mrly-three --example design_fills

use mrly_math::catalog::{ANTIS_3D, CLASSICS_3D};
use mrly_three::{fills, named};

fn main() {
    for design in CLASSICS_3D.iter().chain(ANTIS_3D.iter()) {
        let cell = named(*design, 3, 1).unwrap();
        println!("{:<7} {:>2} of 27", design.name(), fills(&cell));
    }
}
