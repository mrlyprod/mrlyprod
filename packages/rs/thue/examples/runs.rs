// Runs of equal letters in the Thue-Morse word are never longer than two.
// The boundary word marks where the letter changes, and its own runs show the pattern is not periodic.
// cargo run -p mrly-thue --example runs
use mrly_thue::{boundary, digits, runs};

fn main() {
    let word = digits(48);
    let lengths = runs(&word);
    let text: String = lengths.iter().map(|n| n.to_string()).collect();
    println!("run lengths  {text}");
    println!("longest run  {}", lengths.iter().max().unwrap());
    let ones = lengths.iter().filter(|&&n| n == 1).count();
    println!("runs of 1 and 2  {} and {}", ones, lengths.len() - ones);
    println!();
    let marks: String = boundary(&word).iter().map(|b| b.to_string()).collect();
    println!("changes      {marks}");
}
