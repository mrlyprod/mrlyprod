// The snail gives each number n on the spiral a tile whose side is a power of the base: base^k, where k counts the digits of n minus one.
// Growth::Prime enlarges only the primes; Growth::Every enlarges every number. Base 2, numbers 1 to 24.
// cargo run -p mrly-ulam --example snail
use mrly_ulam::{Growth, snail};

fn main() {
    let primes = snail(2, 24, Growth::Prime);
    let every = snail(2, 24, Growth::Every);
    println!("  n  prime  side (primes grow)  side (every grows)");
    for (a, b) in primes.tiles.iter().zip(&every.tiles) {
        println!("{:>3}  {:>5}  {:>18}  {:>18}", a.n, a.prime, a.side, b.side);
    }
    println!();
    println!("tiles per level, primes grow: {:?}", primes.levels);
    println!("tiles per level, every grows: {:?}", every.levels);
}
