// Which square sizes can a tile have? A design of side n at level L has side n^L.
// powers() lists those (n, L) for every size from 4 to 64; products() lists two odd sides multiplied, as in a magic tile.
// cargo run -p mrly-tile --example sizes
use mrly_tile::recipe::{Parity, powers, products};

fn main() {
    println!("n^L from 4 to 64, any n:");
    for (n, level) in powers(4, 64, Parity::Both) {
        println!("  {n}^{level} = {}", n.pow(level as u32));
    }
    println!();
    println!("two odd sides multiplied, size 9 to 30:");
    for pair in products(9, 30, 2, Parity::Odds) {
        println!("  {} x {} = {}", pair[0], pair[1], pair[0] * pair[1]);
    }
}
