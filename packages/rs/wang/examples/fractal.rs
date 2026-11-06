// An octagon carpet: carpet5 with its four corners cut by half tiles, grown to a fractal.
// mrly-two cannot draw it, since its edges are diagonal at every level.
// cargo run -p mrly-wang --example fractal
//
// mrly-two grows a fractal by replacing each filled square with the whole pattern.
// Wang does the same, and half tiles work too: a triangle keeps only the part of the
// pattern inside it. A block's diagonals run through sub-cell corners at every scale,
// so that clip always lands on triangle edges and the lattice is closed under it.
use mrly_wang::{from_strings, svg};

const INK: [&str; 2] = ["#fff6e5", "#1f2a44"];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tile = from_strings(&["6fff3", "f0f0f", "fffff", "f0f0f", "cfff9"])?;
    let level = 2;
    let grown = tile.fractal(level)?;
    println!("{}", svg(&grown, 10, &INK)?);
    Ok(())
}
