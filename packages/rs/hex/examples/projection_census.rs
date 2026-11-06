// Count triangles in the pro and cut projections of the same sponge (level 1).
// euler = vertices - edges + fills + voids.
// cargo run -p mrly-hex --example projection_census

use mrly_hex::{census, cut, pro};
use mrly_three::carpet;

fn main() {
    let cube = carpet(3, 1).unwrap();
    println!("proj  fills voids vertices edges euler");
    for (name, flat) in [("pro", pro(&cube).unwrap()), ("cut", cut(&cube).unwrap())] {
        let c = census(&flat, false);
        println!(
            "{name:<5} {:>5} {:>5} {:>8} {:>5} {:>5}",
            c.fills, c.voids, c.vertices, c.edges, c.euler
        );
    }
}
