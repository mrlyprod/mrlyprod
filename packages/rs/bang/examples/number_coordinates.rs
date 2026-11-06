// A number can be unwoven into one coordinate per axis, and woven back with interleave.
// Digits in base 4 split into a pair of base-2 digits, one per axis.
// cargo run -p mrly-bang --example number_coordinates

use mrly_bang::press::{coordinates, interleave};

fn main() {
    for number in [0u128, 6, 27, 100, 1000] {
        let coords = coordinates(number, 2, 2).unwrap();
        let back = interleave(&coords, 2).unwrap();
        println!("{number:>4} -> {coords:?} -> {back}");
    }
}
