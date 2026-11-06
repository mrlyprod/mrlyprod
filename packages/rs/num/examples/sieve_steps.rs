// The sieve of Eratosthenes, one prime at a time, up to 50.
// Each step names the prime and how many new numbers it strikes out.
// cargo run -p mrly-num --example sieve_steps

use mrly_num::prime::Sieve;

fn main() {
    let mut sieve = Sieve::new(50);
    while !sieve.done() {
        let prime = sieve.step();
        println!("prime {prime:>2} strikes {} new numbers", sieve.struck());
    }
    // Once the sieve is done, everything left unstruck is prime.
    println!("primes up to 50: {}", sieve.count());
}
