// mrly-two grows a fractal by replacing each filled square with the whole pattern.
// Four does the same, and half tiles work too: a triangle keeps only the part of the
// pattern inside it. A block's diagonals run through sub-cell corners at every scale,
// so that clip always lands on triangle edges and the lattice is closed under it.
//
// The tile is carpet5 with its four corners cut by half tiles. A true octagon carpet:
// mrly-two has no way to draw it, since its edges are diagonal at every level.
use mrly_four::{census, from_strings, raster, svg};
use std::fs;

const DATA_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../data/packages/rs/four/examples"
);
const INK: [&str; 2] = ["#fff6e5", "#1f2a44"];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tile = from_strings(&["6fff3", "f0f0f", "fffff", "f0f0f", "cfff9"])?;
    let level = 3;
    let grown = tile.fractal(level)?;

    // The clip check: every pixel of a level is the AND of the level below and the tile.
    let scale = 2;
    let big = raster(&grown, scale);
    let outer = raster(&tile.fractal(level - 1)?, tile.width * scale);
    let repeats = grown.width / tile.width;
    let inner = raster(&tile.tile(repeats, repeats), scale);
    let wrong = (0..big.len())
        .filter(|&i| (big[i] != 0) != (outer[i] != 0 && inner[i] != 0))
        .count();
    println!(
        "{}x{} cells, {wrong} of {} pixels differ from outer AND inner",
        grown.width,
        grown.height,
        big.len()
    );
    let found = census(&grown);
    println!(
        "{} of {} triangles filled, perimeter {} straight + {} half-diagonals",
        found.fills,
        grown.types.len(),
        found.straight,
        found.diagonal
    );

    fs::create_dir_all(DATA_DIR)?;
    fs::write(format!("{DATA_DIR}/fractal.svg"), svg(&grown, 8, &INK)?)?;
    Ok(())
}
