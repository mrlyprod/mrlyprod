// Mixing adds tracks sample by sample; normalizing scales the sum back to a chosen peak.
// cargo run -p mrly-audio --example mix
use mrly_audio::{Timbre, hertz, midi, mix, normalize, peak, tone};

fn main() -> Result<(), String> {
    let timbre = Timbre::default();
    let mut tracks = Vec::new();
    for note in ["C4", "E4", "G4"] {
        let samples = tone(hertz(midi(note)?), &timbre, 0.5)?;
        println!(
            "{note:<3} {} samples, peak {:.3}",
            samples.len(),
            peak(&samples)
        );
        tracks.push(samples);
    }

    // Three unit tones can add up to almost 3 where their peaks line up.
    let chord = mix(&tracks);
    println!("mix {} samples, peak {:.3}", chord.len(), peak(&chord));
    let level = normalize(&chord, 0.8);
    println!("normalized to 0.8, peak {:.3}", peak(&level));

    // A shorter track adds into the start of a longer one.
    let short = tone(hertz(midi("C5")?), &timbre, 0.1)?;
    println!("with a 0.1 s C5: {} samples", mix(&[chord, short]).len());
    Ok(())
}
