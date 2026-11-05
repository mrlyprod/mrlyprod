// A classic Truchet tiling (Sebastien Truchet, 1704).
// Every cell is one half-square tile, drawn at random from the four adjacent pairs.
use mrly_four::{Cell4d, E, HALVES, N, from_turns, random, seams, svg, tag};
use std::fs;

const DATA_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../data/packages/rs/four/examples"
);
const SEED: u64 = 1704;
const PAPER: &str = "#fff6e5";
const INK: &str = "#1f2a44";

// Any random stream will do: the crate only asks for a pick below n.
fn splitmix(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (width, height) = (32, 24);
    let mut state = SEED;
    let tiled: Cell4d = random(width, height, &HALVES, |n| {
        (splitmix(&mut state) % n as u64) as usize
    })?;

    // Truchet's own view: one motif, one rotation int per cell. Same tiling.
    let turns: Vec<u8> = (0..height)
        .flat_map(|y| (0..width).map(move |x| (x, y)))
        .map(|(x, y)| {
            let wanted = tiled.tag(x, y);
            (0..4u8)
                .find(|&k| tag::Move::Turn(usize::from(k)).tag(N | E) == wanted)
                .unwrap_or(0)
        })
        .collect();
    let turned = from_turns(width, height, tag::sides(N | E), &turns)?;
    println!(
        "one motif + a turn per cell == the tagged tiling: {}",
        turned == tiled
    );

    println!(
        "{width}x{height} cells, seams {} of {}",
        seams(&tiled, false),
        2 * width * height - width - height
    );
    fs::create_dir_all(DATA_DIR)?;
    fs::write(
        format!("{DATA_DIR}/truchet.svg"),
        svg(&tiled, 24, &[PAPER, INK])?,
    )?;
    Ok(())
}
