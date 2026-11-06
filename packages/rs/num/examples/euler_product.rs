// Euler's product: zeta(2) is also a product over primes of 1/(1 - p^-2).
// More primes get closer to the sum 1 + 1/4 + 1/9 + ... = pi^2/6.
// cargo run -p mrly-num --example euler_product

use mrly_num::series::{BASEL, euler_product};

fn main() {
    for limit in [10, 100, 1000, 10_000] {
        println!("primes up to {limit:<6} {:.8}", euler_product(2.0, limit));
    }
    println!("pi^2/6             {BASEL:.8}");
}
