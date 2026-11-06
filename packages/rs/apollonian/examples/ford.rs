// A strip packing between two parallel lines: its circles touch the bottom line at fractions.
// Each row below checks that every fraction p/q up to the order is touched, and nothing else.
// cargo run -p mrly-apollonian --example ford
use mrly_apollonian::{grow, shadow, touches};

fn main() {
    let packing = grow("strip", 200).unwrap();
    println!(
        "strip, curvature cap {}, {} circles",
        packing.cap,
        packing.circles.len()
    );
    println!();
    println!("fractions touching the line, left to right:");
    let fractions: Vec<String> = touches(&packing)
        .iter()
        .filter(|t| t.den <= 5)
        .map(|t| format!("{}/{}", t.num, t.den))
        .collect();
    println!("  {}", fractions.join(" "));
    println!();
    println!("order  covered  expected  touched  missed  off-ford");
    for order in 1..=8 {
        let s = shadow(&packing, order).unwrap();
        println!(
            "{:>5}  {:>7}  {:>8}  {:>7}  {:>6}  {:>8}",
            order, s.covered, s.nodes, s.touched, s.missed, s.offford
        );
    }
}
