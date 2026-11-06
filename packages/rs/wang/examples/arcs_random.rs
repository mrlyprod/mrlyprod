// Smith's arcs: random half-square tags drawn as quarter circles.
// Neighbouring arcs meet at an edge midpoint with one shared tangent, so they stitch into smooth strands.
// cargo run -p mrly-wang --example arcs_random
use mrly_wang::arcs::svg;
use mrly_wang::{HALVES, random};

const SEED: u64 = 1987;
const SCALE: usize = 32;
const STROKE: usize = 8;
const COLORS: [&str; 2] = ["#fff6e5", "#1f2a44"];

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
    let tangle = random(8, 8, &HALVES, |n| {
        (splitmix(&mut state) % n as u64) as usize
    })?;
    println!("{}", svg(&tangle, SCALE, STROKE, COLORS, false)?);
    Ok(())
}
