// How many pen strokes does each glyph need? pen is the hand-drawn count, draft is what an automatic walk finds,
// and floor is the least any drawing could use. A pen above the floor wastes a lift; a draft above the pen is a worse guess.
// cargo run -p mrly-font --example strokes
use mrly_font::{draft, floor, glyph, strokes, trim};

fn main() {
    println!("glyph  pen  draft  floor");
    for c in ('A'..='Z').chain('0'..='9') {
        let rows = trim(&glyph(c).unwrap().rows);
        println!(
            "{c:>5}  {:>3}  {:>5}  {:>5}",
            strokes(c).len(),
            draft(&rows).len(),
            floor(&rows)
        );
    }
}
