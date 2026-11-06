// Each classic design has a code, 0 to 15. Its bits switch on kinds of cell in a 3 by 3 block:
// 1 the corners, 2 the top and bottom middles, 4 the left and right middles, 8 the centre. Shown: the code, then the design.
// cargo run -p mrly-tile --example codes
use mrly_tile::classic_code;
use mrly_tile::recipe::Design;
use mrly_two::{named, text};
use std::collections::HashMap;

fn main() {
    let glyphs = HashMap::from([(0u8, ".".to_string()), (1u8, "#".to_string())]);
    let designs = [
        Design::Dust,
        Design::Htree,
        Design::Vtree,
        Design::Star,
        Design::Carpet,
        Design::Point,
        Design::Void,
        Design::Vline,
        Design::Hline,
        Design::Net,
    ];
    for design in designs {
        let picture = text(&named(design, 3, 1, 0).unwrap(), Some(&glyphs));
        println!(
            "{:<7} code {:>2}  {}",
            design.name(),
            classic_code(design).unwrap(),
            picture.join(" ")
        );
    }
}
