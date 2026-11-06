// Connected pieces and holes in the flattened sponge, level 2, pro and cut projections.
// The Euler number counts pieces minus holes, so holes = pieces - euler.
// cargo run -p mrly-hex --example sponge_holes

use mrly_hex::{components, cut, giant, holes, pro};
use mrly_three::carpet;

fn main() {
    let cube = carpet(3, 2).unwrap();
    println!("proj  pieces giant holes");
    for (name, flat) in [("pro", pro(&cube).unwrap()), ("cut", cut(&cube).unwrap())] {
        println!(
            "{name:<5} {:>6} {:>5} {:>5}",
            components(&flat).unwrap(),
            giant(&flat).unwrap(),
            holes(&flat).unwrap()
        );
    }
}
