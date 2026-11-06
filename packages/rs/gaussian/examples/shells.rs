// How many lattice points sit at each norm n: the number of ways to write n as a^2 + b^2.
// Norm 25 has 12 points: (5, 0) and (3, 4) in every sign and order. Norm 65 has 16 because 65 = 1 + 64 = 16 + 49.
// cargo run -p mrly-gaussian --example shells
use mrly_gaussian::{Ring, peak, shells};

fn main() {
    let limit = 65;
    let counts = shells(Ring::Gaussian, limit);
    println!("norm points");
    for (n, c) in counts.iter().enumerate().skip(1) {
        if *c > 0 {
            println!("{n:>4} {c:>3} {}", "#".repeat(*c as usize));
        }
    }
    let (n, c) = peak(Ring::Gaussian, limit);
    println!("most points up to {limit}: {c} at norm {n}");
}
