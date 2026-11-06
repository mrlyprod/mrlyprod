// One bar of eight frames for each movement, played over a C major triad.
// The first frame draws from the chord; each movement then picks the next frame from the last.
// cargo run -p mrly-music --example movements
use mrly_audio::name;
use mrly_music::{ChordType, Movement, Piece, Voice, compose};

fn show(label: &str, frames: &[Vec<i64>]) {
    let cells: Vec<String> = frames
        .iter()
        .map(|notes| match notes.as_slice() {
            [] => "-".to_string(),
            _ => notes.iter().map(|&n| name(n)).collect::<Vec<_>>().join("+"),
        })
        .collect();
    println!("{label:<12} {}", cells.join(" "));
}

fn main() -> Result<(), String> {
    let pool = vec![60, 62, 64, 65, 67, 69, 71];
    for movement in Movement::all() {
        let mut voice = Voice::new(pool.clone(), vec![movement]);
        voice.chord = Some(ChordType::Triad);
        let mut piece = Piece::new("C", vec![voice]);
        piece.bar = 8;
        show(movement.name(), &compose(&piece, 8, 3)?[0]);
    }

    // A voice with several movements draws one at random for every frame.
    // After a pause it picks up again from the last notes it played.
    let mut voice = Voice::new(pool, vec![Movement::Up, Movement::Pause]);
    voice.counts = vec![2];
    let mut piece = Piece::new("C", vec![voice]);
    piece.bar = 8;
    show("up or pause", &compose(&piece, 8, 3)?[0]);
    Ok(())
}
