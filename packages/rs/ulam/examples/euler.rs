// Euler's polynomial k^2 + k + 41 gives a prime for every k from 0 to 39.
// diagonal() lists the cells a polynomial visits and counts how many are prime.
// cargo run -p mrly-ulam --example euler
use mrly_ulam::{Lattice, diagonal};

fn main() {
    let found = diagonal(Lattice::Square, 41, 1, 1, 41);
    println!("numbers on the spiral  {}", found.top);
    println!(
        "primes among them      {} ({:.1}%)",
        found.primes,
        100.0 * found.density
    );
    println!();
    println!("values visited         {}", found.values.len());
    println!(
        "prime                  {} ({:.1}%)",
        found.hits,
        100.0 * found.share
    );
    println!("longest run of primes  {}", found.streak);
    println!();
    println!("first values and their cells");
    for (v, cell) in found.values.iter().zip(&found.cells).take(6) {
        println!("  {v:>4} at {cell:?}");
    }
}
