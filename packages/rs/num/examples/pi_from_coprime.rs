// Pick two whole numbers at random: the chance they share no factor is 6/pi^2.
// Count coprime pairs in an n x n grid and turn that chance back into pi.
// cargo run -p mrly-num --example pi_from_coprime

use mrly_num::lattice::{coprime_pairs, pi_estimate};

fn main() {
    for n in [10, 100, 1000, 5000] {
        println!(
            "n={n:<5} coprime pairs={:<9} pi~{:.5}",
            coprime_pairs(n),
            pi_estimate(n)
        );
    }
    println!("pi    {:.5}", std::f64::consts::PI);
}
