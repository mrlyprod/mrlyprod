// Look down the cube's long diagonal and draw cells on a few of the planes x + y + z = h.
// Each height gets its own colors. The sponge is level 2.
// cargo run -p mrly-three --example diagonal_svg

use mrly_three::{diagonal_svg, levels_code};

fn main() {
    let sponge = levels_code(3, 2, &[0, 1]);
    let svg = diagonal_svg(sponge, 3, 2, 2, &[6, 9, 12, 15], 12).unwrap();
    println!("{svg}");
}
