// The sponge has 20 copies of itself at one third the size, so its dimension is ln 20 / ln 3.
// That is between a surface (2) and a solid (3).
// cargo run -p mrly-three --example sponge_dimension

use mrly_three::sponge::dimension;

fn main() {
    println!("ln 20 / ln 3 = {:.6}", dimension());
}
