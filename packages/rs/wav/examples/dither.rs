// A sine too quiet for 8 bits: under one step of the scale.
// Plain rounding erases it. Adding a little noise first (dither) lets it survive as an average.
// cargo run -p mrly-wav --example dither
use mrly_wav::{decode, dither, encode};

// How strongly the output still follows the input: 1 is a perfect copy, 0 is nothing left.
fn follows(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let (na, nb): (f32, f32) = (
        a.iter().map(|x| x * x).sum::<f32>().sqrt(),
        b.iter().map(|x| x * x).sum::<f32>().sqrt(),
    );
    if na == 0.0 || nb == 0.0 {
        0.0
    } else {
        dot / (na * nb)
    }
}

fn main() -> Result<(), String> {
    let rate = 8000;
    let step = 1.0 / 128.0;
    println!("one 8-bit step is {step:.5}");
    println!("amplitude  plain  dithered");
    for part in [0.3f32, 0.45, 1.0, 4.0] {
        let quiet: Vec<f32> = (0..rate)
            .map(|i| part * step * (std::f32::consts::TAU * 200.0 * i as f32 / rate as f32).sin())
            .collect();
        let plain = decode(&encode(&quiet, 1, rate, 8)?)?.samples;
        let noisy = decode(&encode(&dither(&quiet, 8, 7)?, 1, rate, 8)?)?.samples;
        println!(
            "{part:>4} step  {:>5.2}  {:>8.2}",
            follows(&quiet, &plain),
            follows(&quiet, &noisy)
        );
    }
    Ok(())
}
