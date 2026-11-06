// The deepest fractal level that stays within a cell budget of one million.
// A 2D tile of side n holds n^(2 * level) cells at that level.
// cargo run -p mrly-math --example level_cap

use mrly_math::factory::level_cap;

fn main() {
    for number in [2, 3, 4, 5, 7, 10] {
        println!(
            "side {number:>2}: level {}",
            level_cap(number, 2, 1_000_000)
        );
    }
}
