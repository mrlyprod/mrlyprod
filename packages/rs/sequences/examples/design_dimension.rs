// Fractal dimension ln(fill) / ln(side) of a one-level tile.
// The 2D carpet is ln 8 / ln 3 = 1.8928, the 3D sponge ln 20 / ln 3 = 2.7268.
// cargo run -p mrly-sequences --example design_dimension

use mrly_math::Code;
use mrly_sequences::counts::dimension;

fn main() {
    println!(
        "carpet 2D  {:.4}",
        dimension(Code::from(7u64), 3, 2, 2).unwrap()
    );
    println!(
        "sponge 3D  {:.4}",
        dimension(Code::from(23u64), 3, 3, 2).unwrap()
    );
}
