// The Ulam spiral: write the numbers in a square spiral and mark the primes.
// They fall on diagonal lines. # is a prime, . is not. Printed: the numbers 1 to 1681 on a 41 by 41 grid.
// cargo run -p mrly-ulam --example primes
use mrly_ulam::{Lattice, Mark, marks};

fn main() {
    let side = 41;
    let lattice = Lattice::Square;
    let prime = marks(Mark::Prime, lattice.count(side));
    let r = lattice.radius(side) as i64;
    for y in (-r..=r).rev() {
        let line: Vec<&str> = (-r..=r)
            .map(|x| {
                if prime[lattice.n(x, y) as usize] == 1 {
                    "#"
                } else {
                    "."
                }
            })
            .collect();
        println!("{}", line.join(" "));
    }
}
