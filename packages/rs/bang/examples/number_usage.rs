// Write a number in base 2^d: each digit names a corner. The set of corners it uses is its usage.
// 2D base 2 has 4 corners, so numbers are read in base 4. 27 is 123 in base 4, which uses corners 1, 2 and 3.
// cargo run -p mrly-bang --example number_usage

use mrly_bang::press::{distinct, usage};

fn main() {
    for number in [0u128, 1, 5, 27, 100, 255] {
        let code = usage(number, 2, 2).unwrap();
        println!(
            "{number:>3} usage {:04b} corners {}",
            code.get(),
            distinct(number, 2, 2).unwrap()
        );
    }
}
