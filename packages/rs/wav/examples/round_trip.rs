// One second of a 440 Hz sine, written in every sample format and read back.
// Fewer bits means bigger steps between levels, so a bigger rounding error. Float keeps every bit.
// cargo run -p mrly-wav --example round_trip
use mrly_wav::{decode, encode, encode_float};

fn main() -> Result<(), String> {
    let rate = 44100;
    let sine: Vec<f32> = (0..rate)
        .map(|i| 0.8 * (std::f32::consts::TAU * 440.0 * i as f32 / rate as f32).sin())
        .collect();
    println!("format      bytes  worst error  about one step");
    let show = |label: &str, bytes: Vec<u8>, step: f64| -> Result<(), String> {
        let back = decode(&bytes)?;
        let worst = sine
            .iter()
            .zip(&back.samples)
            .map(|(a, b)| (a - b).abs())
            .fold(0.0f32, f32::max);
        println!("{label:<9} {:>7}  {worst:<11.3e}  {step:.3e}", bytes.len());
        Ok(())
    };
    for bits in [8, 16, 24, 32] {
        let step = 1.0 / (1u64 << (bits - 1)) as f64;
        show(&format!("pcm {bits}"), encode(&sine, 1, rate, bits)?, step)?;
    }
    show("float 32", encode_float(&sine, 1, rate)?, 0.0)?;

    // Stereo is the same samples interleaved: left, right, left, right.
    let stereo: Vec<f32> = sine.iter().flat_map(|&s| [s, -s]).collect();
    let back = decode(&encode(&stereo, 2, rate, 16)?)?;
    println!(
        "stereo: {} channels, {} frames, {:.1} s at {} Hz",
        back.channels,
        back.frames(),
        back.seconds(),
        back.rate
    );
    Ok(())
}
