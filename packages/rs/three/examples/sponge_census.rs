// Count the sponge at levels 1 to 3.
// Fills multiply by 20 each level (20, 400, 8000). Euler = vertices - edges + faces - fills.
// cargo run -p mrly-three --example sponge_census

use mrly_three::{carpet, census};

fn main() {
    println!("level fills voids surface vertices edges faces euler");
    for level in 1..=3 {
        let c = census(&carpet(3, level).unwrap()).unwrap();
        println!(
            "{level:>5} {:>5} {:>5} {:>7} {:>8} {:>5} {:>5} {:>5}",
            c.fills, c.voids, c.surface, c.vertices, c.edges, c.faces, c.euler
        );
    }
}
