// Copies of a flattened 2x2x2 cube packed around a center into a hexagonal disc of 1 ring (7 copies).
// copies(rings) gives the count: 1 + 3 * rings * (rings + 1).
// cargo run -p mrly-six --example hex_disc_svg

use mrly_six::{disc, iso, svg};
use mrly_three::ones;

fn main() {
    let flat = iso(&ones(2, 1).unwrap()).unwrap();
    let disc = disc(&flat, 1).unwrap();
    println!("{}", svg(&disc, 12, None, 0, None).unwrap());
}
