// The neighbourhood is a mask: which cells around a cell count as neighbours. A design code draws it.
// Printed for four codes: the 3 by 3 mask, how many neighbours it has, and the lattice index
// (1 means the neighbours reach every cell; 2 means they only reach half of them, like a checkerboard).
// cargo run -p mrly-life --example masks
use mrly_life::{design_mask, lattice_index, mask_offsets};
use mrly_math::code::Code;

fn main() {
    for code in [7u128, 6, 3, 1] {
        let mask = design_mask(2, Code::from(code), 3, 1).unwrap();
        println!("code {code}");
        for row in 0..3 {
            let line: String = (0..3)
                .map(|col| {
                    if mask.at(row * 3 + col) == 1 {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect();
            println!("  {line}");
        }
        println!(
            "  neighbours {}, lattice index {}",
            mask_offsets(&mask).len(),
            lattice_index(&mask)
        );
    }
}
