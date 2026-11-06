// A square grid looks the same after a 90 degree turn, so only turns modulo 90 matter.
// period() counts the steps of an increment it takes to land on a multiple of 90, or None if that takes over 360 steps.
// eyes(q) lists the increments 90 * p / q degrees (p / q in lowest terms, q up to the bound); the period of each is its denominator.
// cargo run -p mrly-tourbillon --example periods
use mrly_tourbillon::{eyes, period};

fn main() {
    for increment in [90.0, 45.0, 30.0, 22.5, 10.0, 7.0, 137.5] {
        match period(increment) {
            Some(count) => println!("increment {increment:>5}: back after {count} steps"),
            None => println!("increment {increment:>5}: never"),
        }
    }
    println!("eyes with denominator up to 3");
    for eye in eyes(3) {
        println!(
            "  {:>7.3} degrees  {:>3}/{}  period {:?}",
            eye.angle,
            eye.numer,
            eye.denom,
            period(eye.angle)
        );
    }
}
