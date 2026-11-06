// The Farey sequence of order 6: all reduced fractions between 0 and 1 with denominator up to 6.
// Brightness is how many times the denominator fits in 6.
// cargo run -p mrly-num --example farey_sequence

use mrly_num::lattice::farey;

fn main() {
    for node in farey(6) {
        println!("{}/{} brightness {}", node.num, node.den, node.brightness);
    }
}
