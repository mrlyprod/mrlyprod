// The chords of the twelve bar blues, stacked from a pool of notes.
// A letter picks a degree of the pool: C is its first note, D its second, and so on.
// A triad takes every other note from there: degrees 0, 2, 4. A seventh adds 6, a ninth adds 8.
// The pool starts on C3 here, so the letters match the note names.
// cargo run -p mrly-music --example chords
use mrly_audio::name;
use mrly_music::{BLUES, ChordType, ROOT, Scale, chords, pool};

fn main() -> Result<(), String> {
    let notes = pool(Scale::Major, 48, 2);
    println!("progression {BLUES}");
    for kind in ChordType::all() {
        println!("{}", kind.name());
        for (letter, chord) in BLUES.chars().zip(chords(BLUES, &notes, kind)?) {
            let names: Vec<String> = chord.into_iter().map(name).collect();
            println!("  {letter}  {}", names.join(" "));
        }
    }

    // The composer's pools start on G2, so the same letters land a fifth lower.
    // In a one-octave pool the B triad wraps around and reaches back to the bottom.
    let small = pool(Scale::Major, ROOT, 1);
    let names: Vec<String> = chords("B", &small, ChordType::Triad)?[0]
        .iter()
        .map(|&n| name(n))
        .collect();
    println!("B triad in one octave: {}", names.join(" "));
    println!("{}", chords("CH", &small, ChordType::Triad).unwrap_err());
    Ok(())
}
