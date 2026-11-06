// Vertices, edges, faces and cubes of the sponge at each level, counted without building it.
// Euler = vertices - edges + faces - cubes. Levels 1 and 2 match the census in the three crate.
// cargo run -p mrly-sequences --example sponge_skeleton

use mrly_math::Code;
use mrly_sequences::counts::skeleton;

fn main() {
    println!("level vertices edges faces cubes euler");
    for level in 1..=4 {
        let s = skeleton(Code::from(23u64), 3, 3, 2, level).unwrap();
        println!(
            "{level:>5} {:>8} {:>5} {:>5} {:>5} {:>5}",
            s.vertices(),
            s.edges(),
            s.faces(),
            s.cells(),
            s.euler
        );
    }
}
