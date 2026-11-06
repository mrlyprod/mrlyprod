// The coprime gasket count at each level from 0 to 12.
// The count grows about threefold per level, like a Sierpinski gasket.
// cargo run -p mrly-num --example gasket_count

use mrly_num::coprime::gasket;

fn main() {
    for level in 0..=12 {
        println!("level {level:>2} {}", gasket(level).unwrap());
    }
}
