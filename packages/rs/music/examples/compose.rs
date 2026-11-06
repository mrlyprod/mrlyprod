// A random piece from a seed: its progression, its voices, and the first bars it composes.
// The same seed always gives the same piece and the same notes.
// cargo run -p mrly-music --example compose
use mrly_audio::name;
use mrly_music::{Scale, compose, frames, random};

fn names(notes: &[i64]) -> String {
    match notes {
        [] => "-".to_string(),
        _ => notes.iter().map(|&n| name(n)).collect::<Vec<_>>().join(" "),
    }
}

fn main() -> Result<(), String> {
    let piece = random(Scale::Major, 7);
    println!("progression {}", piece.progression);
    println!("bar {} frames, beat {} s", piece.bar, piece.beat);
    for (at, voice) in piece.voices.iter().enumerate() {
        let moves: Vec<&str> = voice.movements.iter().map(|m| m.name()).collect();
        println!(
            "voice {at}: {} notes {}..{}, chord {}, counts {:?}, moves {}",
            voice.pool.len(),
            name(voice.pool[0]),
            name(*voice.pool.last().unwrap()),
            voice.chord.map_or("none", |kind| kind.name()),
            voice.counts,
            moves.join(" "),
        );
    }

    // compose gives one part per voice; frames stacks the parts into what sounds at once.
    let parts = compose(&piece, 8, 1)?;
    println!(
        "frame  {}",
        (0..parts.len())
            .map(|at| format!("voice {at:<14}"))
            .collect::<String>()
    );
    for i in 0..8 {
        let cells: String = parts
            .iter()
            .map(|part| format!("{:<20}", names(&part[i])))
            .collect();
        println!("{i:>5}  {cells}");
    }
    println!("frame 0 sounds {}", names(&frames(&parts)[0]));
    println!(
        "seed 1 again gives the same parts: {}",
        compose(&piece, 8, 1)? == parts
    );
    Ok(())
}
