// Vibration modes of a free square plate made of 16 by 16 cells (design code 15 is the full square). figures() returns the lowest modes: their eigenvalues and how many nodal domains each cuts the plate into.
// On a full square the eigenvalues are known: 4 - 2 cos(a pi / 16) - 2 cos(b pi / 16) for whole numbers a, b from 0 to 15. The last column is that formula, sorted.
// Repeated values are modes that look the same turned a quarter turn.
// cargo run -p mrly-chladni --example square_modes
use mrly_chladni::nodal::figures;
use std::f64::consts::PI;

fn main() {
    let count = 10;
    let modes = figures(15, 4, 2, count).unwrap();
    let mut exact: Vec<f64> = Vec::new();
    for a in 0..16 {
        for b in 0..16 {
            exact.push(
                4.0 - 2.0 * (a as f64 * PI / 16.0).cos() - 2.0 * (b as f64 * PI / 16.0).cos(),
            );
        }
    }
    exact.sort_by(f64::total_cmp);
    println!("side {}", modes.side);
    println!("mode  value    domains  exact");
    for (k, want) in exact.iter().take(count).enumerate() {
        println!(
            "{k:>4}  {:.5}  {:>7}  {:.5}",
            modes.values[k].abs(),
            modes.domains[k],
            want.abs()
        );
    }
}
