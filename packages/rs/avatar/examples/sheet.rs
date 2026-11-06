// A sheet of twelve default profile pictures, one per name.
// Each picture is a 7 by 7 grid mirrored left to right, with one ink picked by the name's hash.
// cargo run -p mrly-avatar --example sheet
use mrly_avatar::svg;

const NAMES: [&str; 12] = [
    "ada",
    "alan",
    "emmy",
    "grace",
    "kurt",
    "maryam",
    "noether",
    "paul",
    "ramanujan",
    "sofia",
    "terence",
    "ursula",
];

const TILE: usize = 96;
const GAP: usize = 24;
const COLUMNS: usize = 4;

fn main() {
    let rows = NAMES.len().div_ceil(COLUMNS);
    let width = GAP + COLUMNS * (TILE + GAP);
    let height = GAP + rows * (TILE + GAP + 8);
    println!(
        "<svg width=\"{width}\" height=\"{height}\" viewBox=\"0 0 {width} {height}\" xmlns=\"http://www.w3.org/2000/svg\">"
    );
    println!("<rect width=\"{width}\" height=\"{height}\" fill=\"#0b0d10\"/>");
    for (i, name) in NAMES.iter().enumerate() {
        let x = GAP + (i % COLUMNS) * (TILE + GAP);
        let y = GAP + (i / COLUMNS) * (TILE + GAP + 8);
        let tile = svg(name, TILE).unwrap();
        // The picture is a complete svg document, so it nests as is once it has a position.
        println!(
            "{}",
            tile.trim()
                .replacen("<svg ", &format!("<svg x=\"{x}\" y=\"{y}\" "), 1)
        );
        println!(
            "<text x=\"{}\" y=\"{}\" fill=\"#8a919a\" font-family=\"sans-serif\" font-size=\"11\" text-anchor=\"middle\">{name}</text>",
            x + TILE / 2,
            y + TILE + 14
        );
    }
    println!("</svg>");
}
