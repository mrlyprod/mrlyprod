// An elementary rule is one byte. Its 8 bits answer: given left, centre and right cells, what is the new centre?
// Rule 30 is 00011110 in binary, so only the neighbourhoods 100, 011, 010 and 001 give 1.
// cargo run -p mrly-wolfram --example rule
use mrly_wolfram::{corner_bits, lambda, output, popcount};

fn main() {
    let rule = 30u8;
    println!(
        "rule {rule}, bits from neighbourhood 000 up: {:?}",
        corner_bits(rule)
    );
    println!();
    println!("left centre right -> new");
    for n in (0..8u8).rev() {
        let (l, c, r) = (n >> 2 & 1, n >> 1 & 1, n & 1);
        println!("   {l}     {c}      {r}   ->  {}", output(rule, l, c, r));
    }
    println!();
    println!(
        "ones: {}, lambda (ones / 8): {}",
        popcount(rule),
        lambda(rule)
    );
}
