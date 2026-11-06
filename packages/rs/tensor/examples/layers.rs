// layers numbers each cell by its ring around the center.
// cargo run -p mrly-tensor --example layers
use mrly_tensor::{Dtype, Tensor};

fn main() {
    // The center cell is ring 0, the next square around it is ring 1, and so on.
    let rings = Tensor::new(vec![7, 7]).layers(Dtype::U8);
    for row in rings.bytes().unwrap().chunks(7) {
        let cells: Vec<String> = row.iter().map(u8::to_string).collect();
        println!("{}", cells.join(" "));
    }
    // An even side has no single center cell, so the middle 2x2 are all ring 0.
    let even = Tensor::new(vec![4, 4]).layers(Dtype::U8);
    println!();
    for row in even.bytes().unwrap().chunks(4) {
        let cells: Vec<String> = row.iter().map(u8::to_string).collect();
        println!("{}", cells.join(" "));
    }
}
