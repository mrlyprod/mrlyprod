// How much of a picture repeats as it turns. harmonics() gives the power at each rotation order m, starting with m = 0 (the plain average).
// A shape with n lobes has power only at orders n, 2n, 3n...; turns() reads n back as the gcd of the orders that carry power.
// The raster is anti-aliased: hard pixel edges leak a little power into every order and hide the symmetry.
// cargo run -p mrly-spin --example harmonics
use mrly_spin::{harmonics, turns};

fn flower(size: usize, lobes: usize) -> Vec<f32> {
    let c = size as f64 / 2.0;
    let sub = 4;
    let mut data = vec![0.0; size * size];
    for row in 0..size {
        for col in 0..size {
            let mut hit = 0;
            for a in 0..sub {
                for b in 0..sub {
                    let dx = col as f64 + (b as f64 + 0.5) / sub as f64 - c;
                    let dy = row as f64 + (a as f64 + 0.5) / sub as f64 - c;
                    let edge = c * 0.6 * (1.0 + 0.5 * (lobes as f64 * dy.atan2(dx)).cos());
                    if dx.hypot(dy) < edge {
                        hit += 1;
                    }
                }
            }
            data[row * size + col] = hit as f32 / (sub * sub) as f32;
        }
    }
    data
}

fn main() {
    let size = 64;
    for lobes in [3, 5, 6] {
        let power = harmonics(&flower(size, lobes), size, 32, 14).unwrap();
        let total: f64 = power.iter().sum();
        println!("{lobes} lobes: turns() = {}", turns(&power));
        for (m, p) in power.iter().enumerate() {
            let share = p / total;
            if share > 1e-3 {
                println!("  m={m:>2} {share:>5.3}");
            }
        }
    }
}
