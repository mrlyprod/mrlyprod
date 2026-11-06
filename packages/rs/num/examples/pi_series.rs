// Three old ways to reach pi, each with 1000 terms.
// All three are slow: 1000 terms give only about 3 correct digits.
// cargo run -p mrly-num --example pi_series

use mrly_num::series::{basel, leibniz, wallis_half_pi};
use std::f64::consts::PI;

fn main() {
    println!("Leibniz  4(1 - 1/3 + 1/5 ...)   {:.6}", 4.0 * leibniz(1000));
    println!(
        "Wallis   2 * product             {:.6}",
        2.0 * wallis_half_pi(1000)
    );
    println!(
        "Basel    sqrt(6 * sum 1/k^2)     {:.6}",
        (6.0 * basel(1000)).sqrt()
    );
    println!("pi                               {PI:.6}");
}
