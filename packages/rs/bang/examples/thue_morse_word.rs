// Two letters, a carpet and a net, placed by the Thue-Morse rule.
// The rule is the parity of the number of 1 bits in the position: 0 1 1 0 1 0 0 1 ...
// cargo run -p mrly-bang --example thue_morse_word

use mrly_bang::word::{Schedule, dimension, spell};
use mrly_math::factory::MagicLayer;
use mrly_math::name::Bang;

fn main() {
    let carpet = MagicLayer::new(Bang::new(7, 2, 2), 3);
    let net = MagicLayer::new(Bang::new(14, 2, 2), 3);
    let word = spell(Schedule::ThueMorse, (carpet.clone(), net), 16);
    let letters: String = word
        .iter()
        .map(|l| if *l == carpet { 'C' } else { 'N' })
        .collect();
    println!("{letters}");
    println!("dimension {:.4}", dimension(&word).unwrap());
}
