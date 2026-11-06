// Count what the carpet is made of at each level.
// Euler number = vertices - edges + filled cells. The carpet gains a hole at every level.
// cargo run -p mrly-two --example carpet_census

use mrly_two::{carpet, census};

fn main() {
    println!("level fills voids perimeter vertices edges euler");
    for level in 1..=3 {
        let c = census(&carpet(3, level).unwrap()).unwrap();
        println!(
            "{level:>5} {:>5} {:>5} {:>9} {:>8} {:>5} {:>5}",
            c.fills, c.voids, c.perimeter, c.vertices, c.edges, c.euler
        );
    }
}
