// Sierpinski carpet as a radix fractal: base 3, with the eight digits of a 3 by 3 tile that leave out the centre.
// The tile code has one bit per cell, so 495 = 0b111101111 keeps all cells but the middle one.
// cargo run -p mrly-radix --example carpet
use mrly_radix::tile;
use std::collections::HashSet;

fn main() {
    let radix = tile(3, 0b1_1110_1111).unwrap();
    println!(
        "{} digits, dimension {:.4}",
        radix.size(),
        radix.dimension()
    );
    let seen: HashSet<(i64, i64)> = radix.words(3).into_iter().collect();
    for y in (0..27).rev() {
        let row: String = (0..27)
            .map(|x| if seen.contains(&(x, y)) { "##" } else { "  " })
            .collect();
        println!("{}", row.trim_end());
    }
}
