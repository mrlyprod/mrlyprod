// Hand it terms of a sequence and it finds the rule that makes them.
// Fibonacci comes back as a(n) = a(n-1) + a(n-2); the growth rate is the golden ratio.
// cargo run -p mrly-num --example find_recurrence

use mrly_num::blend::{growth, recurrence};
use mrly_num::series::fibonacci;

fn main() {
    let terms: Vec<i128> = fibonacci(300).iter().map(|&t| t as i128).collect();
    println!("terms: {:?}", terms);
    let rule = recurrence(&terms).unwrap();
    println!("coefficients (num, den): {rule:?}");
    println!("growth per step: {:.6}", growth(&rule));
}
