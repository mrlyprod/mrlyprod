// Patterns written in a small token grammar, one token per cell:
// `/ab` and `\ab` are half tiles, `(nesw)` is four triangles, a lone tag is a whole square.
use mrly_four::{Cell4d, Move, from_tokens, merge, svg, text};
use std::fs;

const DATA_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../data/packages/rs/four/examples"
);
const PALETTE: [&str; 6] = [
    "#fff6e5", "#8fb8de", "#f2a7a0", "#a8d8b9", "#c9b6e4", "#fbd786",
];
const INK: [&str; 2] = ["#fff6e5", "#1f2a44"];

// A pattern with two mirror axes needs only its first quarter: reflect the rest.
fn unfold(quarter: &Cell4d) -> Result<Cell4d, String> {
    let cells = [
        quarter.clone(),
        quarter.transform(Move::FlipX),
        quarter.transform(Move::FlipY),
        quarter.rotate(2),
    ];
    merge(&cells, 2, 2)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(DATA_DIR)?;

    // Douat's concentric diamonds (1722), in two tags.
    let douat = unfold(&from_tokens(
        6,
        6,
        "/01/10/01/10/01\\10/10\\01\\10\\01\\10/10\\01\\10\\01\\10\\01/01\\10\\01\\10\\01\\10/10/01\\10\\01\\10\\01/01\\01/10\\10\\01/01/10",
    )?)?;
    println!("Douat hex rows:");
    for row in text(&douat) {
        println!("  {row}");
    }
    fs::write(
        format!("{DATA_DIR}/douat.svg"),
        svg(&douat.tile(3, 3), 16, &INK)?,
    )?;

    // Quilt medallions in six tags, so one hex digit per cell is not enough:
    // this is why a cell keeps one value per triangle and treats the digit as a view of it.
    let quilt = unfold(&from_tokens(3, 3, "/01\\212\\343\\215\\34/10")?)?;
    fs::write(
        format!("{DATA_DIR}/quilt.svg"),
        svg(&quilt.tile(4, 4), 24, &PALETTE)?,
    )?;
    Ok(())
}
