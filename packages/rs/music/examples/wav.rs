// A thirty second piece rendered to a wav file in memory, then read back.
// The bytes never leave the program: it prints their size and a short hash instead.
// The same seed gives the same bytes, so the hash only changes when the sound does.
// cargo run -p mrly-music --example wav
use mrly_music::{Scale, random, wav};
use mrly_name::sha::short;

fn main() -> Result<(), String> {
    let piece = random(Scale::Major, 7);
    let bytes = wav(&piece, 30.0, 1)?;
    let back = mrly_wav::decode(&bytes)?;
    println!("{} bytes, hash {}", bytes.len(), short(&bytes));
    println!(
        "{} Hz, {} channel, {} bits, {} frames, {:.2} s",
        back.rate,
        back.channels,
        back.bits,
        back.frames(),
        back.seconds()
    );
    let top = back.samples.iter().fold(0.0f32, |m, s| m.max(s.abs()));
    println!("peak {top:.4}");
    println!("seed 1 again: hash {}", short(&wav(&piece, 30.0, 1)?));
    println!("seed 2:       hash {}", short(&wav(&piece, 30.0, 2)?));
    for scale in [Scale::Dorian, Scale::Minor] {
        let other = random(scale, 7);
        println!("{:<11} {}", scale.name(), short(&wav(&other, 30.0, 1)?));
    }
    Ok(())
}
