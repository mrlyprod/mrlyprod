// The carpet at level 2 as an SVG. Filled cells are indigo squares, empty cells are not drawn.
// paint() maps cell values to colors; value 0 maps to transparent so svg() skips it.
// cargo run -p mrly-two --example carpet_svg

use mrly_color::{ALPHA, INDIGO};
use mrly_two::{Shape, carpet, paint, svg};
use std::collections::HashMap;

fn main() {
    let colors = HashMap::from([(0u8, vec![ALPHA]), (1u8, vec![INDIGO])]);
    let cell = paint(carpet(3, 2).unwrap(), Some(&colors), None, None).unwrap();
    println!("{}", svg(&cell, 16, None, 0, Shape::Square));
}
