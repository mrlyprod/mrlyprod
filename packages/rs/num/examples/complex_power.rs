// Raising a real number to a complex power: 2^(a + bi) = 2^a (cos(b ln 2) + i sin(b ln 2)).
// The result walks around a spiral as b grows; its length stays 2^a.
// cargo run -p mrly-num --example complex_power

use mrly_num::complex::{Complex, raise};

fn main() {
    for b in 0..6 {
        let z = raise(2.0, Complex::new(1.0, b as f64));
        println!(
            "2^(1+{b}i) = {:>7.4} {:+.4}i  length {:.4}",
            z.re,
            z.im,
            Complex::abs(z)
        );
    }
}
