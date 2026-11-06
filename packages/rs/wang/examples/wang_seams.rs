// Count the seams of a Wang tiling against a random Truchet tiling of the same size.
// A seam is a shared edge whose two triangles disagree. Wang tiles have none; random half tiles have many.
// cargo run -p mrly-wang --example wang_seams
use mrly_wang::{CellWang, HALVES, from_tags, random, seams, tag};

const SEED: u64 = 1961;

fn splitmix(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

// Each cell must match the tile above and the tile to its left, drawn from the eight even tags.
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
    let (width, height) = (32, 24);
    let inner = 2 * width * height - width - height;
    let mut state = SEED;
    let wang = lay(width, height, &mut state)?;
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
    Ok(())
}
