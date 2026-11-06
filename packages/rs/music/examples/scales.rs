// The seven modes of the major scale, built up from C4.
// Each mode keeps the same seven steps but starts on a different one, which moves the half steps.
// cargo run -p mrly-music --example scales
use mrly_audio::name;
use mrly_music::{Scale, pool};

fn main() {
    println!("scale       steps                 notes from C4");
    for scale in Scale::all() {
        let steps: Vec<String> = scale.steps().iter().map(|s| format!("{s:>2}")).collect();
        let notes: Vec<String> = pool(scale, 60, 1).into_iter().map(name).collect();
        println!(
            "{:<11} {}  {}",
            scale.name(),
            steps.join(" "),
            notes.join(" ")
        );
    }

    // A pool is a scale stacked over octaves. The composer's home is G2, midi 43.
    let notes: Vec<String> = pool(Scale::Major, mrly_music::ROOT, 2)
        .into_iter()
        .map(name)
        .collect();
    println!("major from G2 over 2 octaves: {}", notes.join(" "));
}
