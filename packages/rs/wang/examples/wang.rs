// A Wang tiling: 8 tiles laid so every shared edge has the same colour on both sides.
// Where a pair matches, its two triangles form a diamond and the seam vanishes.
// cargo run -p mrly-wang --example wang
use mrly_wang::{CellWang, from_tags, svg, tag};

const INK: [&str; 2] = ["#fff6e5", "#1f2a44"];
const SEED: u64 = 1961;

fn splitmix(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

// A Wang tile is four edge colours, and each triangle touches one edge. So [n, e, s, w] is the tile.
// Two colours give exactly the 16 tags; k colours give k^4 tiles.
// The matching rule: my E equals my east neighbour's W, and my S equals my south neighbour's N.
//
// Lay tiles left to right, top to bottom: each cell must match what is above and to its left.
// This set holds the eight even tags, so every (north, west) pair has two tiles to choose from.
fn lay(width: usize, height: usize, state: &mut u64) -> Result<CellWang, String> {
    let tiles: Vec<u8> = (0..16u8).filter(|t| t.count_ones() % 2 == 0).collect();
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

fn main() -> Result<(), String> {
    let mut state = SEED;
    let wang = lay(16, 12, &mut state)?;
    println!("{}", svg(&wang, 16, &INK)?);
    Ok(())
}
