// A classic Truchet tiling (Sebastien Truchet, 1704).
// Every cell is one half-square tile, drawn at random from the four adjacent pairs.
// cargo run -p mrly-wang --example truchet
use mrly_wang::{HALVES, random, svg};

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

fn main() -> Result<(), String> {
    let mut state = SEED;
    let tiled = random(16, 12, &HALVES, |n| {
        (splitmix(&mut state) % n as u64) as usize
    })?;
    println!("{}", svg(&tiled, 16, &[PAPER, INK])?);
    Ok(())
}
