// How many copies fill a hexagonal disc of r rings: 1, 7, 19, 37 ...
// cargo run -p mrly-six --example copies_in_disc

use mrly_six::copies;

fn main() {
    for rings in 0..=6 {
        println!("rings {rings}: {} copies", copies(rings).unwrap());
    }
}
