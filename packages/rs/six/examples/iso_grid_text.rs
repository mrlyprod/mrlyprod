// A 2x2x2 cube seen from the corner, stored as a grid of triangle cells.
// Values: 2 empty grid, 3 top face, 4 left face, 5 right face.
// cargo run -p mrly-six --example iso_grid_text

use mrly_six::iso;
use mrly_three::ones;
use mrly_two::text;

fn main() {
    let cube = ones(2, 1).unwrap();
    let flat = iso(&cube).unwrap();
    println!("width {} height {}", flat.width(), flat.height());
    for row in text(&flat.cell, None) {
        println!("{row}");
    }
}
