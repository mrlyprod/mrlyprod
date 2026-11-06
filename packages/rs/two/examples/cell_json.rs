// A cell saved as JSON and loaded again. The loaded cell equals the original.
// cargo run -p mrly-two --example cell_json

use mrly_two::{carpet, from_json, to_json};

fn main() {
    let cell = carpet(3, 1).unwrap();
    let json = to_json(&cell);
    println!("{json}");
    println!("round trip equal: {}", from_json(&json).unwrap() == cell);
}
