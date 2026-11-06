// Overlay two checkerboards with m and n squares per side and measure how alike their lit cells are (0 means independent).
// Boards with no common factor, like 3 and 5, give exactly 0; 3 and 9 share the factor 3 and do not. If n is prime, witness(n) is a whole row of zeros.
// The sampled value (draw both boards on a common grid and count) agrees with the exact formula.
// cargo run -p mrly-moire --example coprime
use mrly_moire::pairs::{correlation, sampled, witness};

fn main() {
    println!("{:>3} {:>9} {:>8} {:>6}", "n", "max corr", "at m", "prime");
    for n in (3..=33).step_by(2) {
        let w = witness(n).unwrap();
        println!("{n:>3} {:>9.4} {:>8} {:>6}", w.max, w.at, w.prime);
    }
    println!(
        "pair (3, 9): exact {:.6}, sampled {:.6}",
        correlation(3, 9),
        sampled(3, 9).unwrap()
    );
    println!(
        "pair (3, 5): exact {:.6}, sampled {:.6}",
        correlation(3, 5),
        sampled(3, 5).unwrap()
    );
}
