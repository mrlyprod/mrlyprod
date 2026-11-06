// How many primes are below x? The true count against two estimates.
// x/ln(x) is the simple guess, li(x) is the sharper one.
// cargo run -p mrly-num --example prime_counting

use mrly_num::prime::chart;

fn main() {
    println!("{:>7} {:>6} {:>9} {:>9}", "x", "pi(x)", "x/ln x", "li(x)");
    for r in chart(100_000, 10) {
        println!("{:>7} {:>6} {:>9.1} {:>9.1}", r.x, r.pi, r.ratio, r.li);
    }
}
