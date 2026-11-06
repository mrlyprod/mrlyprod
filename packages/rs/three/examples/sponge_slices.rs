// Cut the level 2 sponge with flat planes along the first axis.
// Plane 0 is the full carpet. Planes 1, 3 and 4 cut through the holes.
// cargo run -p mrly-three --example sponge_slices

use mrly_three::{carpet, slice};

fn main() {
    let cell = carpet(3, 2).unwrap();
    for index in [0, 1, 3, 4] {
        let plane = slice(&cell, 0, index).unwrap();
        println!("plane {index}");
        let (h, w) = (plane.height(), plane.width());
        for y in 0..h {
            let row: String = (0..w)
                .map(|x| {
                    if plane.types().at(y * w + x) == 1 {
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
