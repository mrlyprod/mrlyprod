// Two number tables for 1 to 24.
// mobius: 0 if a square divides n, else +1 or -1 by the count of prime factors.
// totient: how many numbers up to n share no factor with n.
// cargo run -p mrly-num --example mobius_totient

use mrly_num::factor::{mobius, totient};

fn main() {
    println!(" n mobius totient");
    for n in 1..=24 {
        println!("{n:>2} {:>6} {:>7}", mobius(n), totient(n));
    }
}
