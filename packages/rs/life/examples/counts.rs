// Birth and survival counts do not have to be typed in. They can be drawn from a number sequence.
// Each row is a source, its OEIS id, and the counts it gives for a neighbourhood of 8 cells (0 and 1 left out).
// cargo run -p mrly-life --example counts
use mrly_life::{Counts, Source};

fn main() {
    for source in Source::all() {
        let counts = Counts::drawn(source, false, false).values(8).unwrap();
        let list: Vec<String> = counts.iter().map(|n| n.to_string()).collect();
        println!(
            "{:<13} {:<8} {}",
            source.name(),
            source.oeis().unwrap_or_default(),
            list.join(" ")
        );
    }
}
