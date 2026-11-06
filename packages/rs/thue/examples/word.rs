// The Thue-Morse word: 0 1 1 0 1 0 0 1 ...
// Two ways to build it give the same word: count the 1 bits of each place, or double by substitution (0 -> 01, 1 -> 10).
// cargo run -p mrly-thue --example word
use mrly_thue::{digits, stage, substitution};

fn show(word: &[u8]) -> String {
    word.iter().map(|b| b.to_string()).collect()
}

fn main() {
    println!("by bit count   {}", show(&digits(32)));
    println!("by doubling    {}", show(&substitution(32)));
    println!();
    println!("each doubling round:");
    for rounds in 0..5 {
        println!("  {} {}", rounds, show(&stage(rounds)));
    }
}
