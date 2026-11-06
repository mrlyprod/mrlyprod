// A quilt medallion in six tags, so one hex digit per cell is not enough.
// This is why a cell keeps one value per triangle and treats the digit as a view of it.
// cargo run -p mrly-wang --example quilt
use mrly_wang::{CellWang, Move, from_tokens, merge, svg};

const PALETTE: [&str; 6] = [
    "#fff6e5", "#8fb8de", "#f2a7a0", "#a8d8b9", "#c9b6e4", "#fbd786",
];

fn unfold(quarter: &CellWang) -> Result<CellWang, String> {
    let cells = [
        quarter.clone(),
        quarter.transform(Move::FlipX),
        quarter.transform(Move::FlipY),
        quarter.rotate(2),
    ];
    merge(&cells, 2, 2)
}

fn main() -> Result<(), String> {
    let quilt = unfold(&from_tokens(3, 3, "/01\\212\\343\\215\\34/10")?)?;
    println!("{}", svg(&quilt.tile(3, 3), 16, &PALETTE)?);
    Ok(())
}
