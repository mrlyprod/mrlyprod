// The zeta function at whole numbers.
// Even arguments are exact multiples of a power of pi: zeta(2) = pi^2/6, the Basel sum.
// cargo run -p mrly-num --example zeta_values

use mrly_num::lattice::zeta_whole;

fn main() {
    for s in 2..=8 {
        println!("zeta({s}) = {:.10}", zeta_whole(s).unwrap());
    }
}
