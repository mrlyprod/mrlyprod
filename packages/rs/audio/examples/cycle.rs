// One cycle of each timbre, drawn as text: a wavetable a synth can loop at any pitch.
// More partials sharpen a sine toward a square.
// cargo run -p mrly-audio --example cycle
use mrly_audio::{CYCLE, Timbre, Wave, cycle};

fn draw(label: &str, table: &[f32]) {
    println!("{label} ({} samples)", table.len());
    let (columns, rows) = (64, 9);
    for row in 0..rows {
        let level = 1.0 - 2.0 * row as f32 / (rows - 1) as f32;
        let line: String = (0..columns)
            .map(|column| {
                let s = table[column * table.len() / columns];
                if (s - level).abs() <= 1.0 / (rows - 1) as f32 {
                    '*'
                } else {
                    ' '
                }
            })
            .collect();
        println!("  |{line}|");
    }
}

fn main() {
    for harmonics in [1, 3, 16] {
        let timbre = Timbre::new(Wave::Square, harmonics);
        draw(
            &format!("square, harmonics {harmonics}"),
            &cycle(&timbre, 220.0),
        );
    }
    draw(
        "sawtooth, harmonics 16",
        &cycle(&Timbre::new(Wave::Sawtooth, 16), 220.0),
    );
    println!("every table holds {CYCLE} samples");
}
