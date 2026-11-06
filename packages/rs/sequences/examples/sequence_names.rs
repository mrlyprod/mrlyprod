// A sequence is named by its design, measure and axis. Key gives the file name and a short id.
// The design here is the 3D carpet, code 23, base 2.
// cargo run -p mrly-sequences --example sequence_names

use mrly_sequences::{Axis, Key, Measure};

fn main() {
    for measure in [Measure::Fills, Measure::Surface, Measure::Euler] {
        for axis in [Axis::Level, Axis::Side] {
            let key = Key::new(23, 3, 2, measure, axis);
            println!("{} -> {}", key.name(), key.id());
        }
    }
}
