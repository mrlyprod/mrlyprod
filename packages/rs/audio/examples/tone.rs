// One second of A4 in several timbres, with its pitch measured back from the samples.
// Each tone is scaled to peak at 1. Counting the times it rises through zero gives the frequency.
// cargo run -p mrly-audio --example tone
use mrly_audio::{MOST, RATE, Timbre, Wave, hertz, peak, tone};

fn main() -> Result<(), String> {
    println!("shape     harmonics  samples  peak   rises");
    for (shape, harmonics) in [
        (Wave::Sine, 1),
        (Wave::Triangle, 8),
        (Wave::Square, 3),
        (Wave::Sawtooth, 8),
    ] {
        let timbre = Timbre::new(shape, harmonics);
        let samples = tone(hertz(69), &timbre, 1.0)?;
        let rises = samples
            .windows(2)
            .filter(|pair| pair[0] <= 0.0 && pair[1] > 0.0)
            .count();
        println!(
            "{:<9} {harmonics:>9}  {:>7}  {:.3}  {rises:>5}",
            shape.name(),
            samples.len(),
            peak(&samples)
        );
    }

    // A tone holds at most MOST samples, one hour; a longer one is refused before anything is allocated.
    let long = MOST as f64 / RATE as f64 + 1.0;
    println!(
        "{long} s: {}",
        tone(440.0, &Timbre::default(), long).unwrap_err()
    );
    Ok(())
}
