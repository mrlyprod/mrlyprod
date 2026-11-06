// The same idea on a hexagonal grid: each number has six neighbours, and the spiral runs round in rings.
// # is a prime, . is not. Printed: the numbers 1 to 469, which is 12 rings.
// cargo run -p mrly-ulam --example hex
use mrly_ulam::{Lattice, Mark, marks};

fn main() {
    let lattice = Lattice::Hex;
    let radius = 12i64;
    let prime = marks(Mark::Prime, lattice.count(2 * radius as usize + 1));
    for r in -radius..=radius {
        let first = (-radius).max(-radius - r);
        let last = radius.min(radius - r);
        print!("{}", " ".repeat(r.unsigned_abs() as usize));
        let row: Vec<&str> = (first..=last)
            .map(|q| {
                if prime[lattice.n(q, r) as usize] == 1 {
                    "#"
                } else {
                    "."
                }
            })
            .collect();
        println!("{}", row.join(" "));
    }
}
