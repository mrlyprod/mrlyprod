// The explicit formula: the zeros of zeta draw the staircase of prime powers.
// psi(x) adds ln(p) at every prime power p^k up to x. The formula rebuilds it from the zeros alone.
// Columns: the true staircase, then the formula with 10 zeros, then with 50.
// cargo run -p mrly-zeta --example primes
use mrly_zeta::{Line, psi_formula, psi_stair};

fn main() {
    let zeros = Line::new().zeros(50);
    println!("     x    psi(x)   10 zeros   50 zeros");
    for x in [10usize, 20, 30, 50, 100, 200, 300] {
        let exact = psi_stair(x)[x - 1];
        println!(
            "{x:>6}  {exact:>8.3}  {:>9.3}  {:>9.3}",
            psi_formula(x as f64, &zeros[..10]),
            psi_formula(x as f64, &zeros)
        );
    }
}
