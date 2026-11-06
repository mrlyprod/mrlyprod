// Goldbach: every even number above 2 seems to be a sum of two primes.
// This counts the ways for each even number from 4 to 60. None is zero.
// cargo run -p mrly-num --example goldbach_counts

use mrly_num::prime::goldbach_record;

fn main() {
    for (i, ways) in goldbach_record(60).iter().enumerate() {
        let even = 2 * (i + 2);
        println!("{even:>2} {}", "#".repeat(*ways));
    }
}
