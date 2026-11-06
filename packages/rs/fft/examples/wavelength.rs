// Find the stripe spacing of a pattern from its spectrum.
// cargo run -p mrly-fft --example wavelength
use mrly_fft::{magnitude_spectrum, peak_ring, peak_wavelength, radial_profile};
use std::f64::consts::PI;

fn main() {
    // Vertical stripes that repeat every 8 cells across a 32x32 field.
    let size = 32;
    let field: Vec<f64> = (0..size * size)
        .map(|i| (2.0 * PI * (i % size) as f64 / 8.0).cos())
        .collect();

    // The spectrum is centered; radial_profile averages it in rings from the center.
    let spectrum = magnitude_spectrum(&field, size).unwrap();
    let profile = radial_profile(&spectrum, size).unwrap();
    for (ring, value) in profile.iter().enumerate().take(8) {
        println!("ring {ring} {value:>7.2}");
    }

    // 32 / 8 = 4 cycles across the field, so the peak is ring 4 and the wavelength is 8.
    println!("peak ring {}", peak_ring(&profile));
    println!("wavelength {}", peak_wavelength(&profile, size));
}
