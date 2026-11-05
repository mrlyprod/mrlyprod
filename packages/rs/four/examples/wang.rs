// Wang tiles are native: a Wang tile is four edge colours, and each triangle touches one edge.
// So [n, e, s, w] is the tile. Two colours give exactly the 16 tags; k colours give k^4 tiles.
// The matching rule: my E equals my east neighbour's W, and my S equals my south neighbour's N.
// When a pair matches, its two triangles form a diamond on the shared edge and the seam vanishes.
use mrly_four::{Cell4d, HALVES, from_tags, from_tokens, random, seams, svg, tag, text};
use std::collections::BTreeSet;
use std::fs;

const DATA_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../data/packages/rs/four/examples"
);
const PALETTE: [&str; 3] = ["#fff6e5", "#8fb8de", "#f2a7a0"];
const INK: [&str; 2] = ["#fff6e5", "#1f2a44"];
const SEED: u64 = 1961;

fn splitmix(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

// Lay tiles left to right, top to bottom: each cell must match what is above and to its left.
// This set holds the eight even tags, so every (north, west) pair has two tiles to choose from.
fn lay(width: usize, height: usize, tiles: &[u8], state: &mut u64) -> Result<Cell4d, String> {
    let mut picks: Vec<u8> = Vec::with_capacity(width * height);
    for y in 0..height {
        for x in 0..width {
            let fits: Vec<u8> = tiles
                .iter()
                .copied()
                .filter(|&tile| {
                    let [n, _, _, w] = tag::sides(tile);
                    let above = y == 0 || tag::sides(picks[(y - 1) * width + x])[2] == n;
                    let left = x == 0 || tag::sides(picks[y * width + x - 1])[1] == w;
                    above && left
                })
                .collect();
            picks.push(fits[(splitmix(state) % fits.len() as u64) as usize]);
        }
    }
    from_tags(width, height, &picks)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(DATA_DIR)?;
    let (width, height) = (32, 24);
    let mut state = SEED;
    let even: Vec<u8> = (0..16u8).filter(|t| t.count_ones() % 2 == 0).collect();
    let wang = lay(width, height, &even, &mut state)?;
    let inner = 2 * width * height - width - height;
    println!(
        "wang, 8 even tiles: {} seams of {inner}",
        seams(&wang, false)
    );

    let truchet = random(width, height, &HALVES, |n| {
        (splitmix(&mut state) % n as u64) as usize
    })?;
    println!(
        "random Truchet, same size: {} seams of {inner}",
        seams(&truchet, false)
    );
    fs::write(format!("{DATA_DIR}/wang.svg"), svg(&wang, 24, &INK)?)?;

    // A traditional quilt block that happens to be a real Wang tiling, in three colours.
    let quilt = from_tokens(3, 3, "(0012)(1010)(1200)\\12(1021)/02/20(2110)\\21")?;
    let tiles: BTreeSet<[u8; 4]> = (0..9).map(|i| quilt.at(i % 3, i / 3)).collect();
    println!(
        "quilt block: {} seams on the torus, {} distinct tiles, tags {:?}",
        seams(&quilt, true),
        tiles.len(),
        text(&quilt)
    );
    fs::write(
        format!("{DATA_DIR}/wang_quilt.svg"),
        svg(&quilt.tile(6, 6), 24, &PALETTE)?,
    )?;
    Ok(())
}
