// One picture drawn large.
// The svg has no pixels of its own: size only sets the width and height attributes, so it stays sharp at any scale.
// cargo run -p mrly-avatar --example large
use mrly_avatar::svg;

fn main() {
    println!("{}", svg("mrly", 384).unwrap().trim());
}
