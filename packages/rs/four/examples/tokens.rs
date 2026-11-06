// Patterns written in a small token grammar, one token per cell, read back as hex rows.
// `/ab` and `\ab` are half tiles, `(nesw)` is four triangles, a lone tag is a whole square.
// cargo run -p mrly-four --example tokens
use mrly_four::{from_tokens, text};

fn main() -> Result<(), String> {
    // One cell per token: a whole 1, then /01, then \01, then (0112).
    let row = from_tokens(4, 1, "1/01\\01(0112)")?;
    println!("cells with the tags of their four triangles:");
    for y in 0..row.height {
        for x in 0..row.width {
            println!("  cell {x}: {:?}", row.at(x, y));
        }
    }
    // Hex keeps only filled or empty, so the 2 in (0112) prints like a 1: 0111 is 7.
    println!("as hex, one digit per cell: {}", text(&row).join(""));
    Ok(())
}
