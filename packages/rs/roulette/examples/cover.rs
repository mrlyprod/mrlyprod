// Which part of the plane a spirograph's curves enclose. cover() draws the curves on a raster, then floods the empty space from the edge (outside) and from the centre (the hollow).
// What the floods cannot reach counts as covered. '#' is covered, '.' is the hollow in the middle, blank is outside the bounding disc.
// The fractions are shares of that disc.
// cargo run -p mrly-roulette --example cover
use mrly_roulette::{cover, pencils, track};

fn main() {
    let tile = [0, 1, 0, 1, 1, 1, 0, 1, 0];
    let list = pencils(&tile, 3, 3, "fill", 0.9, 0.0, 1).unwrap();
    let path = track("in", 8, 3, 0, 1).unwrap();
    let side = 40;
    let found = cover(&path, &list, false, 400, side).unwrap();
    println!(
        "covered {:.3}  hole {:.3}  wall {:.3}",
        found.covered, found.hole, found.wall
    );
    for row in found.mask.chunks(side) {
        let line: String = row
            .iter()
            .map(|&code| match code {
                1 | 0 => "  ",
                2 => "..",
                _ => "##",
            })
            .collect();
        println!("{}", line.trim_end());
    }
}
