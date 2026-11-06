// Write 1 in the middle of a grid, then spiral outward: right, up, left, down, with the arms growing.
// Lattice::xy gives the cell of the number n, and Lattice::n goes back. Printed: the first 49 numbers.
// cargo run -p mrly-ulam --example spiral
use mrly_ulam::Lattice;

fn main() {
    let lattice = Lattice::Square;
    for y in (-3..=3).rev() {
        let line: Vec<String> = (-3..=3)
            .map(|x| format!("{:>3}", lattice.n(x, y)))
            .collect();
        println!("{}", line.join(""));
    }
    println!();
    for n in [1, 2, 3, 10, 25, 49] {
        println!("n = {n:>2} sits at {:?}", lattice.xy(n));
    }
}
