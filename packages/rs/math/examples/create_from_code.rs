// create() builds a pattern from a code: side length, dimension, base, and fractal level.
// Code 7 is the carpet, 6 is the star, 11 has no name.
// The arguments are code, side length, dimension, base and fractal level.
// cargo run -p mrly-math --example create_from_code

use mrly_math::Code;
use mrly_math::factory::create;

fn main() {
    for code in [7u64, 6, 11] {
        let tensor = create(Code::from(code), 3, 2, 2, 2).unwrap();
        println!("code {code}");
        let n = tensor.shape[0];
        for y in 0..n {
            let row: String = (0..n)
                .map(|x| if tensor.at(y * n + x) == 1 { '#' } else { '.' })
                .collect();
            println!("{row}");
        }
        println!();
    }
}
