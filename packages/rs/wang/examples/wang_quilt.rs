// A traditional quilt block that happens to be a real Wang tiling, in three colours.
// Edge colours are 0, 1 and 2, each triangle takes the colour of its edge, and the block tiles the plane with no seam.
// cargo run -p mrly-wang --example wang_quilt
use mrly_wang::{from_tokens, svg};

const PALETTE: [&str; 3] = ["#fff6e5", "#8fb8de", "#f2a7a0"];

fn main() -> Result<(), String> {
    let quilt = from_tokens(3, 3, "(0012)(1010)(1200)\\12(1021)/02/20(2110)\\21")?;
    println!("{}", svg(&quilt.tile(4, 4), 16, &PALETTE)?);
    Ok(())
}
