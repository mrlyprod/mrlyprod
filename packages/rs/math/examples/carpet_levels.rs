// A fractal repeats the tile inside its own filled cells.
// Level 1 is the 3x3 carpet tile, level 2 is 9x9, level 3 is 27x27: the Sierpinski carpet.
// cargo run -p mrly-math --example carpet_levels

use mrly_math::atoms::carpet_2d;
use mrly_math::cell::designs::grow;
use mrly_math::cell::models::Cell2d;

fn main() {
    for level in 1..=3 {
        let cell: Cell2d = grow(carpet_2d(3), level).unwrap();
        let n = cell.width();
        println!("level {level}, {n}x{n}");
        for y in 0..n {
            let row: String = (0..n)
                .map(|x| {
                    if cell.types().at(y * n + x) == 1 {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect();
            println!("{row}");
        }
        println!();
    }
}
