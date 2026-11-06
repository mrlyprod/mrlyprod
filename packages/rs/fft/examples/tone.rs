// The FFT of a pure tone has two spikes, and the inverse brings the tone back.
// cargo run -p mrly-fft --example tone
use mrly_fft::fft;
use std::f64::consts::PI;

fn main() {
    // 16 samples holding exactly 3 cycles of a cosine.
    let n = 16;
    let original: Vec<f64> = (0..n)
        .map(|i| (2.0 * PI * 3.0 * i as f64 / n as f64).cos())
        .collect();

    // fft works in place on separate real and imaginary arrays. Length must be a power of two.
    let mut re = original.clone();
    let mut im = vec![0.0; n];
    fft(&mut re, &mut im, false).unwrap();

    // A cosine with 3 cycles puts n/2 = 8 at bin 3 and at its mirror, bin 13.
    for bin in 0..n {
        let magnitude = re[bin].hypot(im[bin]);
        let bar = "#".repeat(magnitude.round() as usize);
        println!(
            "{}",
            format!("bin {bin:>2}  {magnitude:>5.2} {bar}").trim_end()
        );
    }

    // inverse = true undoes it, up to a factor of n.
    fft(&mut re, &mut im, true).unwrap();
    let worst = re
        .iter()
        .zip(&original)
        .map(|(a, b)| (a / n as f64 - b).abs())
        .fold(0.0, f64::max);
    println!("round trip error below 1e-12: {}", worst < 1e-12);

    // Other lengths are refused.
    let (mut a, mut b) = (vec![0.0; 6], vec![0.0; 6]);
    println!("{}", fft(&mut a, &mut b, false).unwrap_err());
}
