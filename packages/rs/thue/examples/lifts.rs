// A letter t(n) of the word becomes a plane: a rule combines the row i and the column j into one place.
// Each lift is shown at side 8, with # for 1 and . for 0.
// Parity and xor print the same grid: t(i xor j) = t(i) xor t(j).
// cargo run -p mrly-thue --example lifts
use mrly_thue::{LIFTS, lift};

fn main() {
    let side = 8;
    for kind in LIFTS {
        println!("{}  {}", kind.name(), kind.formula());
        for row in lift(kind, side).chunks(side) {
            let line: String = row
                .iter()
                .map(|&b| if b == 1 { '#' } else { '.' })
                .collect();
            println!("  {line}");
        }
        println!();
    }
}
