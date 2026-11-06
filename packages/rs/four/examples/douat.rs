// Douat's concentric diamonds (1722), written in tokens and unfolded from one quarter.
// A pattern with two mirror axes needs only its first quarter: reflect the rest.
// cargo run -p mrly-four --example douat
use mrly_four::{Cell4d, Move, from_tokens, merge, svg};

const INK: [&str; 2] = ["#fff6e5", "#1f2a44"];

fn unfold(quarter: &Cell4d) -> Result<Cell4d, String> {
    let cells = [
        quarter.clone(),
        quarter.transform(Move::FlipX),
        quarter.transform(Move::FlipY),
        quarter.rotate(2),
    ];
    merge(&cells, 2, 2)
}

fn main() -> Result<(), String> {
    let douat = unfold(&from_tokens(
        6,
        6,
        "/01/10/01/10/01\\10/10\\01\\10\\01\\10/10\\01\\10\\01\\10\\01/01\\10\\01\\10\\01\\10/10/01\\10\\01\\10\\01/01\\01/10\\10\\01/01/10",
    )?)?;
    println!("{}", svg(&douat.tile(2, 2), 12, &INK)?);
    Ok(())
}
