// The whole font as one specimen: every supported glyph, 5 cells tall, drawn with # for lit cells.
// raster() lays text out in a grid of 0s and 1s, with one blank column between glyphs.
// cargo run -p mrly-font --example specimen
use mrly_font::{raster, supported};

fn main() {
    let all = supported();
    for line in all.chunks(14) {
        let text: String = line.iter().collect();
        for row in raster(&text) {
            let line: String = row
                .iter()
                .map(|&b| if b == 1 { '#' } else { '.' })
                .collect();
            println!("{line}");
        }
        println!();
    }
}
