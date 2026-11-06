// How many zeros lie between heights 0 and t: N(t).
// Riemann and von Mangoldt gave a formula, t/(2 pi) * ln(t/(2 pi e)) + 7/8, that is off by less than a few zeros.
// cargo run -p mrly-zeta --example count
use mrly_zeta::Line;
use std::f64::consts::{E, PI};

fn main() {
    let line = Line::new();
    println!("       t  counted  formula");
    for t in [50.0, 100.0, 200.0, 500.0, 1000.0, 2000.0] {
        let formula = t / (2.0 * PI) * (t / (2.0 * PI * E)).ln() + 7.0 / 8.0;
        println!("{t:>8}  {:>7}  {formula:>7.2}", line.count(t));
    }
}
