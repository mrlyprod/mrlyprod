// Midi note numbers, their names and their frequencies.
// Each step up is a semitone: the frequency grows by the twelfth root of 2, so 12 steps double it.
// cargo run -p mrly-audio --example notes
use mrly_audio::{hertz, midi, name};

fn main() -> Result<(), String> {
    println!("midi  name  hertz");
    for note in 60..=72 {
        println!("{note:>4}  {:<4}  {:>8.3}", name(note), hertz(note));
    }

    // A4 is the tuning fork: midi 69 at 440 Hz. The piano runs from A0 to C8.
    for note in [21, 69, 108] {
        println!(
            "{:<4} is midi {note:>3} at {:.3} Hz",
            name(note),
            hertz(note)
        );
    }

    // Names parse back to numbers. A flat is the sharp of the note below.
    for text in ["C4", "C#4", "Db4", "Bb2", "G2", "C-1", "G9"] {
        println!("{text:<4} -> {}", midi(text)?);
    }
    println!("H4   -> {}", midi("H4").unwrap_err());

    // Every octave doubles exactly, from the bottom of midi to the top.
    let doubles = (0..116).all(|note| hertz(note + 12) == 2.0 * hertz(note));
    println!("octaves double exactly: {doubles}");
    Ok(())
}
