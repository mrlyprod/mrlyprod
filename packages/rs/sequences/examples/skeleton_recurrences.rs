// Each count of the sponge (vertices, edges, faces, cubes) follows its own recurrence.
// Cubes are simplest: 20 times the last level. Vertices need four earlier terms.
// cargo run -p mrly-sequences --example skeleton_recurrences

use mrly_math::Code;
use mrly_sequences::counts::{Closed, skeleton_recurrence};

fn main() {
    let names = ["vertices", "edges", "faces", "cubes"];
    for (k, name) in names.iter().enumerate() {
        let rule = skeleton_recurrence(Code::from(23u64), 3, 3, 2, Some(k)).unwrap();
        println!("{name:<8} {}", Closed::Recurrence(rule).text());
    }
}
