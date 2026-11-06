// Perfect numbers equal the sum of their proper divisors (the aliquot sum).
// Printed for 1 to 30, with a star on the perfect ones.
// cargo run -p mrly-num --example divisor_sums

use mrly_num::factor::{aliquot, divisors};

fn main() {
    for n in 1..=30usize {
        let mark = if aliquot(n) == n { " *" } else { "" };
        println!(
            "{n:>2} divisors={:?} aliquot={}{mark}",
            divisors(n as u64),
            aliquot(n)
        );
    }
}
