// The core graph of the Sierpinski carpet at levels 1 to 3, with its census.
// Box counting gives a rough dimension that creeps toward ln 8 / ln 3 = 1.8928 as the level rises.
// cargo run -p mrly-graph --example carpet_census

use mrly_graph::{census, core_graph};
use mrly_tensor::Tensor;

fn filled(mut x: usize, mut y: usize) -> bool {
    while x > 0 || y > 0 {
        if x % 3 == 1 && y % 3 == 1 {
            return false;
        }
        x /= 3;
        y /= 3;
    }
    true
}

fn main() {
    println!("level nodes branches tips junctions parts dimension");
    for level in 1..=3u32 {
        let side = 3usize.pow(level);
        let data = (0..side * side)
            .map(|i| u8::from(filled(i % side, i / side)))
            .collect();
        let grid = Tensor::of(data, vec![side, side]).unwrap();
        let c = census(&core_graph(&grid).unwrap()).unwrap();
        println!(
            "{level:>5} {:>5} {:>8} {:>4} {:>9} {:>5} {:>9.4}",
            c.nodes, c.branches, c.tips, c.junctions, c.components, c.fractal_dimension
        );
    }
}
