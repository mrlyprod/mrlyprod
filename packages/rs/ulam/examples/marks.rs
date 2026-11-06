// Four ways to mark a number n, from 1 to 40.
// prime, twin (a prime with a prime 2 away), squarefree (no square divides n), and the Mobius value (0, 1 or -1).
// cargo run -p mrly-ulam --example marks
use mrly_ulam::{Mark, marks};

fn main() {
    let limit = 40;
    print!("{:<11}", "n");
    for n in 1..=limit {
        print!("{n:>3}");
    }
    println!();
    for mark in [Mark::Prime, Mark::Twin, Mark::Squarefree, Mark::Mobius] {
        print!("{:<11}", mark.name());
        for value in &marks(mark, limit)[1..] {
            print!("{value:>3}");
        }
        println!();
    }
}
