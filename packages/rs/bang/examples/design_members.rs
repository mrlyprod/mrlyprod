// A number belongs to a design when every corner it uses is filled in the design.
// These are the first numbers that belong to the carpet (code 7).
// cargo run -p mrly-bang --example design_members

use mrly_bang::press::{count_below, members};
use mrly_math::Code;

fn main() {
    let carpet = Code::from(7u64);
    println!("{:?}", members(carpet, 2, 2, 24).unwrap());
    for limit in [16u128, 256, 4096] {
        println!(
            "below {limit}: {}",
            count_below(carpet, 2, 2, limit).unwrap()
        );
    }
}
