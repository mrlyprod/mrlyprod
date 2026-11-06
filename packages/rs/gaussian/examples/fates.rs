// What happens to a rational prime p when it moves into the Gaussian or Eisenstein integers.
// Split: it breaks into two different primes. Inert: it stays prime. Ramified: it is a unit times a square of a prime.
// cargo run -p mrly-gaussian --example fates
use mrly_gaussian::Ring;

fn main() {
    println!("{:>3}  {:<10} {:<10}", "p", "gaussian", "eisenstein");
    for p in 2..=40u64 {
        let g = Ring::Gaussian.fate(p);
        let e = Ring::Eisenstein.fate(p);
        if g.prime() {
            println!("{p:>3}  {:<10} {:<10}", g.word(), e.word());
        }
    }
}
