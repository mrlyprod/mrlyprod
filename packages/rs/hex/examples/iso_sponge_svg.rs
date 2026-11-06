// The level 1 Menger sponge in isometric projection: the three visible faces of every cube as triangles.
// cargo run -p mrly-hex --example iso_sponge_svg

use mrly_hex::{iso, svg};
use mrly_three::carpet;

fn main() {
    let flat = iso(&carpet(3, 1).unwrap()).unwrap();
    println!("{}", svg(&flat, 16, None, 0, None).unwrap());
}
