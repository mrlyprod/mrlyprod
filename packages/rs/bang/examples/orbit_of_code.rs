// Every code in one family: all the ways to flip or swap the grid that give the same shape.
// Here the family of code 1, a single filled corner, in 2D.
// cargo run -p mrly-bang --example orbit_of_code

use mrly_bang::universe::orbit;
use mrly_math::Code;

fn main() {
    for code in orbit(Code::from(1u64), 2) {
        println!("code {:>2} = {:04b}", code.get(), code.get());
    }
}
